use std::env;

use chrono::{DateTime, Duration, Utc};
use serde_json::Value;

use crate::{DEFAULT_SESSION_TTL_SECONDS, MAX_FRAME_BYTES, MAX_TEXT_CHARS};

#[derive(Clone, Copy, Debug)]
pub struct SessionPolicy {
    ttl_seconds: i64,
}

impl SessionPolicy {
    pub fn from_env() -> Result<Self, String> {
        let ttl_raw = env::var("SESSION_TTL_SECONDS")
            .unwrap_or_else(|_| DEFAULT_SESSION_TTL_SECONDS.to_string());
        let ttl_seconds = ttl_raw
            .trim()
            .parse::<i64>()
            .map_err(|_| "SESSION_TTL_SECONDS must be an integer.".to_string())?;

        Self::from_ttl_seconds(ttl_seconds)
    }

    pub fn from_ttl_seconds(ttl_seconds: i64) -> Result<Self, String> {
        if ttl_seconds <= 0 {
            return Err("SESSION_TTL_SECONDS must be greater than zero.".to_string());
        }

        Ok(Self { ttl_seconds })
    }

    pub fn ttl_seconds(self) -> i64 {
        self.ttl_seconds
    }

    pub fn expires_at(self) -> DateTime<Utc> {
        Utc::now() + Duration::seconds(self.ttl_seconds)
    }
}

pub fn parse_and_validate_chat_text(raw: &str) -> Result<Option<String>, &'static str> {
    if raw.as_bytes().len() > MAX_FRAME_BYTES {
        return Err("Frame exceeds maximum allowed size (4096 bytes).");
    }

    let data: Value =
        serde_json::from_str(raw).map_err(|_| "Malformed JSON - message was not delivered.")?;
    let object = data.as_object().ok_or("Payload must be a JSON object.")?;

    if object.contains_key("sender") {
        return Err("Field 'sender' is server-owned and must not be sent by clients.");
    }

    let raw_text = object
        .get("text")
        .and_then(Value::as_str)
        .ok_or("Field 'text' must be a string.")?;
    let text = raw_text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    if text.chars().count() > MAX_TEXT_CHARS {
        return Err("Message text exceeds 1000 characters.");
    }

    Ok(Some(text.to_string()))
}

#[cfg(test)]
mod tests {
    use super::{parse_and_validate_chat_text, SessionPolicy};

    #[test]
    fn payload_validator_rejects_sender_field() {
        let result = parse_and_validate_chat_text(r#"{"sender":"alice","text":"hello"}"#);

        assert_eq!(
            result.unwrap_err(),
            "Field 'sender' is server-owned and must not be sent by clients."
        );
    }

    #[test]
    fn session_policy_rejects_non_positive_ttl() {
        let result = SessionPolicy::from_ttl_seconds(0);

        assert_eq!(
            result.unwrap_err(),
            "SESSION_TTL_SECONDS must be greater than zero."
        );
    }
}