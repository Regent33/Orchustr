from __future__ import annotations


class ExecutionGraph:
    def __init__(self, nodes: dict, edges: dict, entry: str, exit_node: str) -> None:
        self._nodes = nodes
        self._edges = edges
        self._entry = entry
        self._exit = exit_node

    async def execute(self, state: dict) -> dict:
        current = self._entry
        data = dict(state)
        for _ in range(1024):
            next_state = await self._nodes[current](dict(data))
            data = dict(next_state)
            if current == self._exit:
                return data
            targets = self._edges.get(current, [])
            if len(targets) != 1:
                raise ValueError(f"node {current} requires exactly one default edge")
            current = targets[0]
        raise RuntimeError("graph exceeded execution limit")


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
