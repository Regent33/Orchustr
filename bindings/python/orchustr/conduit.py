from __future__ import annotations

import asyncio
import json
import os
import urllib.request


class _HttpConduit:
    def __init__(self, api_key: str, model: str, endpoint: str, headers: dict) -> None:
        self._api_key = api_key
        self._model = model
        self._endpoint = endpoint
        self._headers = headers

    async def complete_text(self, prompt: str):
        return await self.complete_messages([{"role": "user", "content": [{"type": "text", "text": prompt}]}])

    async def stream_text(self, prompt: str):
        response = await self.complete_text(prompt)
        for chunk in response.text.split():
            yield chunk


class OpenAiConduit(_HttpConduit):
    @classmethod
    def from_env(cls) -> "OpenAiConduit":
        return cls(
            os.environ["OPENAI_API_KEY"],
            os.environ["OPENAI_MODEL"],
            "https://api.openai.com/v1/responses",
            {"Authorization": f"Bearer {os.environ['OPENAI_API_KEY']}"},
        )

    async def complete_messages(self, messages: list[dict]):
        payload = {
            "model": self._model,
            "input": messages,
            "max_output_tokens": 1024,
        }
        return await _complete_http(self._endpoint, payload, self._headers)


class AnthropicConduit(_HttpConduit):
    @classmethod
    def from_env(cls) -> "AnthropicConduit":
        return cls(
            os.environ["ANTHROPIC_API_KEY"],
            os.environ["ANTHROPIC_MODEL"],
            "https://api.anthropic.com/v1/messages",
            {
                "x-api-key": os.environ["ANTHROPIC_API_KEY"],
                "anthropic-version": "2023-06-01",
            },
        )

    async def complete_messages(self, messages: list[dict]):
        payload = {
            "model": self._model,
            "messages": messages,
            "max_tokens": 1024,
        }
        return await _complete_http(self._endpoint, payload, self._headers)


class CompletionResponse:
    def __init__(self, text: str) -> None:
        self.text = text


async def _complete_http(endpoint: str, payload: dict, headers: dict) -> CompletionResponse:
    def _request() -> CompletionResponse:
        request = urllib.request.Request(
            endpoint,
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json", **headers},
            method="POST",
        )
        with urllib.request.urlopen(request, timeout=30) as response:
            body = json.loads(response.read().decode("utf-8"))
        if "output" in body:
            text = "".join(
                item.get("text", "")
                for block in body.get("output", [])
                for item in block.get("content", [])
                if isinstance(item, dict)
            )
        else:
            text = "".join(
                item.get("text", "")
                for item in body.get("content", [])
                if isinstance(item, dict)
            )
        return CompletionResponse(text)

    return await asyncio.to_thread(_request)
