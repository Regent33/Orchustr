from __future__ import annotations

import asyncio
import random
from dataclasses import dataclass, field
from typing import Any, Awaitable, Callable

from .bridge import RustCrateBridge


def _merge_state(left: dict[str, Any], right: dict[str, Any]) -> dict[str, Any]:
    return {**left, **right}


async def _maybe_await(value):
    if asyncio.iscoroutine(value) or isinstance(value, Awaitable):
        return await value
    return value


@dataclass
class TokenBudget:
    max_context_tokens: int
    max_completion_tokens: int

    def fits(self, prompt_tokens: int, completion_tokens: int) -> bool:
        return prompt_tokens + completion_tokens <= self.max_context_tokens


@dataclass
class RetryPolicy:
    max_attempts: int
    base_delay_ms: int
    max_delay_ms: int
    jitter: bool = True


class CoreOrchestrator:
    def enforce_completion_budget(self, budget: TokenBudget, prompt_tokens: int) -> None:
        if not budget.fits(prompt_tokens, budget.max_completion_tokens):
            requested = prompt_tokens + budget.max_completion_tokens
            raise ValueError(
                f"budget exceeded: requested={requested}, budget={budget.max_context_tokens}"
            )

    def next_retry_delay(self, policy: RetryPolicy, attempt: int) -> int:
        if attempt <= 0 or attempt > policy.max_attempts:
            raise ValueError(f"invalid retry attempt: {attempt}")
        base = min(policy.base_delay_ms * (2 ** (attempt - 1)), policy.max_delay_ms)
        if not policy.jitter or base == 0:
            return base
        return random.randint(0, base)


@dataclass
class PrismConfig:
    otlp_endpoint: str
    service_name: str = "orchustr-python"


def install_global_subscriber(otlp_endpoint: str) -> None:
    RustCrateBridge.invoke(
        "or-prism",
        "install_global_subscriber",
        {"otlp_endpoint": otlp_endpoint},
    )


@dataclass
class PlainText:
    text: str


class TextParser:
    def parse(self, raw: str) -> PlainText:
        text = raw.strip()
        if not text:
            raise ValueError("text must not be empty")
        return PlainText(text=text)


@dataclass
class CheckpointRecord:
    checkpoint_id: str
    resume_from: str
    state: dict[str, Any]


class CheckpointGate:
    def __init__(self) -> None:
        self._records: dict[str, CheckpointRecord] = {}

    async def pause(
        self, checkpoint_id: str, resume_from: str, state: dict[str, Any]
    ) -> None:
        self._records[checkpoint_id] = CheckpointRecord(
            checkpoint_id=checkpoint_id,
            resume_from=resume_from,
            state=dict(state),
        )

    async def resume(self, checkpoint_id: str) -> CheckpointRecord:
        if checkpoint_id not in self._records:
            raise KeyError(checkpoint_id)
        return self._records[checkpoint_id]


@dataclass
class RecallEntry:
    kind: str
    value: dict[str, Any]


class RecallStore:
    def __init__(self) -> None:
        self._entries: list[RecallEntry] = []

    async def store(self, entry: RecallEntry) -> None:
        self._entries.append(entry)

    async def list(self, kind: str) -> list[RecallEntry]:
        return [entry for entry in self._entries if entry.kind == kind]


class RecallOrchestrator:
    async def remember(self, store: RecallStore, entry: RecallEntry) -> None:
        await store.store(entry)

    async def recall(self, store: RecallStore, kind: str) -> list[RecallEntry]:
        return await store.list(kind)


@dataclass
class RouteSelection:
    route: str


class CompassRouterBuilder:
    def __init__(self) -> None:
        self._routes: list[tuple[str, Callable[[dict[str, Any]], bool]]] = []
        self._default: str | None = None

    def add_route(
        self, name: str, predicate: Callable[[dict[str, Any]], bool]
    ) -> "CompassRouterBuilder":
        self._routes.append((name, predicate))
        return self

    def set_default(self, route: str) -> "CompassRouterBuilder":
        self._default = route
        return self

    def build(self):
        routes = list(self._routes)
        default = self._default

        class _Router:
            def select(self, state: dict[str, Any]) -> RouteSelection:
                for name, predicate in routes:
                    if predicate(state):
                        return RouteSelection(route=name)
                if default is not None:
                    return RouteSelection(route=default)
                raise ValueError("no matching route")

        return _Router()


