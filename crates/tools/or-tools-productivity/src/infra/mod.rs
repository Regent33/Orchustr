#[cfg(any(
    feature = "gmail",
    feature = "gcalendar",
    feature = "slack",
    feature = "jira",
    feature = "github",
    feature = "trello",
    feature = "notion",
    feature = "clickup",
    feature = "office365"
))]
pub(crate) mod shared;

#[cfg(feature = "clickup")]
pub mod clickup;
#[cfg(feature = "gcalendar")]
pub mod gcalendar;
#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "gmail")]
pub mod gmail;
#[cfg(feature = "jira")]
pub mod jira;
#[cfg(feature = "notion")]
pub mod notion;
#[cfg(feature = "office365")]
pub mod office365;
#[cfg(feature = "slack")]
pub mod slack;
#[cfg(feature = "trello")]
pub mod trello;
