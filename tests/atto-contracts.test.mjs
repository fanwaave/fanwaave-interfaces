import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { validate_atto_projection } from "../scripts/check-atto-contracts.mjs";

const projection = JSON.parse(readFileSync(new URL("../atto/notification-dispatch.v1.json", import.meta.url)));
const schema = JSON.parse(readFileSync(new URL("../schema/v1/notificationdispatch.json", import.meta.url)));
const source = readFileSync(new URL("../atto/notification-dispatch.atto", import.meta.url), "utf8");

test("AttoLang projection mirrors authoritative NotificationDispatch exactly", () => {
  assert.equal(validate_atto_projection(projection, schema), true);
});
test("reject lost required field", () => {
  assert.throws(() => validate_atto_projection({...projection, required:["id","payload"]},schema), /required/);
});
test("reject weaker id constraints and incorrect revision", () => {
  assert.throws(() => validate_atto_projection({...projection, fields:{...projection.fields,id:{type:"string",minLength:1,maxLength:512}}}, schema), /field drift: id/);
  assert.throws(() => validate_atto_projection({...projection, fields:{...projection.fields,revision:{const:"fanwaave-0001"}}}, schema), /field drift: revision/);
});
test("reject payload redirect and unknown fields", () => {
  assert.throws(() => validate_atto_projection({...projection, fields:{...projection.fields,payload:{"$ref":"../../schemas/admin.schema.json"}}},schema), /field drift: payload/);
  assert.throws(() => validate_atto_projection({...projection,fields:{...projection.fields,admin:{type:"boolean"}}},schema), /field set/);
});
test("reject widened object and fabricated generated status", () => {
  assert.throws(() => validate_atto_projection({...projection,additionalProperties:true},schema), /model shape/);
  assert.throws(() => validate_atto_projection({...projection,generated:true},schema), /authority/);
});
test("AttoLang guards are pure, with exact length bounds and no direct transport", () => {
  assert.match(source, /pub fnc valid_dispatch_id_length\(int id_bytes\): bool/);
  assert.match(source, /if id_bytes < 8 then/);
  assert.match(source, /return id_bytes <= 512;/);
  assert.match(source, /pub fnc valid_dispatch_shape\(int id_bytes, bool revision_matches, bool payload_present\): bool/);
  assert.doesNotMatch(source, /\b(?:fnc main|http\.get|Authorization|send)\b/);
});
