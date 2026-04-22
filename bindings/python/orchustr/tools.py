from __future__ import annotations

from typing import Any

from .bridge import RustCrateBridge


class SearchTools:
    @staticmethod
    def search(provider: str, query: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-search",
            "search",
            {"provider": provider, "query": query, "config": config or {}},
        )


class WebTools:
    @staticmethod
    def fetch(provider: str, request: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-web",
            "fetch",
            {"provider": provider, "request": request, "config": config or {}},
        )

    @staticmethod
    def scrape(provider: str, url: str, config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-web",
            "scrape",
            {"provider": provider, "url": url, "config": config or {}},
        )


class VectorTools:
    @staticmethod
    def ensure_collection(provider: str, data: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-vector",
            "ensure_collection",
            {"provider": provider, "data": data, "config": config or {}},
        )

    @staticmethod
    def upsert(provider: str, data: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-vector",
            "upsert",
            {"provider": provider, "data": data, "config": config or {}},
        )

    @staticmethod
    def delete(provider: str, data: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-vector",
            "delete",
            {"provider": provider, "data": data, "config": config or {}},
        )

    @staticmethod
    def query(provider: str, data: dict[str, Any], config: dict[str, Any] | None = None) -> list[dict[str, Any]]:
        return RustCrateBridge.invoke(
            "or-tools-vector",
            "query",
            {"provider": provider, "data": data, "config": config or {}},
        )


class LoaderTools:
    @staticmethod
    def load(request: dict[str, Any]) -> list[dict[str, Any]]:
        return RustCrateBridge.invoke("or-tools-loaders", "load", {"request": request})


class ExecTools:
    @staticmethod
    def execute(
        request: dict[str, Any],
        providers: list[str] | None = None,
        config: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-exec",
            "execute",
            {
                "request": request,
                "providers": providers or ["python", "shell"],
                "config": config or {},
            },
        )


class FileTools:
    @staticmethod
    def read(path: str, provider: str = "local", config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-file",
            "read",
            {"provider": provider, "path": path, "config": config or {}},
        )

    @staticmethod
    def write(path: str, content: str, provider: str = "local", config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-file",
            "write",
            {"provider": provider, "path": path, "content": content, "config": config or {}},
        )

    @staticmethod
    def list(path: str, provider: str = "local", config: dict[str, Any] | None = None) -> list[dict[str, Any]]:
        return RustCrateBridge.invoke(
            "or-tools-file",
            "list",
            {"provider": provider, "path": path, "config": config or {}},
        )

    @staticmethod
    def delete(path: str, provider: str = "local", config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-file",
            "delete",
            {"provider": provider, "path": path, "config": config or {}},
        )

    @staticmethod
    def fetch(provider: str, query: dict[str, Any], config: dict[str, Any] | None = None) -> Any:
        return RustCrateBridge.invoke(
            "or-tools-file",
            "fetch",
            {"provider": provider, "query": query, "config": config or {}},
        )


class CommsTools:
    @staticmethod
    def send(
        provider: str,
        to: str,
        body: str,
        from_: str | None = None,
        config: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        payload = {"provider": provider, "to": to, "body": body, "config": config or {}}
        if from_ is not None:
            payload["from"] = from_
        return RustCrateBridge.invoke("or-tools-comms", "send", payload)


class ProductivityTools:
    @staticmethod
    def list_emails(provider: str, query: dict[str, Any] | None = None, config: dict[str, Any] | None = None) -> list[dict[str, Any]]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "list_emails",
            {"provider": provider, "query": query or {}, "config": config or {}},
        )

    @staticmethod
    def send_email(provider: str, item: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "send_email",
            {"provider": provider, "item": item, "config": config or {}},
        )

    @staticmethod
    def list_events(provider: str, query: dict[str, Any] | None = None, config: dict[str, Any] | None = None) -> list[dict[str, Any]]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "list_events",
            {"provider": provider, "query": query or {}, "config": config or {}},
        )

    @staticmethod
    def create_event(provider: str, item: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "create_event",
            {"provider": provider, "item": item, "config": config or {}},
        )

    @staticmethod
    def list_issues(provider: str, query: dict[str, Any] | None = None, config: dict[str, Any] | None = None) -> list[dict[str, Any]]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "list_issues",
            {"provider": provider, "query": query or {}, "config": config or {}},
        )

    @staticmethod
    def create_issue(provider: str, item: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "create_issue",
            {"provider": provider, "item": item, "config": config or {}},
        )

    @staticmethod
    def search_pages(provider: str, query: dict[str, Any] | None = None, config: dict[str, Any] | None = None) -> list[dict[str, Any]]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "search_pages",
            {"provider": provider, "query": query or {}, "config": config or {}},
        )

    @staticmethod
    def create_page(provider: str, item: dict[str, Any], config: dict[str, Any] | None = None) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "create_page",
            {"provider": provider, "item": item, "config": config or {}},
        )

    @staticmethod
    def post_message(
        provider: str,
        channel: str,
        text: str,
        config: dict[str, Any] | None = None,
    ) -> dict[str, Any]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "post_message",
            {"provider": provider, "channel": channel, "text": text, "config": config or {}},
        )

    @staticmethod
    def search_messages(provider: str, query: dict[str, Any] | None = None, config: dict[str, Any] | None = None) -> list[dict[str, Any]]:
        return RustCrateBridge.invoke(
            "or-tools-productivity",
            "search_messages",
            {"provider": provider, "query": query or {}, "config": config or {}},
        )
