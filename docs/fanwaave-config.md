# `.fanwaave-cfg.toml`

`.fanwaave-cfg.toml` is the Fanwaave domain runtime configuration contract. It is intentionally **not** a second command-line schema.

Executable argument handling remains owned by [`flags-2-env`](https://github.com/flags-2-env/flags-2-env) and the repository-root `.cli-flags.toml`. A Fanwaave executable must audit that CLI contract, parse argv through the official `flags-2-env` runtime binding, and pass only the normalized argv-derived environment overrides into Fanwaave configuration resolution.

The effective precedence is:

1. normalized argv overrides emitted by `flags-2-env`;
2. the ambient process environment / approved secret delivery boundary;
3. a non-secret `default` declared by `.fanwaave-cfg.toml`.

A `secret = true` binding may never declare `default`. Secret bindings are environment/secret-store only and must not be exposed as CLI flags. Missing required bindings, invalid scalar coercion, duplicate logical names, duplicate environment keys, unresolved client/server references, an unsafe flags contract path, disabled strict mode, or disabled flags audit are startup failures.

## Contract authorities

The parsed TOML object has two independent human-authored authorities:

- `contracts/fanwaave-config/fanwaave-config.tsp`
- `contracts/fanwaave-config/fanwaave-config.schema.json` (JSON Schema Draft 2020-12)

`ORESoftware/typespec-json-schema-validator` (TJSV) admits the two lanes fail-closed. Its generated TypeSpec JSON Schema witness and receipts are evidence only; neither generated output is an authored authority.

TJSV validates the canonical object after TOML decoding. TOML syntax, duplicate TOML keys, and the semantic invariants described above are enforced by the runtime parser/validator in `fanwaave-lib-core`.

## Shape

The root fields are:

- `version`: currently `1` only.
- `mode`: `client`, `server`, or `hybrid`.
- `strict`: must be `true` at runtime.
- `flags2env`: declares the `.cli-flags.toml` contract, mandatory audit, and `argv-over-env` precedence.
- `env`: zero or more named environment bindings.
- `client` / `server`: optional role-specific references to those named bindings.

Each `[[env]]` binding has a stable logical `name`, an uppercase environment `key`, a coercion `kind`, `required` and `secret` policy, and optional non-secret `default` / human description.

Supported kinds in v1 are `string`, `bool`, `integer`, `double`, `json`, and `url`.

See `examples/.fanwaave-cfg.toml` for a hybrid client/server example. The example contains no credential values; secret fields name environment keys only.

## Same repository, client and server

A repository containing both client and server code should use `mode = "hybrid"` and one shared binding inventory. Client and server sections reference the same logical bindings rather than maintaining two independent configuration dialects. Runtime entry points consume only the role-relevant resolved view.

A pure client or pure server uses the corresponding mode and must not enable the opposite role. This consistency check is semantic, so every language binding must implement the same fail-closed rule from the admitted contract evidence rather than silently guessing.
