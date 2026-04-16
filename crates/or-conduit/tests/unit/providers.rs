//! Provider construction, payload shaping, and security tests for all conduit types.

use or_conduit::{
    AI21Conduit, AnthropicConduit, AzureConduit, BedrockConduit, CohereConduit, ConduitError,
    GeminiConduit, HuggingFaceConduit, OpenAiCompatConduit, ReplicateConduit, VertexConduit,
};

// ---------------------------------------------------------------------------
// 1. Construction tests — every provider builds with explicit keys
// ---------------------------------------------------------------------------

#[test]
fn openai_conduit_constructs_with_explicit_key() {
    let c = OpenAiCompatConduit::openai("sk-test", "gpt-4o").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"), "api_key must be redacted in Debug");
    assert!(!dbg.contains("sk-test"), "raw key must not leak in Debug");
}

#[test]
fn openrouter_conduit_constructs_with_explicit_key() {
    let c = OpenAiCompatConduit::openrouter("or-key", "meta-llama/llama-3").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("openrouter.ai"), "base_url should be openrouter");
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn together_conduit_constructs() {
    let c = OpenAiCompatConduit::together("tog-key", "mistral-7b").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("together"));
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn groq_conduit_constructs() {
    let c = OpenAiCompatConduit::groq("gsk-key", "llama3-70b").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("groq"));
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn fireworks_conduit_constructs() {
    let c = OpenAiCompatConduit::fireworks("fw-key", "llama-v3p1-70b").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("fireworks"));
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn deepseek_conduit_constructs() {
    let c = OpenAiCompatConduit::deepseek("ds-key", "deepseek-chat").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("deepseek"));
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn mistral_conduit_constructs() {
    let c = OpenAiCompatConduit::mistral("mis-key", "mistral-large").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("mistral"));
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn xai_conduit_constructs() {
    let c = OpenAiCompatConduit::xai("xai-key", "grok-2").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("x.ai"));
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn nvidia_conduit_constructs() {
    let c = OpenAiCompatConduit::nvidia("nv-key", "nemotron-4-340b").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("nvidia"));
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn ollama_conduit_constructs() {
    let c = OpenAiCompatConduit::ollama("llama3").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("localhost") || dbg.contains("127.0.0.1"));
}

#[test]
fn anthropic_conduit_constructs_and_redacts() {
    let c = AnthropicConduit::new("ant-key", "claude-3").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"));
    assert!(!dbg.contains("ant-key"));
}

#[test]
fn gemini_conduit_constructs_and_redacts() {
    let c = GeminiConduit::new("gem-key", "gemini-pro").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn cohere_conduit_constructs_and_redacts() {
    let c = CohereConduit::new("co-key", "command-r-plus").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn ai21_conduit_constructs_and_redacts() {
    let c = AI21Conduit::new("ai21-key", "jamba-1.5").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn huggingface_conduit_constructs_and_redacts() {
    let c = HuggingFaceConduit::new("hf-key", "meta-llama/Llama-3").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn replicate_conduit_constructs_and_redacts() {
    let c = ReplicateConduit::new("rep-key", "meta/llama-3").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn azure_conduit_constructs_and_redacts() {
    let c = AzureConduit::new("az-key", "https://my.openai.azure.com", "gpt-4o", "2024-02-01")
        .unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"));
    assert!(!dbg.contains("az-key"));
}

#[test]
fn bedrock_conduit_constructs_and_redacts() {
    let c = BedrockConduit::new("br-key", "us-east-1", "anthropic.claude-3").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"));
}

#[test]
fn vertex_conduit_constructs_and_redacts() {
    let c = VertexConduit::new("vx-token", "my-project", "us-central1", "gemini-pro").unwrap();
    let dbg = format!("{c:?}");
    assert!(dbg.contains("REDACTED"));
}

// ---------------------------------------------------------------------------
// 2. from_env tests — verify env var requirements
// ---------------------------------------------------------------------------

#[test]
fn openai_from_env_fails_without_env_vars() {
    // Safety: test-only single-threaded env mutation
    unsafe { std::env::remove_var("OPENAI_API_KEY") };
    let result = OpenAiCompatConduit::openai_from_env();
    assert!(matches!(
        result,
        Err(ConduitError::MissingEnvironmentVariable(_))
    ));
}

#[test]
fn anthropic_from_env_fails_without_env_vars() {
    unsafe { std::env::remove_var("ANTHROPIC_API_KEY") };
    let result = AnthropicConduit::from_env();
    assert!(matches!(
        result,
        Err(ConduitError::MissingEnvironmentVariable(_))
    ));
}

// ---------------------------------------------------------------------------
// 3. Error variant coverage
// ---------------------------------------------------------------------------

#[test]
fn conduit_error_display_includes_context() {
    let err = ConduitError::AuthenticationFailed("bad token".to_owned());
    assert!(err.to_string().contains("bad token"));

    let err = ConduitError::Timeout;
    assert!(err.to_string().contains("timed out"));

    let err = ConduitError::BudgetExceeded {
        requested: 200_000,
        budget: 128_000,
    };
    assert!(err.to_string().contains("200000"));
    assert!(err.to_string().contains("128000"));
}

#[test]
fn conduit_error_equality() {
    assert_eq!(ConduitError::Timeout, ConduitError::Timeout);
    assert_ne!(
        ConduitError::Timeout,
        ConduitError::AuthenticationFailed("x".into())
    );
}
