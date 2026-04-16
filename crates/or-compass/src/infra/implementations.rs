use crate::domain::entities::RouteSelection;
use crate::domain::errors::CompassError;
use or_core::OrchState;
use std::sync::Arc;

type Predicate<T> = Arc<dyn Fn(&T) -> bool + Send + Sync + 'static>;

#[derive(Clone)]
struct Route<T: OrchState> {
    name: String,
    predicate: Predicate<T>,
}

/// Not serializable because it stores executable route predicates.
#[derive(Clone)]
pub struct CompassRouterBuilder<T: OrchState> {
    routes: Vec<Route<T>>,
    default_route: Option<String>,
}

impl<T: OrchState> Default for CompassRouterBuilder<T> {
    fn default() -> Self {
        Self {
            routes: Vec::new(),
            default_route: None,
        }
    }
}

impl<T: OrchState> CompassRouterBuilder<T> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn add_route<F>(mut self, name: &str, predicate: F) -> Self
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.routes.push(Route {
            name: name.trim().to_owned(),
            predicate: Arc::new(predicate),
        });
        self
    }

    #[must_use]
    pub fn set_default(mut self, route: &str) -> Self {
        self.default_route = Some(route.to_owned());
        self
    }

    pub fn build(self) -> Result<CompassRouter<T>, CompassError> {
        validate_routes(&self.routes, self.default_route.as_deref())?;
        Ok(CompassRouter {
            routes: self.routes,
            default_route: self.default_route,
        })
    }
}

/// Not serializable because it stores executable route predicates.
#[derive(Clone)]
pub struct CompassRouter<T: OrchState> {
    routes: Vec<Route<T>>,
    default_route: Option<String>,
}

impl<T: OrchState> CompassRouter<T> {
    pub fn select(&self, state: &T) -> Result<RouteSelection, CompassError> {
        if let Some(route) = self.routes.iter().find(|route| (route.predicate)(state)) {
            return Ok(RouteSelection {
                route: route.name.clone(),
            });
        }
        if let Some(route) = &self.default_route {
            return Ok(RouteSelection {
                route: route.clone(),
            });
        }
        Err(CompassError::NoMatchingRoute)
    }
}

fn validate_routes<T: OrchState>(
    routes: &[Route<T>],
    default_route: Option<&str>,
) -> Result<(), CompassError> {
    if routes.is_empty() {
        return Err(CompassError::EmptyRouter);
    }

    let mut names = std::collections::BTreeSet::new();
    for route in routes {
        let name = route.name.trim();
        if name.is_empty() {
            return Err(CompassError::BlankRouteName);
        }
        if !names.insert(name.to_owned()) {
            return Err(CompassError::DuplicateRoute(name.to_owned()));
        }
    }

    if let Some(default_route) = default_route {
        if !names.contains(default_route) {
            return Err(CompassError::MissingDefaultRoute(default_route.to_owned()));
        }
    }
    Ok(())
}
