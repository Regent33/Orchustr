pub mod application;
pub mod domain;
pub mod infra;

pub use application::orchestrators::{ProductivityOrchestrator, ProductivityTool};
pub use domain::contracts::{
    CalendarClient, EmailClient, KnowledgeBase, ProjectTracker, TeamMessenger,
};
pub use domain::entities::{CalendarEvent, Email, Issue, Page, ProductivityTask};
pub use domain::errors::ProductivityError;
