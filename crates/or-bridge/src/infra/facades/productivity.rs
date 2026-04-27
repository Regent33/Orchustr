//! Bridge entry point for `or-tools-productivity` (email / calendar /
//! issue tracker / knowledge base / team messenger providers).

use super::helpers::{
    block_on, from_field, get_str, invocation, json_value, required_str, unsupported,
    unsupported_provider,
};
use crate::domain::errors::BridgeError;
use or_tools_productivity::infra::{
    clickup::ClickUpTracker,
    gcalendar::GoogleCalendarClient,
    github::GitHubTracker,
    gmail::GmailClient,
    jira::JiraTracker,
    notion::NotionBase,
    office365::{OutlookCalendarClient, OutlookEmailClient},
    slack::SlackMessenger,
    trello::TrelloTracker,
};
use or_tools_productivity::{
    CalendarClient, CalendarEvent, Email, EmailClient, Issue, KnowledgeBase, Page, ProjectTracker,
    TeamMessenger,
};
use serde_json::{Value, json};

pub(crate) fn invoke(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    let provider_name = required_str(&payload, "provider", "or-tools-productivity", operation)?;
    match operation {
        "list_emails" => {
            let client = build_email_client(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on("or-tools-productivity", operation, client.list(query)).and_then(json_value)
        }
        "send_email" => {
            let client = build_email_client(provider_name, payload.get("config"))?;
            let email: Email = from_field(&payload, "item", "or-tools-productivity", operation)?;
            let id = block_on("or-tools-productivity", operation, client.send_email(email))?;
            Ok(json!({ "id": id }))
        }
        "list_events" => {
            let client = build_calendar_client(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on(
                "or-tools-productivity",
                operation,
                client.list_events(query),
            )
            .and_then(json_value)
        }
        "create_event" => {
            let client = build_calendar_client(provider_name, payload.get("config"))?;
            let event: CalendarEvent =
                from_field(&payload, "item", "or-tools-productivity", operation)?;
            let id = block_on(
                "or-tools-productivity",
                operation,
                client.create_event(event),
            )?;
            Ok(json!({ "id": id }))
        }
        "list_issues" => {
            let tracker = build_project_tracker(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on(
                "or-tools-productivity",
                operation,
                tracker.list_issues(query),
            )
            .and_then(json_value)
        }
        "create_issue" => {
            let tracker = build_project_tracker(provider_name, payload.get("config"))?;
            let issue: Issue = from_field(&payload, "item", "or-tools-productivity", operation)?;
            let id = block_on(
                "or-tools-productivity",
                operation,
                tracker.create_issue(issue),
            )?;
            Ok(json!({ "id": id }))
        }
        "search_pages" => {
            let kb = build_knowledge_base(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on("or-tools-productivity", operation, kb.search(query)).and_then(json_value)
        }
        "create_page" => {
            let kb = build_knowledge_base(provider_name, payload.get("config"))?;
            let page: Page = from_field(&payload, "item", "or-tools-productivity", operation)?;
            let id = block_on("or-tools-productivity", operation, kb.create_page(page))?;
            Ok(json!({ "id": id }))
        }
        "post_message" => {
            let messenger = build_team_messenger(provider_name, payload.get("config"))?;
            let channel = required_str(&payload, "channel", "or-tools-productivity", operation)?;
            let text = required_str(&payload, "text", "or-tools-productivity", operation)?;
            let id = block_on(
                "or-tools-productivity",
                operation,
                messenger.post(channel, text),
            )?;
            Ok(json!({ "id": id }))
        }
        "search_messages" => {
            let messenger = build_team_messenger(provider_name, payload.get("config"))?;
            let query = payload.get("query").cloned().unwrap_or(Value::Null);
            block_on(
                "or-tools-productivity",
                operation,
                messenger.search_messages(query),
            )
            .and_then(json_value)
        }
        _ => Err(unsupported("or-tools-productivity", operation)),
    }
}

fn build_email_client(
    provider: &str,
    config: Option<&Value>,
) -> Result<Box<dyn EmailClient>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let email: Box<dyn EmailClient> = match provider {
        "gmail" => Box::new(
            if let (Some(base_url), Some(access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "access_token")),
            ) {
                GmailClient::with_config(client, base_url, access_token)
            } else {
                GmailClient::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "email", error))?
            },
        ),
        "office365" => Box::new(
            if let (Some(base_url), Some(access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "access_token")),
            ) {
                OutlookEmailClient::with_config(client, base_url, access_token)
            } else {
                OutlookEmailClient::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "email", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(email)
}

fn build_calendar_client(
    provider: &str,
    config: Option<&Value>,
) -> Result<Box<dyn CalendarClient>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let calendar: Box<dyn CalendarClient> = match provider {
        "gcalendar" => Box::new(
            if let (Some(base_url), Some(access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "access_token")),
            ) {
                GoogleCalendarClient::with_config(client, base_url, access_token)
            } else {
                GoogleCalendarClient::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "calendar", error))?
            },
        ),
        "office365" => Box::new(
            if let (Some(base_url), Some(access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "access_token")),
            ) {
                OutlookCalendarClient::with_config(client, base_url, access_token)
            } else {
                OutlookCalendarClient::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "calendar", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(calendar)
}

