# Building Your First Agent in Python

The Python package does not expose a full Rust-backed agent runtime yet, so the most practical beginner path is to start with the graph and prompt primitives that the package already implements.

## Minimal Example

```python
import asyncio
from orchustr import GraphBuilder, PromptBuilder

prompt = PromptBuilder().template("Answer briefly about {{topic}}.").build()

async def build_prompt(state: dict) -> dict:
    return {**state, "prompt": prompt.render(state)}

graph = (
    GraphBuilder()
    .add_node("build_prompt", build_prompt)
    .set_entry("build_prompt")
    .set_exit("build_prompt")
    .build()
)

async def main():
    result = await graph.execute({"topic": "tool routing"})
    print(result["prompt"])

asyncio.run(main())
```

## Next Step

Once you need provider-backed completions, use `OpenAiConduit` or `AnthropicConduit` from the Python package and keep state in ordinary dictionaries.

⚠️ Known Gaps & Limitations
- There is no Python exposure of the Rust `or-sentinel` agent runtime in the current repository.
- Higher-level agent behavior in Python must be composed from the package primitives.
