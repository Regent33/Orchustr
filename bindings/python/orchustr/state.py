from __future__ import annotations

from typing import Any, Mapping


class DynState(dict[str, Any]):
    """Mutable graph state threaded through Orchustr Python workflows."""

    def __init__(self, initial: Mapping[str, Any] | None = None, **kwargs: Any) -> None:
        super().__init__()
        if initial is not None:
            self.update(dict(initial))
        if kwargs:
            self.update(kwargs)

    def copy(self) -> "DynState":
        return DynState(self)

    def to_dict(self) -> dict[str, Any]:
        return dict(self)
