// Source-owned structural cross-authority check; only public JSON and TypeSpec.
// The real TypeSpec compiler separately proves that the TypeSpec text parses.
import { readFileSync } from "node:fs";

const spec = readFileSync("contracts/contact-read.tsp", "utf8");
const schema = JSON.parse(readFileSync("schemas/contact-read.schema.json", "utf8"));

const expected = new Map([
  ["ContactReadOperation", "operation"],
  ["ContactReadGrantClaims", "claims"],
]);
const models = new Map();
for (const match of spec.matchAll(/model\s+([A-Za-z_][A-Za-z0-9_]*)\s*\{([^}]*)\}/g)) {
  if (models.has(match[1])) throw Error("duplicate model " + match[1]);
  const fields = new Map();
  for (const raw of match[2].split("\n")) {
    const line = raw.trim();
    if (!line || line.startsWith("//")) continue;
    const field = line.match(/^([A-Za-z_][A-Za-z0-9_]*):\s*(string|int64|"[^"]+");$/);
    if (!field) throw Error("unsupported TypeSpec field " + line);
    if (fields.has(field[1])) throw Error("duplicate field " + field[1]);
    fields.set(field[1], field[2]);
  }
  models.set(match[1], fields);
}
function check(specModels, jsonSchema) {
  const names = [...specModels.keys()].sort().join(",");
  if (names !== [...expected.keys()].sort().join(",")) {
    throw Error("unexpected TypeSpec models");
  }
  if (!jsonSchema.$defs || [...Object.keys(jsonSchema.$defs)].sort().join(",") !== "claims,operation") {
    throw Error("unexpected JSON Schema model definitions");
  }
  for (const [modelName, defName] of expected) {
    const fields = specModels.get(modelName);
    const def = jsonSchema.$defs[defName];
    if (def.type !== "object" || def.additionalProperties !== false) {
      throw Error("object must reject unexpected fields: " + defName);
    }
    const fieldNames = [...fields.keys()].sort().join(",");
    const properties = [...Object.keys(def.properties || {})].sort().join(",");
    const required = [...(def.required || [])].sort().join(",");
    if (fieldNames !== properties || fieldNames !== required) {
      throw Error("required fields drifted: " + defName);
    }
    for (const [field, kind] of fields) {
      const js = def.properties[field];
      if (kind === "string" && js.type !== "string") throw Error("expected string: " + field);
      if (kind === "int64" && js.type !== "integer") throw Error("expected integer: " + field);
      if (kind.startsWith('"') && js.const !== kind.slice(1, -1)) {
        throw Error("literal drift: " + field);
      }
    }
  }
  const refs = jsonSchema.oneOf?.map((v) => v.$ref).sort().join(",");
  if (refs !== "#/$defs/claims,#/$defs/operation") {
    throw Error("top-level union drifted");
  }
}
check(models, schema);

// Negative controls prove the verifier cannot silently accept drift.
const tampered = structuredClone(schema);
tampered.$defs.claims.properties.scope.const = "contact_job.write";
let rejected = false;
try {
  check(models, tampered);
} catch {
  rejected = true;
}
if (!rejected) throw Error("peer drift negative control was not rejected");

const loosened = structuredClone(schema);
loosened.$defs.operation.additionalProperties = true;
rejected = false;
try {
  check(models, loosened);
} catch {
  rejected = true;
}
if (!rejected) throw Error("extra-property negative control was not rejected");

console.log("ContactRead TypeSpec/JSON Schema structural peers and negative controls verified.");
