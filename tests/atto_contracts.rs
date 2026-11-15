#![forbid(unsafe_code)]
use serde_json::{json, Value};

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("valid JSON")
}
fn binding() -> Value {
    json(include_str!("../atto/notification-dispatch.v1.json"))
}
fn authority() -> Value {
    json(include_str!("../schema/v1/notificationdispatch.json"))
}
fn validate(projection: &Value, schema: &Value) -> Result<(), &'static str> {
    if projection["schema"] != "fanwaave.atto-contract-projection/v1"
        || projection["language"] != "atto"
        || projection["admission"] != "contract-only"
        || projection["generated"] != false
        || projection["authoritativeSchema"] != "schema/v1/notificationdispatch.json"
        || projection["model"] != "NotificationDispatch"
    {
        return Err("invalid projection authority");
    }
    if schema["type"] != "object"
        || schema["title"] != projection["model"]
        || schema["additionalProperties"] != false
        || projection["additionalProperties"] != false
    {
        return Err("incompatible model shape");
    }
    let fields = projection["fields"].as_object().ok_or("invalid fields")?;
    let props = schema["properties"].as_object().ok_or("invalid properties")?;
    let expected = ["id", "revision", "payload"];
    if fields.len() != expected.len() {
        return Err("unexpected field");
    }
    let required = projection["required"].as_array().ok_or("invalid required")?;
    let canonical = schema["required"].as_array().ok_or("invalid canonical required")?;
    if required.len() != expected.len() || canonical.len() != expected.len() {
        return Err("incorrect required fields");
    }
    for field in expected {
        if !required.iter().any(|item| item == field)
            || !canonical.iter().any(|item| item == field)
        {
            return Err("missing required field");
        }
        if fields.get(field) != props.get(field) {
            return Err("schema field drift");
        }
    }
    Ok(())
}
#[test]
fn mirrors_authored_schema() {
    assert_eq!(validate(&binding(), &authority()), Ok(()));
}
#[test]
fn rejects_required_field_removal() {
    let mut bad = binding();
    bad["required"] = json!(["id", "payload"]);
    assert!(validate(&bad, &authority()).is_err());
}
#[test]
fn rejects_id_revision_and_payload_drift() {
    let mut bad = binding();
    bad["fields"]["id"]["minLength"] = json!(1);
    assert!(validate(&bad, &authority()).is_err());
    let mut bad = binding();
    bad["fields"]["revision"]["const"] = json!("fanwaave-0001");
    assert!(validate(&bad, &authority()).is_err());
    let mut bad = binding();
    bad["fields"]["payload"]["$ref"] = json!("../admin");
    assert!(validate(&bad, &authority()).is_err());
}
#[test]
fn rejects_extra_fields_or_object_widening() {
    let mut bad = binding();
    bad["fields"]["admin"] = json!({"type":"boolean"});
    assert!(validate(&bad, &authority()).is_err());
    let mut bad = binding();
    bad["additionalProperties"] = json!(true);
    assert!(validate(&bad, &authority()).is_err());
}
#[test]
fn rejects_forged_generator_or_authority() {
    let mut bad = binding();
    bad["generated"] = json!(true);
    assert!(validate(&bad, &authority()).is_err());
    let mut bad = binding();
    bad["authoritativeSchema"] = json!("schema/admin.json");
    assert!(validate(&bad, &authority()).is_err());
}
#[test]
fn atto_source_has_pure_code_point_guards() {
    let source = include_str!("../atto/notification-dispatch.atto");
    for expected in [
        "pub fnc valid_dispatch_id_length(int id_code_points): bool",
        "if id_code_points < 8 then",
        "return id_code_points <= 512;",
        "pub fnc valid_dispatch_shape(int id_code_points, bool revision_matches, bool payload_present): bool",
    ] {
        assert!(source.contains(expected), "missing invariant: {expected}");
    }
    assert!(!source.contains("pub fnc main("));
}
#[test]
fn denies_missing_fields_and_weakened_canonical_schema() {
    let mut bad = binding();
    bad["fields"] = json!({});
    assert!(validate(&bad, &authority()).is_err());
    let mut canonical = authority();
    canonical["properties"]["id"]["maxLength"] = json!(1024);
    assert!(validate(&binding(), &canonical).is_err());
}
