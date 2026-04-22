import asyncio

from orchustr import DynState, GraphBuilder, NodeResult, PromptBuilder


def test_graph_builder_two_node_pipeline():
    """Graph with two nodes executes in order and state is threaded through."""

    async def think_node(state: DynState) -> NodeResult:
        state["thought"] = "I should look this up"
        return NodeResult.advance(state)

    async def act_node(state: DynState) -> NodeResult:
        state["action"] = "search"
        return NodeResult.exit(state)

    graph = (
        GraphBuilder()
        .add_node("think", think_node)
        .add_node("act", act_node)
        .add_edge("think", "act")
        .set_entry("think")
        .set_exit("act")
        .build()
    )

    result = asyncio.run(graph.invoke(DynState({"query": "What is Orchustr?"})))

    assert isinstance(result, DynState)
    assert result["query"] == "What is Orchustr?"
    assert result["thought"] == "I should look this up"
    assert result["action"] == "search"


def test_dynstate_insert_and_get():
    state = DynState({"query": "orchustr"})
    state["provider"] = "mock"

    assert state["query"] == "orchustr"
    assert state.get("provider") == "mock"
    assert state.to_dict() == {"query": "orchustr", "provider": "mock"}


def test_prompt_builder_renders_template():
    template = PromptBuilder().template("Hello {{name}}").build()

    assert template.render({"name": "Orchustr"}) == "Hello Orchustr"


def test_node_result_advance_vs_exit():
    state = DynState({"step": 1})

    advanced = NodeResult.advance(state)
    exited = NodeResult.exit(state)

    assert advanced.kind == "advance"
    assert exited.kind == "exit"
    assert advanced.state["step"] == 1
    assert exited.state["step"] == 1
