#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::InterfaceError;

pub const PROTOCOL_VERSION: &str = "1";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Health {
    pub ok: bool,
    pub service: String,
    pub protocol: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationDispatch {
    #[serde(deserialize_with = "deserialize_non_blank")]
    pub id: String,
    #[serde(deserialize_with = "deserialize_non_blank")]
    pub revision: String,
    #[serde(default)]
    pub payload: Value,
}


fn deserialize_non_blank<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    if value.trim().is_empty() {
        // Never include the rejected identifier in a parser error.
        return Err(serde::de::Error::custom("notification identifier must be non-empty"));
    }
    Ok(value)
}

impl NotificationDispatch {
    pub fn parse(id: String, revision: String, payload: Value) -> Result<Self, InterfaceError> {
        if id.trim().is_empty() {
            return Err(InterfaceError::EmptyId);
        }
        if revision.trim().is_empty() {
            return Err(InterfaceError::EmptyRevision);
        }
        Ok(Self { id, revision, payload })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_serde_cannot_bypass_notification_id_validation() {
        let valid = r#"{"id":"event-1","revision":"rev-1"}"#;
        let parsed: NotificationDispatch = serde_json::from_str(valid).unwrap();
        assert_eq!(parsed.id, "event-1");
        assert_eq!(parsed.revision, "rev-1");
        assert_eq!(parsed.payload, serde_json::Value::Null);

        for invalid in [
            r#"{"id":"","revision":"rev-1"}"#,
            r#"{"id":"   ","revision":"rev-1"}"#,
            r#"{"id":"event-1","revision":""}"#,
            r#"{"id":"event-1","revision":"\t"}"#,
        ] {
            assert!(serde_json::from_str::<NotificationDispatch>(invalid).is_err());
        }
    }

    #[test]
    fn explicit_constructor_rejects_blank_fields() {
        assert!(matches!(
            NotificationDispatch::parse(" ".into(), "rev".into(), Value::Null),
            Err(InterfaceError::EmptyId)
        ));
        assert!(matches!(
            NotificationDispatch::parse("id".into(), "\t".into(), Value::Null),
            Err(InterfaceError::EmptyRevision)
        ));
    }
}
