use crate::domain::entities::RouteSelection;
use crate::domain::errors::CompassError;
use crate::infra::implementations::CompassRouter;
use or_core::OrchState;

#[derive(Debug, Clone, Default)]
pub struct CompassOrchestrator;

impl CompassOrchestrator {
    pub fn select_route<T: OrchState>(
        &self,
        router: &CompassRouter<T>,
        state: &T,
    ) -> Result<RouteSelection, CompassError> {
        let span = tracing::info_span!(
            "compass.select_route",
            otel.name = "compass.select_route",
            status = tracing::field::Empty,
        );
        let _guard = span.enter();
        let result = router.select(state);
        span.record("status", if result.is_ok() { "success" } else { "failure" });
        result
    }
}
