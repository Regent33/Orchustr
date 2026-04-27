#[cfg(feature = "arxiv")]
pub mod arxiv;
#[cfg(feature = "financial")]
pub mod financial;
#[cfg(feature = "gdrive")]
pub mod gdrive;
#[cfg(feature = "json-toolkit")]
pub mod json_toolkit;
#[cfg(feature = "local")]
pub mod local_fs;
#[cfg(any(feature = "gdrive", feature = "arxiv", feature = "financial"))]
pub(crate) mod shared;
