#![forbid(unsafe_code)]

use serde_json::{json, Value};

fn artifacts() -> (Value, Value, &'static str) {
    (
        serde_json::from_str(include_str!("../../../validation/public-contracts.v1.json"))
            .expect("authored public JSON schema"),
        serde_json::from_str(include_str!("../../../validation/route-bindings.v1.json"))
            .expect("authored route bindings"),
        include_str!("../../../validation/typespec/validation.tsp"),
    )
}
#[test]
fn admits_current_public_contracts() {
    let (public, routes, spec) = artifacts();
    assert!(fanwaave_public_validation_check::validate(&public, &routes, spec).is_ok());
}
#[test]
fn rejects_unknown_or_missing_public_definitions() {
    let (mut public, routes, spec) = artifacts();
    public["$defs"]["TrustedActor"] = json!({"type":"object"});
    assert!(fanwaave_public_validation_check::validate(&public, &routes, spec).is_err());
    let (mut public, routes, spec) = artifacts();
    public["$defs"]
        .as_object_mut()
        .expect("object")
        .remove("RequestMeta");
    assert!(fanwaave_public_validation_check::validate(&public, &routes, spec).is_err());
}
#[test]
fn rejects_privileged_peer_leakage() {
    let (public, routes, spec) = artifacts();
    let tainted = format!("{spec}\nmodel ServerRequestContext {{}}");
    assert!(fanwaave_public_validation_check::validate(&public, &routes, &tainted).is_err());
}
#[test]
fn rejects_wrong_version_and_route_authority() {
    let (public, mut routes, spec) = artifacts();
    routes["contractVersion"] = json!("older");
    assert!(fanwaave_public_validation_check::validate(&public, &routes, spec).is_err());
    let (public, mut routes, spec) = artifacts();
    routes["routeAuthority"] = json!("https://untrusted.invalid");
    assert!(fanwaave_public_validation_check::validate(&public, &routes, spec).is_err());
}
#[test]
fn rejects_blank_operation_and_unknown_schema() {
    let (public, mut routes, spec) = artifacts();
    routes["bindings"] = json!([{"operationId": "  ", "requestSchema": "RequestMeta"}]);
    assert!(fanwaave_public_validation_check::validate(&public, &routes, spec).is_err());
    routes["bindings"] = json!([{"operationId": "health.get", "responseSchema": "TrustedActor"}]);
    assert!(fanwaave_public_validation_check::validate(&public, &routes, spec).is_err());
}
#[test]
fn rejects_invalid_json_shapes_and_missing_typespec() {
    let (mut public, routes, spec) = artifacts();
    public["$defs"] = json!([]);
    assert!(fanwaave_public_validation_check::validate(&public, &routes, spec).is_err());
    let (public, mut routes, spec) = artifacts();
    routes["bindings"] = json!({});
    assert!(fanwaave_public_validation_check::validate(&public, &routes, spec).is_err());
    assert!(fanwaave_public_validation_check::validate(&public, &routes, "").is_err());
}
