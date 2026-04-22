from __future__ import annotations

import asyncio
import json
import os

try:
    import aiohttp

    _HAS_AIOHTTP = True
except ImportError:  # pragma: no cover - fall back to urllib for zero-dep envs
    _HAS_AIOHTTP = False


class ConduitProvider:
    """Base Python conduit interface mirroring the Rust provider contract."""

    async def complete_text(self, prompt: str):
        raise NotImplementedError

    async def complete_messages(self, messages: list[dict]):
        raise NotImplementedError

    async def stream_text(self, prompt: str):
        response = await self.complete_text(prompt)
        yield response.text


class _HttpConduit(ConduitProvider):
    def __init__(self, api_key: str, model: str, endpoint: str, headers: dict) -> None:
        self._api_key = api_key
        self._model = model
        self._endpoint = endpoint
        self._headers = headers

    async def complete_text(self, prompt: str):
        return await self.complete_messages([{"role": "user", "content": [{"type": "text", "text": prompt}]}])

    async def stream_text(self, prompt: str):
        """Non-streaming fallback — yields the full response as one chunk.

        Override in a subclass for true SSE streaming.
        """
        response = await self.complete_text(prompt)
        yield response.text


class OpenAiConduit(_HttpConduit):
    @classmethod
    def from_env(cls) -> "OpenAiConduit":
        return cls(
            os.environ["OPENAI_API_KEY"],
            os.environ["OPENAI_MODEL"],
            # Uses the OpenAI Responses API (not Chat Completions).
            # Schema: input=[...], response has output=[{content:[{text:...}]}]
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


_OPENAI_COMPAT_ENDPOINTS = {
    "openai": "https://api.openai.com/v1/chat/completions",
    "openrouter": "https://openrouter.ai/api/v1/chat/completions",
    "together": "https://api.together.xyz/v1/chat/completions",
    "groq": "https://api.groq.com/openai/v1/chat/completions",
    "fireworks": "https://api.fireworks.ai/inference/v1/chat/completions",
    "deepseek": "https://api.deepseek.com/v1/chat/completions",
    "mistral": "https://api.mistral.ai/v1/chat/completions",
    "xai": "https://api.x.ai/v1/chat/completions",
    "nvidia": "https://integrate.api.nvidia.com/v1/chat/completions",
    "ollama": "http://localhost:11434/v1/chat/completions",
}


class OpenAiCompatConduit(_HttpConduit):
    """Generic OpenAI-compatible conduit for providers that speak the Chat Completions API.

    Use the factory classmethods (openrouter, groq, together, fireworks, deepseek, mistral,
    xai, nvidia, ollama) or pass a custom endpoint directly.
    """

    def __init__(self, api_key: str, model: str, endpoint: str) -> None:
        super().__init__(
            api_key,
            model,
            endpoint,
            {"Authorization": f"Bearer {api_key}"},
        )

    @classmethod
    def openrouter(cls, api_key: str, model: str) -> "OpenAiCompatConduit":
        return cls(api_key, model, _OPENAI_COMPAT_ENDPOINTS["openrouter"])

    @classmethod
    def groq(cls, api_key: str, model: str) -> "OpenAiCompatConduit":
        return cls(api_key, model, _OPENAI_COMPAT_ENDPOINTS["groq"])

    @classmethod
    def together(cls, api_key: str, model: str) -> "OpenAiCompatConduit":
        return cls(api_key, model, _OPENAI_COMPAT_ENDPOINTS["together"])

    @classmethod
    def fireworks(cls, api_key: str, model: str) -> "OpenAiCompatConduit":
        return cls(api_key, model, _OPENAI_COMPAT_ENDPOINTS["fireworks"])

    @classmethod
    def deepseek(cls, api_key: str, model: str) -> "OpenAiCompatConduit":
        return cls(api_key, model, _OPENAI_COMPAT_ENDPOINTS["deepseek"])

    @classmethod
    def mistral(cls, api_key: str, model: str) -> "OpenAiCompatConduit":
        return cls(api_key, model, _OPENAI_COMPAT_ENDPOINTS["mistral"])

    @classmethod
    def xai(cls, api_key: str, model: str) -> "OpenAiCompatConduit":
        return cls(api_key, model, _OPENAI_COMPAT_ENDPOINTS["xai"])

    @classmethod
    def nvidia(cls, api_key: str, model: str) -> "OpenAiCompatConduit":
        return cls(api_key, model, _OPENAI_COMPAT_ENDPOINTS["nvidia"])

    @classmethod
    def ollama(cls, model: str, endpoint: str | None = None) -> "OpenAiCompatConduit":
        return cls("", model, endpoint or _OPENAI_COMPAT_ENDPOINTS["ollama"])

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


def _extract_text(body: dict) -> str:
    """Extract text from OpenAI Responses API, Chat Completions API, or Anthropic Messages API."""
    # Chat Completions: {"choices": [{"message": {"content": "..."}}]}
    choices = body.get("choices")
    if isinstance(choices, list) and choices:
        msg = choices[0].get("message", {}) if isinstance(choices[0], dict) else {}
        content = msg.get("content", "")
        if isinstance(content, str):
            return content
    # OpenAI Responses API: {"output": [{"content": [{"text": "..."}]}]}
    if "output" in body:
        return "".join(
            item.get("text", "")
            for block in body.get("output", [])
            for item in block.get("content", [])
            if isinstance(item, dict)
        )
    # Anthropic: {"content": [{"text": "..."}]}
    return "".join(
        item.get("text", "")
        for item in body.get("content", [])
        if isinstance(item, dict)
    )


async def _complete_http(endpoint: str, payload: dict, headers: dict) -> CompletionResponse:
    if _HAS_AIOHTTP:
        return await _complete_http_aiohttp(endpoint, payload, headers)
    return await _complete_http_urllib(endpoint, payload, headers)


async def _complete_http_aiohttp(endpoint: str, payload: dict, headers: dict) -> CompletionResponse:
    """True async HTTP using aiohttp — no thread-pool exhaustion under load."""
    async with aiohttp.ClientSession() as session:
        async with session.post(
            endpoint,
            json=payload,
            headers={"Content-Type": "application/json", **headers},
            timeout=aiohttp.ClientTimeout(total=30),
        ) as response:
            response.raise_for_status()
            body = await response.json()
    return CompletionResponse(_extract_text(body))


async def _complete_http_urllib(endpoint: str, payload: dict, headers: dict) -> CompletionResponse:
    """Fallback using urllib in a thread — used when aiohttp is not installed."""
    import urllib.request

    def _request() -> CompletionResponse:
        request = urllib.request.Request(
            endpoint,
            data=json.dumps(payload).encode("utf-8"),
            headers={"Content-Type": "application/json", **headers},
            method="POST",
        )
        with urllib.request.urlopen(request, timeout=30) as resp:
            body = json.loads(resp.read().decode("utf-8"))
        return CompletionResponse(_extract_text(body))

    return await asyncio.to_thread(_request)
