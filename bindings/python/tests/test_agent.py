"""
Agent test for the Orchustr Python bindings.

Simulates a ReAct-style agent loop using:
- PromptBuilder (Bug 8 fix: template not sanitized)
- GraphBuilder  (graph execution)
- ForgeRegistry (Bug 7 fix: closure capture)

Tests are self-contained — no external dependencies needed.
"""
from __future__ import annotations

import asyncio
import os
import sys

# Ensure the package is importable from the repo root
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from orchustr import ForgeRegistry, GraphBuilder, PromptBuilder


async def test_prompt_builder_preserves_template_chars():
    """Bug 8: template string should NOT be sanitized."""
    # A template with a tab character should survive
    template = PromptBuilder().template("Hello\t{{name}}").build()
    rendered = template.render({"name": "Agent"})
    assert rendered == "Hello\tAgent", f"template tab stripped: {rendered!r}"
    # But injected values should still be sanitized
    rendered2 = template.render({"name": "Ra\x07lph"})
    assert rendered2 == "Hello\tRalph", f"value not sanitized: {rendered2!r}"
    print("PASS prompt_builder_preserves_template_chars")


async def test_forge_closure_captures_client():
    """Bug 7: closure should capture client reference explicitly."""

    class MockMcpClient:
        def __init__(self):
            self.call_log = []

        async def list_tools(self):
            return [{"name": "search"}, {"name": "calculate"}]

        async def invoke_tool(self, name, args):
            self.call_log.append((name, args))
            return {"result": f"{name}({args})"}

    client = MockMcpClient()
    forge = ForgeRegistry()
    imported = await forge.import_from_mcp(client)
    assert imported == 2, f"expected 2 tools, got {imported}"

    # Invoke both tools
    r1 = await forge.invoke("search", {"query": "weather"})
    r2 = await forge.invoke("calculate", {"expr": "2+2"})

    assert r1["result"] == "search({'query': 'weather'})", f"unexpected: {r1}"
    assert r2["result"] == "calculate({'expr': '2+2'})", f"unexpected: {r2}"
    assert len(client.call_log) == 2
    print("PASS forge_closure_captures_client")


async def test_react_agent_loop():
    """Full ReAct agent loop using pipeline-style graph (linear chain per run)."""

    # Register tools
    forge = ForgeRegistry()
    forge.register("calculate", lambda args: {"answer": eval(args["expr"])})

    # Build a linear graph: plan → act → observe → finish
    graph = (
        GraphBuilder()
        .add_node("plan", _plan_node)
        .add_node("act", lambda state: _act_node(state, forge))
        .add_node("observe", _observe_node)
        .add_node("finish", _identity_node)
        .add_edge("plan", "act")
        .add_edge("act", "observe")
        .add_edge("observe", "finish")
        .set_entry("plan")
        .set_exit("finish")
        .build()
    )

    # Simulate the agent loop manually: re-execute the graph until done
    state = {"task": "What is 2+2?", "iteration": 0}
    for _ in range(10):
        state = await graph.execute(state)
        if state.get("action") == "answer":
            break

    assert "final_answer" in state, f"agent should produce final_answer: {state}"
    assert state["iteration"] == 2, f"expected 2 iterations: {state['iteration']}"
    assert "4" in str(state["final_answer"]), f"answer should contain 4: {state}"
    # Verify state accumulation
    assert state["task"] == "What is 2+2?", "task should survive through iterations"
    print("PASS react_agent_loop")


async def test_pipeline_agent():
    """Multi-step pipeline that enriches state through a chain."""
    graph = (
        GraphBuilder()
        .add_node("classify", _classify)
        .add_node("fetch", _fetch)
        .add_node("generate", _generate)
        .add_edge("classify", "fetch")
        .add_edge("fetch", "generate")
        .set_entry("classify")
        .set_exit("generate")
        .build()
    )

    result = await graph.execute({"input": "What's the weather?"})

    assert result["intent"] == "weather", f"intent: {result['intent']}"
    assert "weather" in result["context"], f"context: {result['context']}"
    assert "weather" in result["response"], f"response: {result['response']}"
    assert result["input"] == "What's the weather?", "input should survive"
    print("PASS pipeline_agent")


# ── Node handlers ────────────────────────────────────────────────────

async def _plan_node(state):
    iteration = state.get("iteration", 0)
    if iteration < 1:
        return {**state, "action": "use_tool", "tool_name": "calculate", "tool_args": {"expr": "2+2"}}
    return {**state, "action": "answer", "final_answer": f"The result is {state.get('tool_result', '?')}"}


async def _act_node(state, forge):
    if state.get("action") == "use_tool":
        tool_name = state["tool_name"]
        tool_args = state["tool_args"]
        result = await forge.invoke(tool_name, tool_args)
        return {**state, "tool_result": result}
    return state


async def _observe_node(state):
    iteration = state.get("iteration", 0) + 1
    return {**state, "iteration": iteration}


async def _identity_node(state):
    return state


async def _classify(state):
    text = state.get("input", "")
    return {**state, "intent": "weather" if "weather" in text else "general"}


async def _fetch(state):
    intent = state.get("intent", "general")
    return {**state, "context": f"fetched data for intent={intent}"}


async def _generate(state):
    context = state.get("context", "")
    return {**state, "response": f"Answer based on: {context}"}


# ── Main ─────────────────────────────────────────────────────────────

async def main():
    await test_prompt_builder_preserves_template_chars()
    await test_forge_closure_captures_client()
    await test_react_agent_loop()
    await test_pipeline_agent()
    print("\nAll Python agent tests passed!")


if __name__ == "__main__":
    try:
        asyncio.run(main())
    except Exception as e:
        print(f"FAIL: {e}", file=sys.stderr)
        sys.exit(1)
