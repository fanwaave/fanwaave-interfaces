// Fail-closed projection verifier for AttoLang adapters. Never replaces JSON Schema.
export function validate_atto_projection(projection, schema) {
  const reject = (reason) => { throw new Error(`AttoLang contract rejected: ${reason}`); };
  if (!projection || typeof projection !== "object" || Array.isArray(projection) ||
      projection.schema !== "fanwaave.atto-contract-projection/v1" ||
      projection.language !== "atto" || projection.admission !== "contract-only" ||
      projection.generated !== false ||
      projection.authoritativeSchema !== "schema/v1/notificationdispatch.json" ||
      projection.model !== "NotificationDispatch") reject("unknown projection authority");
  if (!schema || schema.type !== "object" || schema.title !== projection.model ||
      schema.additionalProperties !== false || projection.additionalProperties !== false) reject("model shape");
  const fields = projection.fields;
  if (!fields || typeof fields !== "object" || Array.isArray(fields) ||
      Object.keys(fields).sort().join(",") !== "id,payload,revision") reject("field set");
  const required = ["id", "revision", "payload"];
  if (!Array.isArray(projection.required) || projection.required.length !== required.length ||
      required.some((key) => !projection.required.includes(key)) ||
      !Array.isArray(schema.required) || schema.required.length !== required.length ||
      required.some((key) => !schema.required.includes(key))) reject("required field set");
  for (const key of required) {
    if (JSON.stringify(fields[key]) !== JSON.stringify(schema.properties?.[key])) reject(`field drift: ${key}`);
  }
  return true;
}