class PipelineBuilder:
    def __init__(self) -> None:
        self._nodes: list[tuple[str, Callable[[dict[str, Any]], Any]]] = []

    def add_node(
        self, name: str, handler: Callable[[dict[str, Any]], Any]
    ) -> "PipelineBuilder":
        self._nodes.append((name, handler))
        return self

    def build(self):
        nodes = list(self._nodes)
        if not nodes:
            raise ValueError("pipeline requires at least one node")

        class _Pipeline:
            async def execute(self, initial_state: dict[str, Any]) -> dict[str, Any]:
                state = dict(initial_state)
                for _, handler in nodes:
                    patch = dict(await _maybe_await(handler(dict(state))))
                    state = _merge_state(state, patch)
                return state

            async def invoke(self, initial_state: dict[str, Any]) -> dict[str, Any]:
                return await self.execute(initial_state)

        return _Pipeline()


class RelayBuilder:
    def __init__(self) -> None:
        self._branches: list[tuple[str, Callable[[dict[str, Any]], Any]]] = []

    def add_branch(
        self, name: str, handler: Callable[[dict[str, Any]], Any]
    ) -> "RelayBuilder":
        self._branches.append((name, handler))
        return self

    def build(self):
        branches = list(self._branches)
        if not branches:
            raise ValueError("relay requires at least one branch")

        @dataclass
        class _Plan:
            branches: list[tuple[str, Callable[[dict[str, Any]], Any]]]

        return _Plan(branches=branches)


class RelayExecutor:
    async def execute(self, plan, initial_state: dict[str, Any]) -> dict[str, Any]:
        async def _run(name: str, handler):
            patch = dict(await _maybe_await(handler(dict(initial_state))))
            return name, patch

        patches = await asyncio.gather(
            *[_run(name, handler) for name, handler in plan.branches]
        )
        state = dict(initial_state)
        for _, patch in sorted(patches, key=lambda item: item[0]):
            state = _merge_state(state, patch)
        return state


@dataclass
class ColonyMember:
    name: str
    role: str


@dataclass
class ColonyMessage:
    from_: str
    to: str
    content: str


@dataclass
class ColonyResult:
    summary: str
    state: dict[str, Any]
    transcript: list[ColonyMessage]


class ColonyBuilder:
    def __init__(self) -> None:
        self._orchestrator = ColonyOrchestrator()

    def add_member(
        self, name: str, role: str, agent: Callable[[dict[str, Any], list[ColonyMessage], ColonyMember], Any]
    ) -> "ColonyBuilder":
        self._orchestrator.add_member(name, role, agent)
        return self

    def build(self) -> ColonyOrchestrator:
        return self._orchestrator


class ColonyOrchestrator:
    def __init__(self) -> None:
        self._members: list[tuple[ColonyMember, Callable[..., Any]]] = []

    def add_member(
        self, name: str, role: str, agent: Callable[[dict[str, Any], list[ColonyMessage], ColonyMember], Any]
    ) -> "ColonyOrchestrator":
        self._members.append((ColonyMember(name=name, role=role), agent))
        return self

    async def coordinate(self, initial_state: dict[str, Any]) -> ColonyResult:
        if not self._members:
            raise ValueError("colony requires at least one member")
        state = dict(initial_state)
        transcript: list[ColonyMessage] = []
        for member, agent in self._members:
            response = await _maybe_await(agent(dict(state), list(transcript), member))
            if isinstance(response, ColonyMessage):
                message = response
            else:
                message = ColonyMessage(
                    from_=member.name,
                    to="all",
                    content=str(response),
                )
            transcript.append(message)
            state[member.name] = message.content
        summary = transcript[-1].content if transcript else ""
        return ColonyResult(summary=summary, state=state, transcript=transcript)


@dataclass
class SentinelConfig:
    max_steps: int = 8
    metadata: dict[str, Any] = field(default_factory=dict)


@dataclass
class StepOutcome:
    status: str
    state: dict[str, Any]
    message: str | None = None


class SentinelOrchestrator:
    async def run_agent(
        self,
        agent: Callable[[dict[str, Any], SentinelConfig], Any],
        initial_state: dict[str, Any],
        config: SentinelConfig,
    ) -> StepOutcome:
        result = await _maybe_await(agent(dict(initial_state), config))
        if isinstance(result, StepOutcome):
            return result
        if isinstance(result, dict):
            return StepOutcome(status="completed", state=result)
        return StepOutcome(status="completed", state=dict(initial_state), message=str(result))
