use crate::domain::entities::{ColonyMember, ColonyMessage};
use crate::domain::errors::ColonyError;
use or_core::DynState;
use std::future::Future;
use std::pin::Pin;

pub type ColonyFuture =
    Pin<Box<dyn Future<Output = Result<ColonyMessage, ColonyError>> + Send + 'static>>;

#[cfg_attr(test, mockall::automock)]
pub trait ColonyAgentTrait: Send + Sync + 'static {
    fn respond(
        &self,
        state: DynState,
        inbox: Vec<ColonyMessage>,
        member: ColonyMember,
    ) -> ColonyFuture;
}
