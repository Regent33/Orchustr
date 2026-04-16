# Python Examples

## Render a Prompt

```python
from orchustr import PromptBuilder

template = PromptBuilder().template("Summarize {{topic}} for {{audience}}.").build()
print(template.render({"topic": "retrieval", "audience": "operators"}))
```

## Build a Small Graph

```python
import asyncio
from orchustr import GraphBuilder

async def add_text(state: dict) -> dict:
    return {**state, "text": "hello"}

async def summarize(state: dict) -> dict:
    return {**state, "summary": state["text"].upper()}

graph = (
    GraphBuilder()
    .add_node("add_text", add_text)
    .add_node("summarize", summarize)
    .add_edge("add_text", "summarize")
    .set_entry("add_text")
    .set_exit("summarize")
    .build()
)

async def main():
    result = await graph.execute({})
    print(result["summary"])

asyncio.run(main())
```

⚠️ Known Gaps & Limitations
- These examples use the Python package surface rather than direct Rust bindings to every crate.
