//! Provider payload/response shaping lives in `adapters/`.
//! Shared HTTP execution and retry helpers live in `http`.
//! Concrete provider structs and trait impls stay in `implementations/`.

pub(crate) mod adapters;
pub(crate) mod http;
pub mod implementations;
