from __future__ import annotations

import asyncio


class ForgeRegistry:
    def __init__(self) -> None:
        self._tools: dict[str, object] = {}

    def register(self, name: str, handler) -> None:
        self._tools[name] = handler

    async def import_from_mcp(self, client) -> int:
        for tool in await client.list_tools():
            async def proxy(args, tool_name=tool["name"]):
                return await client.invoke_tool(tool_name, args)
            self._tools[tool["name"]] = proxy
        return len(self._tools)

    async def invoke(self, name: str, args: dict) -> object:
        if name not in self._tools:
            raise KeyError(f"unknown tool: {name}")
        result = self._tools[name](args)
        if asyncio.iscoroutine(result):
            return await result
        return result
