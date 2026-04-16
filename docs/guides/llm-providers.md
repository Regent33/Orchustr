# LLM Providers

`or-conduit` is the Rust provider abstraction layer. It implements a unified `ConduitProvider` trait for all supported LLM providers with retry and token-budget enforcement.

## Provider Architecture

All providers share a common HTTP infrastructure (`infra/http.rs`) with configurable timeouts, bearer auth, and retry logic. Providers fall into two categories:

- **OpenAI-compatible** — Providers that speak the `/v1/chat/completions` format are served by a single generic `OpenAiCompatConduit` struct with named factory constructors.
- **Dedicated** — Providers with unique APIs (Anthropic, Gemini, Cohere, etc.) have their own conduit types with custom payload adapters.

## Full Provider Table

| Provider | Conduit Type | Base URL | Env Var (Key) | Env Var (Model) |
|---|---|---|---|---|
| **OpenAI** | `OpenAiCompatConduit::openai()` | `api.openai.com/v1/responses` | `OPENAI_API_KEY` | `OPENAI_MODEL` |
| **OpenRouter** | `OpenAiCompatConduit::openrouter()` | `openrouter.ai/api/v1/chat/completions` | `OPENROUTER_API_KEY` | `OPENROUTER_MODEL` |
| **Together AI** | `OpenAiCompatConduit::together()` | `api.together.xyz/v1/chat/completions` | `TOGETHER_API_KEY` | `TOGETHER_MODEL` |
| **Groq** | `OpenAiCompatConduit::groq()` | `api.groq.com/openai/v1/chat/completions` | `GROQ_API_KEY` | `GROQ_MODEL` |
| **Fireworks AI** | `OpenAiCompatConduit::fireworks()` | `api.fireworks.ai/inference/v1/chat/completions` | `FIREWORKS_API_KEY` | `FIREWORKS_MODEL` |
| **DeepSeek** | `OpenAiCompatConduit::deepseek()` | `api.deepseek.com/v1/chat/completions` | `DEEPSEEK_API_KEY` | `DEEPSEEK_MODEL` |
| **Mistral AI** | `OpenAiCompatConduit::mistral()` | `api.mistral.ai/v1/chat/completions` | `MISTRAL_API_KEY` | `MISTRAL_MODEL` |
| **xAI (Grok)** | `OpenAiCompatConduit::xai()` | `api.x.ai/v1/chat/completions` | `XAI_API_KEY` | `XAI_MODEL` |
| **Nvidia** | `OpenAiCompatConduit::nvidia()` | `integrate.api.nvidia.com/v1/chat/completions` | `NVIDIA_API_KEY` | `NVIDIA_MODEL` |
| **Ollama** | `OpenAiCompatConduit::ollama()` | `localhost:11434/v1/chat/completions` | *(none)* | *(constructor arg)* |
| **Anthropic** | `AnthropicConduit` | `api.anthropic.com/v1/messages` | `ANTHROPIC_API_KEY` | `ANTHROPIC_MODEL` |
| **Google Gemini** | `GeminiConduit` | `generativelanguage.googleapis.com` | `GEMINI_API_KEY` | `GEMINI_MODEL` |
| **Cohere** | `CohereConduit` | `api.cohere.com/v2/chat` | `COHERE_API_KEY` | `COHERE_MODEL` |
| **AI21 Labs** | `AI21Conduit` | `api.ai21.com/studio/v1/chat/completions` | `AI21_API_KEY` | `AI21_MODEL` |
| **Hugging Face** | `HuggingFaceConduit` | `api-inference.huggingface.co` | `HF_API_KEY` | `HF_MODEL` |
| **Replicate** | `ReplicateConduit` | `api.replicate.com/v1/predictions` | `REPLICATE_API_KEY` | `REPLICATE_MODEL` |
| **Azure OpenAI** | `AzureConduit` | `{resource}.openai.azure.com` | `AZURE_OPENAI_API_KEY` | *(constructor args)* |
| **AWS Bedrock** | `BedrockConduit` | `bedrock-runtime.{region}.amazonaws.com` | `AWS_BEDROCK_ACCESS_KEY` | *(constructor args)* |
| **Google Vertex AI** | `VertexConduit` | `{region}-aiplatform.googleapis.com` | `VERTEX_ACCESS_TOKEN` | *(constructor args)* |

## Aggregators vs Direct Providers

**Direct providers** (OpenAI, Anthropic, Gemini, etc.) serve their own models from their own infrastructure.

**Aggregators/routers** (OpenRouter, Together, Groq, Fireworks, Bedrock, Vertex, Azure) proxy requests to underlying models. They are distinct services with their own API keys, pricing, and model namespaces:

- **OpenRouter** — Multi-provider LLM router with unified billing; access OpenAI, Anthropic, Meta, Mistral, and others through a single API key.
- **Groq** — Hardware-accelerated inference for open models (Llama, Mixtral, etc.).
- **Together AI** — Cloud inference for open-source models with fine-tuning support.
- **Fireworks AI** — Optimized inference for open-source models with custom deployments.
- **AWS Bedrock** — Amazon's managed AI service; bring-your-own-token (IAM credentials).
- **Google Vertex AI** — Google Cloud's managed AI service; bring-your-own-token (OAuth2).
- **Azure OpenAI** — Microsoft's hosted OpenAI models; bring-your-own-token + deployment config.

## Usage Examples

### Direct Provider (Explicit Key)

```rust
use or_conduit::OpenAiCompatConduit;

let provider = OpenAiCompatConduit::openai("sk-your-key", "gpt-4o")?;
```

### Direct Provider (From Environment)

```rust
use or_conduit::OpenAiCompatConduit;

// Reads OPENAI_API_KEY and OPENAI_MODEL from the environment
let provider = OpenAiCompatConduit::openai_from_env()?;
```

### OpenRouter

```rust
use or_conduit::OpenAiCompatConduit;

let provider = OpenAiCompatConduit::openrouter("or-key", "meta-llama/llama-3-70b")?;
```

### Anthropic

```rust
use or_conduit::AnthropicConduit;

let provider = AnthropicConduit::new("ant-key", "claude-sonnet-4-20250514")?;
```

### Google Gemini

```rust
use or_conduit::GeminiConduit;

let provider = GeminiConduit::new("gem-key", "gemini-2.5-pro")?;
```

### Local Ollama

```rust
use or_conduit::OpenAiCompatConduit;

// No API key needed — connects to localhost:11434
let provider = OpenAiCompatConduit::ollama("llama3")?;
```

## Request Shape

The Rust provider trait works on `Vec<CompletionMessage>` rather than raw prompt strings. Use `complete_text` only as a convenience wrapper when you do not need multi-message control.

## Security

- API keys are **redacted** in all `Debug` output (`[REDACTED]`).
- Configurable request timeout (default 30s) prevents hung connections.
- `bearer_headers()` returns `Result`, failing early on missing/empty keys instead of silently sending unauthenticated requests.
- Token budget enforcement prevents accidental overspend before the request is sent.
