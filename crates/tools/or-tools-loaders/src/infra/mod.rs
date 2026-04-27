#[cfg(feature = "csv")]
pub mod csv_loader;
#[cfg(feature = "html")]
pub mod html;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "markdown")]
pub mod markdown;
#[cfg(feature = "pdf")]
pub mod pdf;
#[cfg(feature = "text")]
pub mod text;

pub(crate) mod shared;
