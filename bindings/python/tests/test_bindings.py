import asyncio

from orchustr import GraphBuilder, PromptBuilder


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
