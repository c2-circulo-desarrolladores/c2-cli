from abc import ABC
from typing import Any, Optional
from .utils import ensure_logger
import logging

import httpx


class ApiWrapper(ABC):
    BASE_URL: str

    def __init__(
        self,
        api_key: str,
        timeout: float = 30.0,
        logger: Optional[logging.Logger] = None,
    ):
        self.timeout = timeout
        self.headers: dict # <- Should include api key
        self._client: Optional[httpx.Client]
        self.logger = ensure_logger(self, logger)
        self.api_calls = 0

        # Implementation for child classes:
        # super().__init__(api_key, timeout, logger)
        # self.headers = {"authorization": api_key}

    def __enter__(self):
        self._client = httpx.Client(
            headers=self.headers, timeout=self.timeout, base_url=self.BASE_URL
        )
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self._client:
            self._client.close()
            self._client = None
            self.logger.info(f"Total API calls: {self.api_calls}")
            self.api_calls = 0

    def _get_client(self) -> httpx.Client:
        """Get or create HTTP client"""
        if self._client is None:
            self._client = httpx.Client(
                headers=self.headers, timeout=self.timeout, base_url=self.BASE_URL
            )
        return self._client

    def close(self):
        """Explicitly close the client"""
        if self._client:
            self._client.close()
            self._client = None

    def _api_request(
        self, endpoint: str, params: dict[str, Any] | None = None
    ) -> httpx.Response:
        """Make API request with error handling"""
        client = self._get_client()
        try:
            response = client.get(endpoint, params=params)
            self.api_calls += 1
            self.logger.debug(f"Api calls: {self.api_calls}")
            response.raise_for_status()
            return response
        except httpx.HTTPStatusError as e:
            raise Exception(f"HTTP {e.response.status_code}: {e.response.text}") from e
        except httpx.RequestError as e:
            raise Exception(f"Request failed: {str(e)}") from e
