from __future__ import annotations

import asyncio
import inspect

from .result import NodeResult
from .state import DynState


async def _maybe_await(value):
    if asyncio.iscoroutine(value) or inspect.isawaitable(value):
        return await value
    return value


def _coerce_state(state: dict | DynState) -> DynState:
    if isinstance(state, DynState):
        return state.copy()
    return DynState(state)


def _coerce_result(result, current: str, exits: set[str]) -> NodeResult:
    if isinstance(result, NodeResult):
        return result
    state = _coerce_state(result)
    if current in exits:
        return NodeResult.exit(state)
    return NodeResult.advance(state)


class ExecutionGraph:
    def __init__(self, nodes: dict, edges: dict, entry: str, exit_node: str) -> None:
        self._nodes = nodes
        self._edges = edges
        self._entry = entry
        self._exit = exit_node
        self._exits = {exit_node}

    async def invoke(self, state: dict | DynState) -> DynState:
        current = self._entry
        data = _coerce_state(state)
        for _ in range(1024):
            outcome = await _maybe_await(self._nodes[current](_coerce_state(data)))
            result = _coerce_result(outcome, current, self._exits)
            data = result.state.copy()
            if result.kind == "exit":
                return data
            if result.kind == "advance" and current in self._exits:
                return data
            if result.kind == "pause":
                raise RuntimeError(
                    f"graph paused at checkpoint {result.checkpoint_id or '<unknown>'}"
                )
            if result.kind == "branch":
                if not result.next:
                    raise ValueError(f"node {current} returned branch without a next node")
                current = result.next
                continue
            targets = self._edges.get(current, [])
            if len(targets) != 1:
                raise ValueError(f"node {current} requires exactly one default edge")
            current = targets[0]
        raise RuntimeError("graph exceeded execution limit")

    async def execute(self, state: dict | DynState) -> DynState:
        return await self.invoke(state)


class GraphBuilder:
    def __init__(self) -> None:
        self._nodes: dict[str, object] = {}
        self._edges: dict[str, list[str]] = {}
        self._entry: str | None = None
        self._exit: str | None = None

    def add_node(self, name: str, handler) -> "GraphBuilder":
        self._nodes[name] = handler
        return self

    def add_edge(self, source: str, target: str) -> "GraphBuilder":
        self._edges.setdefault(source, []).append(target)
        return self

    def set_entry(self, name: str) -> "GraphBuilder":
        self._entry = name
        return self

    def set_exit(self, name: str) -> "GraphBuilder":
        self._exit = name
        return self

    def build(self) -> ExecutionGraph:
        if not self._nodes or not self._entry or not self._exit:
            raise ValueError("graph requires nodes, entry, and exit")
        return ExecutionGraph(self._nodes, self._edges, self._entry, self._exit)
