"""
AsyncApiWrapper: clase base abstracta para clientes de API asíncronos.

Provee manejo de sesión HTTP (via httpx2.AsyncClient), conteo de requests,
y métodos de conveniencia (GET/POST/PATCH/DELETE).

Uso (siempre como context manager):

    class MyApiClient(AsyncApiWrapper):
        BASE_URL = "https://api.example.com"

        def __init__(self, api_key: str, timeout: float = 30.0, logger=None):
            super().__init__(api_key, timeout, logger)
            self.headers = {"authorization": api_key}

    async def main():
        async with MyApiClient(api_key="...") as client:
            response = await client.GET("/users", params={"limit": 10})
            data = response.json()

Notas para subclases:
    - BASE_URL es obligatorio: debe definirse como atributo de clase.
    - self.headers debe setearse en __init__ de la subclase (llamando a
      super().__init__() primero) antes de usar el cliente.
"""

import logging
from abc import ABC
from typing import Any, Optional

import httpx2

from .ensure_logger import ensure_logger


class AsyncApiWrapper(ABC):
    BASE_URL: str

    def __init__(
        self,
        api_key: str,
        timeout: float = 30.0,
        logger: Optional[logging.Logger] = None,
    ):
        self.timeout = timeout
        self.headers: dict
        self._client: Optional[httpx2.AsyncClient]
        self.logger = ensure_logger(logger)
        self.api_calls = 0
        # Implementation for child classes:
        # super().__init__(api_key, timeout, logger)
        # self.headers = {"authorization": api_key}

    async def __aenter__(self):
        self._client = httpx2.AsyncClient(
            headers=self.headers, timeout=self.timeout, base_url=self.BASE_URL
        )
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        if self._client:
            await self._client.aclose()
            self._client = None
            self.logger.info(f"Total API calls: {self.api_calls}")
            self.api_calls = 0

    async def _get_client(self) -> httpx2.AsyncClient:
        """Get or create HTTP client"""
        if self._client is None:
            self._client = httpx2.AsyncClient(
                headers=self.headers, timeout=self.timeout, base_url=self.BASE_URL
            )
        return self._client

    async def close(self):
        """Explicitly close the client"""
        if self._client:
            await self._client.aclose()
            self._client = None

    async def _request(
        self,
        method: str,
        endpoint: str,
        params: dict[str, Any] | None = None,
        json: dict[str, Any] | None = None,
    ) -> httpx2.Response:
        client = await self._get_client()
        response = await client.request(method, endpoint, params=params, json=json)
        self.api_calls += 1
        self.logger.debug(f"Api calls: {self.api_calls}")
        response.raise_for_status()
        return response

    async def GET(
        self, endpoint: str, params: dict[str, Any] | None = None
    ) -> httpx2.Response:
        return await self._request("GET", endpoint, params=params)

    async def POST(
        self, endpoint: str, json: dict[str, Any] | None = None
    ) -> httpx2.Response:
        return await self._request("POST", endpoint, json=json)

    async def PATCH(
        self, endpoint: str, json: dict[str, Any] | None = None
    ) -> httpx2.Response:
        return await self._request("PATCH", endpoint, json=json)

    async def DELETE(
        self, endpoint: str, params: dict[str, Any] | None = None
    ) -> httpx2.Response:
        return await self._request("DELETE", endpoint, params=params)
