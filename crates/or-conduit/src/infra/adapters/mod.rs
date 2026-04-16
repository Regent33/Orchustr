//! Provider-specific payload/response shaping modules.
//!
//! Each sub-module handles request construction and response parsing
//! for a specific LLM API format.

pub(crate) mod ai21;
pub(crate) mod anthropic;
pub(crate) mod bedrock;
pub(crate) mod cohere;
pub(crate) mod gemini;
pub(crate) mod huggingface;
pub(crate) mod openai_compat;
pub(crate) mod replicate;
pub(crate) mod vertex;
