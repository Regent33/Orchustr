from __future__ import annotations

import json
from dataclasses import dataclass
from typing import Any

from ._runtime import invoke_crate_native, workspace_catalog_native


@dataclass(frozen=True)
class CrateBinding:
    crate_name: str
    binding_mode: str
    description: str
    operations: list[str]


class RustCrateBridge:
    @staticmethod
    def available() -> bool:
        return workspace_catalog_native() is not None

    @staticmethod
    def catalog() -> list[CrateBinding]:
        raw = workspace_catalog_native()
        if raw is None:
            return []
        items = json.loads(raw)
        return [
            CrateBinding(
                crate_name=item["crate_name"],
                binding_mode=item["binding_mode"],
                description=item["description"],
                operations=list(item["operations"]),
            )
            for item in items
        ]

    @staticmethod
    def invoke(crate_name: str, operation: str, payload: dict[str, Any] | None = None) -> Any:
        raw = invoke_crate_native(crate_name, operation, json.dumps(payload or {}))
        if raw is None:
            raise RuntimeError("native bridge is not available")
        return json.loads(raw)
