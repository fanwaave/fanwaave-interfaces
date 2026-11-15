#![forbid(unsafe_code)]

use serde_json::Value;

const PUBLIC: [&str; 3] = ["RequestMeta", "PageQuery", "ProblemDetails"];
const PRIVATE: [&str; 3] = ["TrustedActor", "ServerRequestContext", "InternalCommand"];

pub fn validate(public: &Value, bindings: &Value, typespec: &str) -> Result<(), String> {
    if public["contractVersion"] != "ores.validation.v1" || public["visibility"] != "public" {
        return Err("invalid public schema version or visibility".into());
    }
    let definitions = public["$defs"]
        .as_object()
        .ok_or("public definitions must be an object")?;
    if definitions.len() != PUBLIC.len() || PUBLIC.iter().any(|name| !definitions.contains_key(*name))
    {
        return Err("public definition set drift".into());
    }
    if bindings["contractVersion"] != public["contractVersion"]
        || bindings["routeAuthority"] != public["routeAuthority"]
    {
        return Err("route-binding authority or contract version drift".into());
    }
    let authored_json = serde_json::to_string(public).map_err(|e| e.to_string())?;
    for name in PUBLIC {
        if !typespec.contains(&format!("model {name}")) {
            return Err(format!("missing TypeSpec peer model: {name}"));
        }
    }
    for name in PRIVATE {
        if authored_json.contains(name) || typespec.contains(name) {
            return Err(format!("private model leaked into public contracts: {name}"));
        }
    }
    let routes = bindings["bindings"]
        .as_array()
        .ok_or("route bindings must be an array")?;
    for (index, route) in routes.iter().enumerate() {
        let operation = route["operationId"]
            .as_str()
            .ok_or_else(|| format!("route {index} missing operationId"))?;
        if operation.trim().is_empty() {
            return Err(format!("route {index} has blank operationId"));
        }
        for field in ["requestSchema", "responseSchema", "errorSchema"] {
            if let Some(schema_name) = route.get(field) {
                let name = schema_name
                    .as_str()
                    .ok_or_else(|| format!("route {index} has nonstring {field}"))?;
                if !PUBLIC.contains(&name) {
                    return Err(format!("route {index} has unrecognized {field}: {name}"));
                }
            }
        }
    }
    Ok(())
}
