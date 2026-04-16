# or-conduit API Reference

This page documents the main public surface re-exported by `or-conduit/src/lib.rs`.

## Core Trait

### `ConduitProvider`
| Property | Value |
|---|---|
| **Kind** | trait |
| **Visibility** | pub |
| **File** | `domain/contracts.rs` |
| **Status** | 🟢 |

Async provider abstraction for message completion and streaming. All 22 providers implement this trait.

```rust
pub trait ConduitProvider: Send + Sync + 'static {
    async fn complete_messages(&self, messages: Vec<CompletionMessage>) -> Result<CompletionResponse, ConduitError>;
    async fn complete_text(&self, prompt: &str) -> Result<CompletionResponse, ConduitError>;
    async fn stream_text(&self, messages: Vec<CompletionMessage>) -> Result<TextStream, ConduitError>;
}
```

## Data Types

### `CompletionMessage`
Role-tagged message with multimodal content parts.

```rust
pub struct CompletionMessage {
    pub role: MessageRole,
    pub content: Vec<ContentPart>,
}
```

### `ContentPart`
Represents text, image, or document content within a message.

```rust
pub enum ContentPart {
    Text { text: String },
    Image { url: String, detail: ImageDetail },
    Document { data: String, media_type: String },
}
```

### `CompletionResponse`
Returned completion text, token usage, and finish reason.

```rust
pub struct CompletionResponse {
    pub text: String,
    pub usage: TokenUsage,
    pub finish_reason: FinishReason,
}
```

### `ConduitError`
Error type for provider operations.

```rust
pub enum ConduitError {
    MissingEnvironmentVariable(String),
    HttpError(String),
    ProviderError(String),
    BudgetExceeded { requested: usize, budget: usize },
    NotImplemented(String),
    Timeout,
    AuthenticationFailed(String),
}
```

## Provider Conduits

### `OpenAiCompatConduit`
| **File** | `infra/implementations/openai_compat.rs` |
|---|---|

Generic conduit for all OpenAI-compatible APIs. Factory constructors determine the target provider.

| Constructor | Provider |
|---|---|
| `::openai(key, model)` | OpenAI |
| `::openai_from_env()` | OpenAI (env vars) |
| `::openrouter(key, model)` | OpenRouter |
| `::together(key, model)` | Together AI |
| `::groq(key, model)` | Groq |
| `::fireworks(key, model)` | Fireworks AI |
| `::deepseek(key, model)` | DeepSeek |
| `::mistral(key, model)` | Mistral AI |
| `::xai(key, model)` | xAI (Grok) |
| `::nvidia(key, model)` | Nvidia (Nemotron) |
| `::ollama(model)` | Ollama (local, no key) |

All constructors return `Result<Self, ConduitError>`.

### `AnthropicConduit`
| **File** | `infra/implementations/anthropic.rs` |
|---|---|

Dedicated conduit for the Anthropic Messages API (`/v1/messages`). Uses `x-api-key` header.

```rust
AnthropicConduit::new(api_key, model) -> Result<Self, ConduitError>
AnthropicConduit::from_env() -> Result<Self, ConduitError>
```

### `GeminiConduit`
| **File** | `infra/implementations/gemini.rs` |
|---|---|

Conduit for Google's Gemini `generateContent` API. Uses query-string API key auth.

```rust
GeminiConduit::new(api_key, model) -> Result<Self, ConduitError>
GeminiConduit::from_env() -> Result<Self, ConduitError>
```

### `CohereConduit`
| **File** | `infra/implementations/cohere.rs` |
|---|---|

Conduit for Cohere's Chat API (`/v2/chat`). Uses bearer token auth.

```rust
CohereConduit::new(api_key, model) -> Result<Self, ConduitError>
CohereConduit::from_env() -> Result<Self, ConduitError>
```

### `AI21Conduit`
| **File** | `infra/implementations/ai21.rs` |
|---|---|

Conduit for AI21's Chat Completions API. Uses bearer token auth.

```rust
AI21Conduit::new(api_key, model) -> Result<Self, ConduitError>
AI21Conduit::from_env() -> Result<Self, ConduitError>
```

### `HuggingFaceConduit`
| **File** | `infra/implementations/huggingface.rs` |
|---|---|

Conduit for Hugging Face Inference API. Uses bearer token auth.

```rust
HuggingFaceConduit::new(api_key, model) -> Result<Self, ConduitError>
HuggingFaceConduit::from_env() -> Result<Self, ConduitError>
```

### `ReplicateConduit`
| **File** | `infra/implementations/replicate.rs` |
|---|---|

Conduit for Replicate's Predictions API. Uses bearer token auth.

```rust
ReplicateConduit::new(api_key, model) -> Result<Self, ConduitError>
ReplicateConduit::from_env() -> Result<Self, ConduitError>
```

### `AzureConduit`
| **File** | `infra/implementations/azure.rs` |
|---|---|

Conduit for Azure OpenAI Service. Uses `api-key` header with deployment-based URLs.

```rust
AzureConduit::new(api_key, resource_url, deployment, api_version) -> Result<Self, ConduitError>
```

### `BedrockConduit`
| **File** | `infra/implementations/bedrock.rs` |
|---|---|

Conduit for AWS Bedrock. Bring-your-own-token; uses bearer auth with STS/IAM tokens.

```rust
BedrockConduit::new(access_token, region, model_id) -> Result<Self, ConduitError>
```

### `VertexConduit`
| **File** | `infra/implementations/vertex.rs` |
|---|---|

Conduit for Google Vertex AI. Bring-your-own-token; uses OAuth2 bearer auth.

```rust
VertexConduit::new(access_token, project_id, region, model) -> Result<Self, ConduitError>
```

## Application Layer

### `ConduitOrchestrator`
| **File** | `application/orchestrators.rs` |
|---|---|

Application helper for preparing and executing completion requests.

```rust
pub struct ConduitOrchestrator;
```

## Security Notes

- All conduits implement `Debug` with `[REDACTED]` for API keys.
- `bearer_headers()` returns `Err(AuthenticationFailed)` on empty keys.
- Configurable HTTP timeout with `ConduitError::Timeout`.
