import asyncio

from orchustr import (
    CoreOrchestrator,
    GraphBuilder,
    PipelineBuilder,
    PromptBuilder,
    RustCrateBridge,
    TokenBudget,
)


def test_prompt_builder_renders_variables():
    template = PromptBuilder().template("Hello {{name}}").build()
    assert template.render({"name": "Ralph"}) == "Hello Ralph"


def test_prompt_builder_sanitizes_control_characters():
    template = PromptBuilder().template("Hello {{name}}").build()
    assert template.render({"name": "Ra\al\0ph"}) == "Hello Ralph"


def test_graph_builder_executes_async_nodes():
    async def run():
        async def start(state):
            return {**state, "text": "hello"}

        async def finish(state):
            return {**state, "done": state["text"].upper()}

        graph = (
            GraphBuilder()
            .add_node("start", start)
            .add_node("finish", finish)
            .add_edge("start", "finish")
            .set_entry("start")
            .set_exit("finish")
            .build()
        )
        return await graph.execute({})

    assert asyncio.run(run())["done"] == "HELLO"


def test_core_orchestrator_enforces_budget():
    CoreOrchestrator().enforce_completion_budget(
        TokenBudget(max_context_tokens=100, max_completion_tokens=20),
        70,
    )


def test_pipeline_builder_executes_sequential_nodes():
    async def run():
        pipeline = (
            PipelineBuilder()
            .add_node("one", lambda state: {**state, "a": 1})
            .add_node("two", lambda state: {**state, "b": state["a"] + 1})
            .build()
        )
        return await pipeline.execute({})

    assert asyncio.run(run())["b"] == 2


def test_rust_crate_bridge_catalog_is_optional():
    catalog = RustCrateBridge.catalog()
    assert isinstance(catalog, list)
