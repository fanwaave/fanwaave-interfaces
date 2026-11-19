#![forbid(unsafe_code)]
//! Data-only scoped contact read authority. Parsing is NOT signature verification.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const CONTACT_READ_ISSUER: &str = "fanwaave-contact-auth";
pub const CONTACT_READ_AUDIENCE: &str = "fanwaave-contact-reader";
pub const CONTACT_READ_SCOPE: &str = "contact_job.read";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContactReadOperation {
    GetContactJob { job_id: String },
}

impl ContactReadOperation {
    #[must_use]
    pub fn job_id(&self) -> &str {
        match self {
            Self::GetContactJob { job_id } => job_id,
        }
    }

    /// # Errors
    /// Rejects invalid or noncanonical UUIDs.
    pub fn validate(&self) -> Result<(), ContactReadContractError> {
        let id =
            Uuid::parse_str(self.job_id()).map_err(|_| ContactReadContractError::InvalidJobId)?;
        if id.to_string() != self.job_id() {
            return Err(ContactReadContractError::InvalidJobId);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContactReadGrantClaims {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub scope: String,
    pub job_id: String,
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
    pub jti: String,
}

impl ContactReadGrantClaims {
    /// Checks shape only. A trusted signature verifier MUST run before auth.
    ///
    /// # Errors
    /// Rejects inconsistent scope, UUIDs, principals and times.
    pub fn validate_shape(&self) -> Result<(), ContactReadContractError> {
        if self.iss != CONTACT_READ_ISSUER
            || self.aud != CONTACT_READ_AUDIENCE
            || self.scope != CONTACT_READ_SCOPE
        {
            return Err(ContactReadContractError::InvalidAuthority);
        }
        if self.sub.is_empty() || self.sub.len() > 128 {
            return Err(ContactReadContractError::InvalidPrincipal);
        }
        ContactReadOperation::GetContactJob {
            job_id: self.job_id.clone(),
        }
        .validate()?;
        if Uuid::parse_str(&self.jti).is_err() {
            return Err(ContactReadContractError::InvalidRequestId);
        }
        if self.iat < 0 || self.nbf < self.iat || self.exp <= self.nbf {
            return Err(ContactReadContractError::InvalidTimeWindow);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContactReadContractError {
    InvalidJobId,
    InvalidRequestId,
    InvalidAuthority,
    InvalidPrincipal,
    InvalidTimeWindow,
}

impl std::fmt::Display for ContactReadContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::InvalidJobId => "invalid contact job ID",
            Self::InvalidRequestId => "invalid contact read request ID",
            Self::InvalidAuthority => "unrecognized contact read authority",
            Self::InvalidPrincipal => "missing or invalid contact read principal",
            Self::InvalidTimeWindow => "invalid contact read time window",
        })
    }
}

impl std::error::Error for ContactReadContractError {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const ID: &str = "00000000-0000-4000-8000-000000000001";

    fn claims() -> ContactReadGrantClaims {
        ContactReadGrantClaims {
            iss: CONTACT_READ_ISSUER.into(),
            aud: CONTACT_READ_AUDIENCE.into(),
            sub: "service-contact-query".into(),
            scope: CONTACT_READ_SCOPE.into(),
            job_id: ID.into(),
            iat: 100,
            nbf: 100,
            exp: 200,
            jti: "00000000-0000-4000-8000-000000000003".into(),
        }
    }

    #[test]
    fn only_get_contact_job_is_deserializable() {
        let valid: ContactReadOperation =
            serde_json::from_value(json!({"kind": "get_contact_job", "job_id": ID}))
                .expect("canonical operation");
        assert_eq!(valid.validate(), Ok(()));
        for invalid in [
            json!({"kind": "delete_contact_job", "job_id": ID}),
            json!({"kind": "get_contact_job", "job_id": "not-uuid"}),
            json!({"kind": "get_contact_job", "job_id": ID, "extra": true}),
        ] {
            assert!(serde_json::from_value::<ContactReadOperation>(invalid)
                .map_or(true, |value| value.validate().is_err()));
        }
    }

    #[test]
    fn claim_shape_is_strict_but_is_not_a_signature() {
        let good = claims();
        assert_eq!(good.validate_shape(), Ok(()));
        let mut wrong = claims();
        wrong.scope = "contact_job.write".into();
        assert_eq!(
            wrong.validate_shape(),
            Err(ContactReadContractError::InvalidAuthority)
        );
        wrong = claims();
        wrong.exp = 99;
        assert_eq!(
            wrong.validate_shape(),
            Err(ContactReadContractError::InvalidTimeWindow)
        );
        wrong = claims();
        wrong.sub.clear();
        assert_eq!(
            wrong.validate_shape(),
            Err(ContactReadContractError::InvalidPrincipal)
        );
        wrong = claims();
        wrong.job_id = "all".into();
        assert_eq!(
            wrong.validate_shape(),
            Err(ContactReadContractError::InvalidJobId)
        );
    }

    #[test]
    fn unknown_claim_fields_are_rejected() {
        let mut obj = serde_json::to_value(claims()).expect("serializes");
        obj.as_object_mut()
            .expect("object")
            .insert("admin".into(), json!(true));
        assert!(serde_json::from_value::<ContactReadGrantClaims>(obj).is_err());
    }
}
