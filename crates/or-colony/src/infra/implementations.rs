use crate::domain::contracts::ColonyAgentTrait;
use crate::domain::entities::ColonyMember;
use crate::domain::errors::ColonyError;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct ColonyWorker {
    pub(crate) member: ColonyMember,
    pub(crate) agent: Arc<dyn ColonyAgentTrait>,
}

#[derive(Clone, Default)]
pub(crate) struct ColonyRoster {
    workers: Vec<ColonyWorker>,
}

impl ColonyRoster {
    pub(crate) fn add_member<A>(
        &mut self,
        name: &str,
        role: &str,
        agent: A,
    ) -> Result<(), ColonyError>
    where
        A: ColonyAgentTrait,
    {
        if self.workers.iter().any(|worker| worker.member.name == name) {
            return Err(ColonyError::DuplicateMember(name.to_owned()));
        }
        self.workers.push(ColonyWorker {
            member: ColonyMember {
                name: name.to_owned(),
                role: role.to_owned(),
            },
            agent: Arc::new(agent),
        });
        Ok(())
    }

    pub(crate) fn workers(&self) -> &[ColonyWorker] {
        &self.workers
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.workers.is_empty()
    }
}
