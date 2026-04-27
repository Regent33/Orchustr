//! Bridge entry point for `or-tools-comms` (outbound messaging
//! providers: SMS, WhatsApp, Discord, Telegram, etc.).

use super::helpers::{
    block_on, get_str, invocation, json_value, required_str, unsupported, unsupported_provider,
};
use crate::domain::errors::BridgeError;
use or_tools_comms::infra::{
    discord::DiscordSender, facebook::FacebookSender, messenger::MessengerSender,
    telegram::TelegramSender, twilio::TwilioSender, whatsapp::WhatsAppSender,
};
use or_tools_comms::{Channel, CommsOrchestrator, Message, MessageSender};
use serde_json::Value;
use std::sync::Arc;

pub(crate) fn invoke(operation: &str, payload: Value) -> Result<Value, BridgeError> {
    if operation != "send" {
        return Err(unsupported("or-tools-comms", operation));
    }
    let provider_name = required_str(&payload, "provider", "or-tools-comms", operation)?;
    let message = build_message(provider_name, &payload)?;
    let sender = build_message_sender(provider_name, payload.get("config"))?;
    let orchestrator = CommsOrchestrator::new(vec![sender]);
    block_on("or-tools-comms", operation, orchestrator.send(message)).and_then(json_value)
}

fn build_message(provider: &str, payload: &Value) -> Result<Message, BridgeError> {
    Ok(Message {
        channel: match provider {
            "sms" | "twilio" => Channel::Sms,
            "telegram" => Channel::Telegram,
            "discord" => Channel::Discord,
            "whatsapp" => Channel::WhatsApp,
            "facebook" => Channel::Facebook,
            "messenger" => Channel::Messenger,
            other => return Err(unsupported_provider("or-tools-comms", other)),
        },
        to: required_str(payload, "to", "or-tools-comms", "send")?.to_owned(),
        body: required_str(payload, "body", "or-tools-comms", "send")?.to_owned(),
        from: payload
            .get("from")
            .and_then(Value::as_str)
            .map(str::to_owned),
    })
}

fn build_message_sender(
    provider: &str,
    config: Option<&Value>,
) -> Result<Arc<dyn MessageSender>, BridgeError> {
    let cfg = config.and_then(Value::as_object);
    let client = reqwest::Client::new();
    let sender: Arc<dyn MessageSender> = match provider {
        "sms" | "twilio" => Arc::new(
            if let (Some(account_sid), Some(auth_token), Some(from)) = (
                cfg.and_then(|v| get_str(v, "account_sid")),
                cfg.and_then(|v| get_str(v, "auth_token")),
                cfg.and_then(|v| get_str(v, "from")),
            ) {
                TwilioSender::with_config(client, account_sid, auth_token, from)
            } else {
                TwilioSender::from_env()
                    .map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "telegram" => Arc::new(
            if let (Some(base_url), Some(bot_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "bot_token")),
            ) {
                TelegramSender::with_config(client, base_url, bot_token)
            } else {
                TelegramSender::from_env()
                    .map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "discord" => Arc::new(
            if let (Some(base_url), Some(bot_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| get_str(v, "bot_token")),
            ) {
                DiscordSender::with_config(client, base_url, bot_token)
            } else {
                DiscordSender::from_env()
                    .map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "whatsapp" => Arc::new(
            if let (Some(account_sid), Some(auth_token), Some(from)) = (
                cfg.and_then(|v| get_str(v, "account_sid")),
                cfg.and_then(|v| get_str(v, "auth_token")),
                cfg.and_then(|v| get_str(v, "from")),
            ) {
                WhatsAppSender::with_config(client, account_sid, auth_token, from)
            } else {
                WhatsAppSender::from_env()
                    .map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "facebook" => Arc::new(
            if let (Some(base_url), Some(page_access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| {
                    get_str(v, "page_token").or_else(|| get_str(v, "page_access_token"))
                }),
            ) {
                FacebookSender::with_config(client, base_url, page_access_token)
            } else {
                FacebookSender::from_env()
                    .map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        "messenger" => Arc::new(
            if let (Some(base_url), Some(page_access_token)) = (
                cfg.and_then(|v| get_str(v, "base_url")),
                cfg.and_then(|v| {
                    get_str(v, "page_token").or_else(|| get_str(v, "page_access_token"))
                }),
            ) {
                MessengerSender::with_config(client, base_url, page_access_token)
            } else {
                MessengerSender::from_env()
                    .map_err(|error| invocation("or-tools-comms", "send", error))?
            },
        ),
        other => return Err(unsupported_provider("or-tools-comms", other)),
    };
    Ok(sender)
}
