# fanwaave-interfaces

Data-only contracts for Fan and push-notification delivery across web, mobile, and desktop. Schema: `schema/v1`. Generated TypeScript and Dart live under `generated/` and must stay types-only.

## Experimental AttoLang projection

`atto/notification-dispatch.atto` defines pure validation guards for the id-length and shape boundary of `NotificationDispatch`. `atto/notification-dispatch.v1.json` is a **contract-only, non-generated** projection of the authored JSON Schema at `schema/v1/notificationdispatch.json`. It intentionally does not claim to replace JSON Schema, TypeSpec, or the generated TypeScript/Dart contracts. The consuming adapter must count Unicode code points (not UTF-8 bytes), check exact `fanwaave-0002` revision and payload presence, and validate the full authored schema separately.

Run the exact-field parity and negative-case checks:

```sh
node --test tests/atto-contracts.test.mjs
```

These Node admission tests do not compile or execute `.atto` code. AttoLang compiler conformance and the downstream client transport are release prerequisites.
