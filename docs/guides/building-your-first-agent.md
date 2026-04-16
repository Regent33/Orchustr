# Building Your First Agent

This guide uses the current public Rust surface from `or-sentinel`, `or-conduit`, `or-forge`, and `or-core`.

## Minimal Shape

```rust
use or_conduit::{CompletionMessage, ContentPart, MessageRole, OpenAiCompatConduit};
use or_core::{DynState, RetryPolicy, TokenBudget};
use or_forge::ForgeRegistry;
use or_sentinel::{SentinelAgent, SentinelConfig, SentinelOrchestrator};
use or_sentinel::domain::contracts::SentinelAgentTrait;

# async fn example() -> anyhow::Result<()> {
// Option 1: OpenAI via environment variable
let provider = OpenAiCompatConduit::openai_from_env()?;

// Option 2: OpenAI with explicit key
// let provider = OpenAiCompatConduit::openai("sk-your-key", "gpt-4o")?;

// Option 3: OpenRouter (access 100+ models with one key)
// let provider = OpenAiCompatConduit::openrouter("or-your-key", "meta-llama/llama-3-70b")?;

// Option 4: Local Ollama (no API key needed)
// let provider = OpenAiCompatConduit::ollama("llama3")?;

let registry = ForgeRegistry::new();
let agent = SentinelAgent::new(provider, registry)?;

let messages = vec![CompletionMessage {
    role: MessageRole::User,
    content: vec![ContentPart::Text { text: "Say hello in one sentence.".into() }],
}];

let mut state = DynState::new();
state.insert("messages".into(), serde_json::to_value(messages)?);

let config = SentinelConfig {
    max_steps: 2,
    step_budget: TokenBudget { max_context_tokens: 8_192, max_completion_tokens: 1_024 },
    tool_retry: RetryPolicy::no_retry(),
};

let outcome = SentinelOrchestrator::default().run_agent(&agent, state, config).await?;
# Ok(()) }
```

## What Happens

- `SentinelAgent::new` builds an internal think/act/exit graph.
- The initial state must include `messages`.
- The agent calls the provider, optionally calls tools, then returns a `StepOutcome`.

## Choosing a Provider

See the full [LLM Providers Guide](./llm-providers.md) for all 19 supported providers, including cloud aggregators and local inference.

| Use Case | Recommended |
|---|---|
| Production (proprietary) | `OpenAiCompatConduit::openai()` or `AnthropicConduit::new()` |
| Multi-model routing | `OpenAiCompatConduit::openrouter()` |
| Fast open-source inference | `OpenAiCompatConduit::groq()` |
| Local development | `OpenAiCompatConduit::ollama()` |
| Enterprise cloud | `AzureConduit`, `BedrockConduit`, or `VertexConduit` |

## Security Notes

- API keys are **never** printed in debug output — all conduits redact keys.
- Use environment variables (`from_env()`) in CI/CD and production.
- Token budgets prevent accidental overspend before requests are sent.
