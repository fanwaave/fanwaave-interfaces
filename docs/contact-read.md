# Scoped contact reads — authority stage

This contract adds exactly one operation: get_contact_job, with a
canonical lowercase UUID and the existing ContactDelivery.ContactJobView
as the response type. No new database schema or mutation operation exists.

The read operation is data only. Authorization travels separately as the
standard ores_transport::Envelope bearer header, never inside operation data.

## Authenticated, job-bound bearer grants

For this proposed internal integration, a trusted identity/authorization
service must issue a signed JWT using HS256 with a strong key selected by
the JOSE kid header. Required claims:

- iss = fanwaave-contact-auth
- aud = fanwaave-contact-reader
- sub = authenticated, nonempty principal identifier
- scope = contact_job.read
- job_id = exact resource UUID
- iat, nbf, exp = seconds since epoch
- jti = UUID identifying the individual grant

A real verifier must first enforce a trusted kid allowlist, HS256 only,
cryptographic signature, issuer, audience, timestamps, a short maximum
lifetime, principal, resource ID, and scope. Expired, wrong-job, wrong-scope,
tampered or unknown-key tokens are denied BEFORE database access.
Rotating issuers can have overlapping known keys; unknown keys never
fall back to a global secret.

IMPORTANT: the data structures and their structural validation do not
authenticate anything. sub, job_id and other fields in a decoded,
unsigned object are attacker controlled until signature verification.
Nothing in this repository issues JWTs, stores signing keys, authorizes an
end user, or exposes a public HTTP endpoint. The trusted issuer and its
mapping of principals to resource access remain separate, independently
reviewed production decisions. The current schema has no tenant_id, and
this contract does not claim database-enforced tenant isolation.

The base contact job view authority remains
contracts/contact-delivery.tsp and schemas/contact-delivery.schema.json;
do not duplicate it in this file.

This proposal remains draft until issuer integration, key management,
source-executed CI and independent security review are completed.
