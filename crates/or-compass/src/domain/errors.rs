use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize, Error, PartialEq, Eq)]
pub enum CompassError {
    #[error("router must contain at least one route")]
    EmptyRouter,
    #[error("route name must not be blank")]
    BlankRouteName,
    #[error("duplicate route: {0}")]
    DuplicateRoute(String),
    #[error("default route not found: {0}")]
    MissingDefaultRoute(String),
    #[error("no route matched and no default route was set")]
    NoMatchingRoute,
}
