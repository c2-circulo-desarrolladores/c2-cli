r"""
Azure Blob Storage manager.

Combines authentication, downloading and uploading in a single class.

Usage
-----
>>> manager = AzureManager(download_path="./data", base_prefix="uploads/")
>>> manager.download_all("logs/", r".*\.ndjson$")
>>> manager.upload_file("report.csv")
"""

import os
import re

from azure.storage.blob import BlobServiceClient
from dotenv import load_dotenv

load_dotenv()


class AzureManager:
    """Single entry point for Azure Blob Storage operations."""

    def __init__(self, download_path: str | None = None, base_prefix: str = "") -> None:
        """
        Parameters
        ----------
        download_path : str, optional
            Local directory to save downloaded files. If None, files are
            returned as bytes in memory.
        base_prefix : str, default=""
            Base folder prefix for all uploads (e.g., "uploads/").
        """
        self.download_path = download_path
        self.base_prefix = base_prefix

        if self.download_path:
            os.makedirs(self.download_path, exist_ok=True)

        self.auth()

    def auth(self) -> "AzureManager":
        """Authenticate against Azure Blob Storage using environment variables."""
        account = os.getenv("AZURE_STORAGE_ACCOUNT")
        key = os.getenv("AZURE_STORAGE_KEY")
        self.container_name = os.getenv("CONTAINER_NAME", "")

        if not all([account, key, self.container_name]):
            raise ValueError(
                "Missing one of env vars: AZURE_STORAGE_ACCOUNT, AZURE_STORAGE_KEY, CONTAINER_NAME"
            )

        account_url = f"https://{account}.blob.core.windows.net"
        self._blob_service_client = BlobServiceClient(
            account_url=account_url, credential=key
        )
        self.container_client = self._blob_service_client.get_container_client(
            self.container_name
        )
        return self

    def _get_blob_path(self, blob_name: str) -> str:
        """Combine base prefix with blob name."""
        if self.base_prefix:
            return f"{self.base_prefix.rstrip('/')}/{blob_name.lstrip('/')}"
        return blob_name

    # ------------------------------------------------------------------
    # Download
    # ------------------------------------------------------------------

    def list_blobs(self, prefix: str = "", pattern: str | None = None) -> list[str]:
        """List blobs with an optional prefix and regex pattern."""
        blobs = [
            blob.name
            for blob in self.container_client.list_blobs(name_starts_with=prefix)
        ]

        if pattern:
            regex = re.compile(pattern)
            blobs = [blob for blob in blobs if regex.match(blob)]

        return blobs

    def download(self, blob_name: str) -> bytes | str:
        """
        Download a single blob.

        Returns bytes if no download_path was set, otherwise the local file path.
        """
        blob_client = self.container_client.get_blob_client(blob_name)
        data = blob_client.download_blob().readall()

        if self.download_path:
            local_path = os.path.join(self.download_path, blob_name)
            os.makedirs(os.path.dirname(local_path), exist_ok=True)

            with open(local_path, "wb") as f:
                f.write(data)

            return local_path

        return data

    def _download_list(self, blob_names: list[str]) -> dict[str, bytes | str]:
        """Download multiple blobs from a list of names."""
        return {blob_name: self.download(blob_name) for blob_name in blob_names}

    def download_all(
        self, prefix: str = "", pattern: str | None = None
    ) -> dict[str, bytes | str]:
        """List and download all blobs matching the criteria."""
        return self._download_list(self.list_blobs(prefix, pattern))

    def _download_matching(
        self, pattern: str, prefix: str = ""
    ) -> dict[str, bytes | str]:
        """Download blobs matching a regex pattern."""
        return self.download_all(prefix, pattern)

    def download_latest(
        self, prefix: str = "", pattern: str | None = None
    ) -> tuple[str, bytes | str]:
        """Download the most recent blob (sorted by name)."""
        blobs = self.list_blobs(prefix, pattern)

        if not blobs:
            raise FileNotFoundError(
                f"No blobs found with prefix='{prefix}' pattern='{pattern}'"
            )

        latest_blob = sorted(blobs)[-1]
        return latest_blob, self.download(latest_blob)

    def _download_range(
        self, prefix: str, start_date: str, end_date: str, extension: str = "ndjson"
    ) -> dict[str, bytes | str]:
        """Download blobs within a date range (YYYY-MM-DD in names)."""
        blobs = self.list_blobs(prefix, rf".*\.{extension}$")

        results = {}
        for blob_name in blobs:
            date_match = re.search(r"(\d{4}-\d{2}-\d{2})", blob_name)
            if date_match:
                blob_date = date_match.group(1)
                if start_date <= blob_date <= end_date:
                    results[blob_name] = self.download(blob_name)

        return results

    # ------------------------------------------------------------------
    # Upload / management
    # ------------------------------------------------------------------

    def upload(self, blob_name: str, data: bytes | str, overwrite: bool = True) -> str:
        """Upload data to a blob. Strings are encoded as UTF-8."""
        if isinstance(data, str):
            data = data.encode("utf-8")

        full_path = self._get_blob_path(blob_name)
        blob_client = self.container_client.get_blob_client(full_path)
        blob_client.upload_blob(data, overwrite=overwrite)

        return full_path

    def upload_file(
        self, local_path: str, blob_name: str | None = None, overwrite: bool = True
    ) -> str:
        """Upload a local file to blob storage."""
        if not os.path.exists(local_path):
            raise FileNotFoundError(f"Local file not found: {local_path}")

        if blob_name is None:
            blob_name = os.path.basename(local_path)

        with open(local_path, "rb") as f:
            data = f.read()

        return self.upload(blob_name, data, overwrite=overwrite)

    def _upload_files(
        self, local_paths: list[str], blob_prefix: str = "", overwrite: bool = True
    ) -> dict[str, str]:
        """Upload multiple local files."""
        results = {}
        for local_path in local_paths:
            filename = os.path.basename(local_path)
            blob_name = (
                f"{blob_prefix.rstrip('/')}/{filename}" if blob_prefix else filename
            )
            results[local_path] = self.upload_file(
                local_path, blob_name, overwrite=overwrite
            )

        return results

    def delete(self, blob_name: str) -> str:
        """Delete a blob and return its full path."""
        full_path = self._get_blob_path(blob_name)
        blob_client = self.container_client.get_blob_client(full_path)
        blob_client.delete_blob()

        return full_path

    def _delete_multiple(self, blob_names: list[str]) -> list[str]:
        """Delete multiple blobs."""
        return [self.delete(blob_name) for blob_name in blob_names]

    def exists(self, blob_name: str) -> bool:
        """Check if a blob exists."""
        full_path = self._get_blob_path(blob_name)
        blob_client = self.container_client.get_blob_client(full_path)
        return blob_client.exists()

    def _append_to_blob(self, blob_name: str, data: bytes | str) -> str:
        """Append data to a blob (download, append, re-upload)."""
        if isinstance(data, str):
            data = data.encode("utf-8")

        full_path = self._get_blob_path(blob_name)
        blob_client = self.container_client.get_blob_client(full_path)

        try:
            existing_data = blob_client.download_blob().readall()
        except Exception:
            existing_data = b""

        blob_client.upload_blob(existing_data + data, overwrite=True)

        return full_path

    def _copy_blob(self, source_blob: str, dest_blob: str) -> str:
        """Copy a blob to a new location within the same container."""
        source_path = self._get_blob_path(source_blob)
        dest_path = self._get_blob_path(dest_blob)

        source_client = self.container_client.get_blob_client(source_path)
        dest_client = self.container_client.get_blob_client(dest_path)

        dest_client.start_copy_from_url(source_client.url)

        return dest_path
