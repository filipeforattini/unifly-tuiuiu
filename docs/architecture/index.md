# Architecture

Unifly is a two-crate Rust workspace with a clean dependency chain.

## Crate Map

```mermaid
graph TD
    UNIFLY["unifly<br/><i>CLI + TUI binaries</i>"]
    API["unifly-api<br/><i>Library: transport, controller,<br/>data store, domain models</i>"]

    UNIFLY --> API
```

## Design Principles

### Thin Binaries, Fat Library

The `unifly` crate produces two binaries (`unifly` CLI and `unifly-tui` TUI) via feature flags. Both are thin shells — argument parsing and rendering only. All business logic lives in `unifly-api`, which provides the Controller lifecycle, DataStore, entity models, and API transport. Config and profile management lives in the `unifly` crate alongside the binaries.

### Reactive Data Store

The `DataStore` uses `DashMap` for lock-free concurrent reads and `tokio::watch` channels for reactive updates. The TUI subscribes to entity streams and re-renders when data changes — no polling needed within the app.

### Dual API Transparency

`unifly-api`'s `Controller` transparently routes requests to the correct API backend. Callers don't need to know whether a feature uses the Integration API or the Legacy API — the controller handles routing, authentication, and response normalization.

## Key Types

| Type | Purpose |
|---|---|
| `Controller` | Main entry point — wraps `Arc<ControllerInner>` for cheap cloning across async tasks |
| `DataStore` | Entity storage — `DashMap` + `watch` channels for lock-free reactive updates |
| `EntityStream<T>` | Reactive subscription — wraps `watch::Receiver` with `current()`/`changed()` API |
| `EntityId` | Dual-identity — `Uuid(Uuid)` or `Legacy(String)` for entities that exist in both APIs |
| `AuthCredentials` | Auth mode — `ApiKey`, `Credentials`, `Hybrid`, or `Cloud` variants |

## Next Steps

- [Crate Structure](/architecture/crates) — what each crate does
- [Data Flow](/architecture/data-flow) — how data moves through the system
- [API Surface](/architecture/api-surface) — Integration API vs Legacy API
