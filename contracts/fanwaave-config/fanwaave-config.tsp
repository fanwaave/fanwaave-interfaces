import "@typespec/json-schema";
using TypeSpec.JsonSchema;

namespace Fanwaave.Config;

@id("FanwaaveMode")
enum FanwaaveMode {
  client: "client",
  server: "server",
  hybrid: "hybrid",
}

@id("EnvKind")
enum EnvKind {
  string: "string",
  bool: "bool",
  integer: "integer",
  double: "double",
  json: "json",
  url: "url",
}

@id("Flags2EnvPrecedence")
enum Flags2EnvPrecedence {
  argv_over_env: "argv-over-env",
}

@id("Flags2EnvConfig")
model Flags2EnvConfig {
  @minLength(1)
  @maxLength(256)
  contract: string;

  require_audit: boolean;
  precedence: Flags2EnvPrecedence;
}

@id("EnvBinding")
model EnvBinding {
  @minLength(1)
  @maxLength(64)
  @pattern("^[a-z][a-z0-9_]*$")
  name: string;

  @minLength(1)
  @maxLength(128)
  @pattern("^[A-Z_][A-Z0-9_]*$")
  key: string;

  kind: EnvKind;
  required: boolean;
  secret: boolean;

  @maxLength(8192)
  default?: string;

  @maxLength(512)
  description?: string;
}

@id("FanwaaveClientConfig")
model FanwaaveClientConfig {
  enabled: boolean;

  @maxLength(64)
  api_base_url_binding?: string;

  @maxLength(64)
  tenant_id_binding?: string;

  @maxLength(64)
  auth_token_binding?: string;
}

@id("FanwaaveServerConfig")
model FanwaaveServerConfig {
  enabled: boolean;

  @maxLength(64)
  bind_addr_binding?: string;

  @maxLength(64)
  database_url_binding?: string;

  @maxLength(64)
  push_provider_binding?: string;
}

@id("FanwaaveConfig")
model FanwaaveConfig {
  @minValue(1)
  @maxValue(1)
  version: int32;

  mode: FanwaaveMode;
  strict: boolean;
  flags2env: Flags2EnvConfig;
  env: EnvBinding[];
  client?: FanwaaveClientConfig;
  server?: FanwaaveServerConfig;
}
