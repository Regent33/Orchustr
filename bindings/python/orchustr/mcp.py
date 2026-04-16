from __future__ import annotations

import asyncio
import json
import urllib.request


class NexusClient:
    def __init__(self, endpoint: str) -> None:
        self._endpoint = endpoint
        self._next_id = 1

    @classmethod
    async def connect_http(cls, endpoint: str) -> "NexusClient":
        return cls(endpoint)

    async def send(self, method: str, params: dict) -> dict:
        payload = json.dumps(
            {"jsonrpc": "2.0", "id": self._next_id, "method": method, "params": params}
        ).encode("utf-8")
        self._next_id += 1

        def _request() -> dict:
            request = urllib.request.Request(
                self._endpoint,
                data=payload,
                headers={"Content-Type": "application/json"},
                method="POST",
            )
            with urllib.request.urlopen(request, timeout=30) as response:
                body = json.loads(response.read().decode("utf-8"))
                return body["result"]

        return await asyncio.to_thread(_request)

    async def list_tools(self) -> list[dict]:
        return (await self.send("tools/list", {})).get("tools", [])

    async def invoke_tool(self, name: str, args: dict) -> dict:
        return await self.send("tools/call", {"name": name, "arguments": args})