fn build_project_tracker(
    provider: &str,
    config: Option<&Value>,
) -> Result<Box<dyn ProjectTracker>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let tracker: Box<dyn ProjectTracker> = match provider {
        "jira" => Box::new(
            if let (Some(base_url), Some(auth_header)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "auth_header")),
            ) {
                JiraTracker::with_config(client, base_url, auth_header)
            } else {
                JiraTracker::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "tracker", error))?
            },
        ),
        "github" => Box::new(
            if let (Some(base_url), Some(token), Some(owner), Some(repo)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "token")),
                cfg.and_then(|v| get_str(v, "owner")),
                cfg.and_then(|v| get_str(v, "repo")),
            ) {
                GitHubTracker::with_config(client, base_url, token, owner, repo)
            } else {
                GitHubTracker::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "tracker", error))?
            },
        ),
        "trello" => Box::new(
            if let (Some(base_url), Some(api_key), Some(token), Some(list_id)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "api_key")),
                cfg.and_then(|v| get_str(v, "token")),
                cfg.and_then(|v| get_str(v, "list_id")),
            ) {
                TrelloTracker::with_config(client, base_url, api_key, token, list_id)
            } else {
                TrelloTracker::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "tracker", error))?
            },
        ),
        "clickup" => Box::new(
            if let (Some(base_url), Some(api_key), Some(list_id)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "api_key")),
                cfg.and_then(|v| get_str(v, "list_id")),
            ) {
                ClickUpTracker::with_config(client, base_url, api_key, list_id)
            } else {
                ClickUpTracker::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "tracker", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(tracker)
}

fn build_knowledge_base(
    provider: &str,
    config: Option<&Value>,
) -> Result<Box<dyn KnowledgeBase>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let knowledge: Box<dyn KnowledgeBase> = match provider {
        "notion" => Box::new(
            if let (Some(base_url), Some(api_key), Some(database_id)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "api_key")),
                cfg.and_then(|v| get_str(v, "database_id")),
            ) {
                NotionBase::with_config(client, base_url, api_key, database_id)
            } else {
                NotionBase::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "knowledge", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(knowledge)
}

fn build_team_messenger(
    provider: &str,
    config: Option<&Value>,
) -> Result<Box<dyn TeamMessenger>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let messenger: Box<dyn TeamMessenger> = match provider {
        "slack" => Box::new(
            if let (Some(base_url), Some(bot_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "bot_token")),
            ) {
                SlackMessenger::with_config(client, base_url, bot_token)
            } else {
                SlackMessenger::from_env()
                    .map_err(|error| invocation("or-tools-productivity", "messenger", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-productivity", other)),
    };
    Ok(messenger)
}
