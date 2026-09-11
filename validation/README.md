# Public validation contracts

This directory contains only contracts that may cross a trust boundary and be shipped to browser, mobile, desktop, CLI, and server consumers.

`RequestMeta`, `PageQuery`, and `ProblemDetails` are independently authored in JSON Schema and TypeSpec. Those sources are peer, top-level authorities. Neither file is generated from or allowed to overwrite the other. Any semantic discrepancy is a stop-and-evaluate release failure.

The matching `fanwaave-lib-core` repository implements these public contracts natively with Zod, Garde, `go-playground/validator/v10`, and Gleam decoders. It also owns separate private server packages. Names such as `TrustedActor`, `ServerRequestContext`, and `InternalCommand` must never appear in this public package or in generated client artifacts.

`route-bindings.v1.json` binds public request, response, and error schemas to stable operation IDs from `ORESoftware/api-docs`. The array starts empty rather than guessing operation IDs. Each binding must be added in the same reviewed change that updates the digest-bound API-docs route bundle.
