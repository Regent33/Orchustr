//! In-memory storage lives in `implementations`.
//! The optional SQLite backend lives in `sqlite`.

pub mod adapters;
pub mod implementations;
#[cfg(feature = "sqlite")]
pub mod sqlite;
