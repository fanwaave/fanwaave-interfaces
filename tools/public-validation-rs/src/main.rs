#![forbid(unsafe_code)]

use serde_json::Value;
use std::{fs, path::Path};

fn read_json(path: &Path) -> Result<Value, String> {
    let data = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    serde_json::from_str(&data).map_err(|e| format!("{}: {e}", path.display()))
}

fn run() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let public = read_json(&root.join("validation/public-contracts.v1.json"))?;
    let bindings = read_json(&root.join("validation/route-bindings.v1.json"))?;
    let typespec_path = root.join("validation/typespec/validation.tsp");
    let typespec = fs::read_to_string(&typespec_path)
        .map_err(|e| format!("{}: {e}", typespec_path.display()))?;
    fanwaave_public_validation_check::validate(&public, &bindings, &typespec)?;
    println!("public validation authorities and route bindings are structurally coherent (Rust)");
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("public validation check failed: {error}");
        std::process::exit(1);
    }
}
