from __future__ import annotations

from dataclasses import dataclass

from .state import DynState


@dataclass(frozen=True)
class NodeResult:
    """Control result returned by a Python graph node."""

    state: DynState
    kind: str
    next: str | None = None
    checkpoint_id: str | None = None

    @staticmethod
    def advance(state: DynState) -> "NodeResult":
        return NodeResult(state=DynState(state), kind="advance")

    @staticmethod
    def exit(state: DynState) -> "NodeResult":
        return NodeResult(state=DynState(state), kind="exit")

    @staticmethod
    def branch(state: DynState, next_node: str) -> "NodeResult":
        return NodeResult(state=DynState(state), kind="branch", next=next_node)

    @staticmethod
    def pause(checkpoint_id: str, state: DynState) -> "NodeResult":
        return NodeResult(
            state=DynState(state),
            kind="pause",
            checkpoint_id=checkpoint_id,
        )
