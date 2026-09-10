namespace Fanwaave.Config;

@doc("Runtime role supported by one Fanwaave repository or executable.")
enum FanwaaveMode {
  client: "client",
  server: "server",
  hybrid: "hybrid",
}

@doc("Canonical scalar coercion applied after an environment binding resolves.")
enum EnvKind {
  string: "string",
  bool: "bool",
  integer: "integer",
  double: "double",
  json: "json",
  url: "url",
}

@doc("flags-2-env remains the argv authority; argv-derived overrides win over ambient environment values.")
model Flags2EnvConfig {
  @minLength(1)
  @maxLength(256)
  contract: string;

  require_audit: true;
  precedence: "argv-over-env";
}

@doc("One named environment binding consumed by Fanwaave domain configuration.")
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

@doc("Client-side Fanwaave binding references. References are checked semantically after schema admission.")
model FanwaaveClientConfig {
  enabled: boolean;

  @maxLength(64)
  api_base_url_binding?: string;

  @maxLength(64)
  tenant_id_binding?: string;

  @maxLength(64)
  auth_token_binding?: string;
}

@doc("Server-side Fanwaave binding references. References are checked semantically after schema admission.")
model FanwaaveServerConfig {
  enabled: boolean;

  @maxLength(64)
  bind_addr_binding?: string;

  @maxLength(64)
  database_url_binding?: string;

  @maxLength(64)
  push_provider_binding?: string;
}

@doc("Canonical parsed form of repository-root .fanwaave-cfg.toml version 1.")
model FanwaaveConfig {
  version: 1;
  mode: FanwaaveMode;
  strict: true;
  flags2env: Flags2EnvConfig;
  env: EnvBinding[];
  client?: FanwaaveClientConfig;
  server?: FanwaaveServerConfig;
}
