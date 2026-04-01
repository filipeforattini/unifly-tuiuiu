//! Async Rust client and reactive data layer for UniFi controller APIs.
//!
//! This crate provides both the HTTP transport layer and the domain model
//! for communicating with UniFi Network controllers.
//!
//! ## Transport layer
//!
//! - **Integration API** ([`IntegrationClient`]) — RESTful OpenAPI-based interface
//!   authenticated via `X-API-KEY` header. Primary surface for CRUD operations on
//!   devices, clients, networks, firewall rules, and other managed entities.
//!
//! - **Legacy API** ([`LegacyClient`]) — Session/cookie-authenticated endpoints under
//!   `/api/s/{site}/`. Used for data not yet exposed by the Integration API: events,
//!   traffic stats, admin users, DPI data, system info, and real-time WebSocket events.
//!
//! Both clients share a common [`TransportConfig`] for reqwest-based HTTP transport
//! with configurable TLS ([`TlsMode`]: system CA, custom PEM, or danger-accept for
//! self-signed controllers) and timeout settings.
//!
//! ## Domain layer
//!
//! - **[`Controller`]** — Central facade managing the full lifecycle: authentication,
//!   background refresh, and command routing.
//!
//! - **[`DataStore`]** — Lock-free reactive storage built on `DashMap` + `watch` channels.
//!
//! - **[`EntityStream<T>`]** — Subscription handle for TUI reactive rendering.
//!
//! - **Domain model** ([`model`]) — Canonical types (`Device`, `Client`, `Network`,
//!   `FirewallPolicy`, `Event`, etc.) with [`EntityId`] supporting both UUID and
//!   string-based identifiers.

// ── Transport layer ──────────────────────────────────────────────
pub mod auth;
pub mod error;
pub mod integration;
pub mod legacy;
pub mod transport;
pub mod websocket;

// ── Domain layer (merged from unifly-core) ───────────────────────
pub mod command;
pub mod config;
pub mod controller;
pub mod convert;
pub mod core_error;
pub mod model;
pub mod store;
pub mod stream;

// ── Transport re-exports ─────────────────────────────────────────
pub use auth::{AuthStrategy, ControllerPlatform, Credentials};
pub use error::Error;
pub use integration::IntegrationClient;
pub use integration::types as integration_types;
pub use legacy::LegacyClient;
pub use legacy::models as legacy_models;
pub use transport::{TlsMode, TransportConfig};

// ── Domain re-exports ────────────────────────────────────────────
pub use command::requests::*;
pub use command::{Command, CommandResult};
pub use config::{AuthCredentials, ControllerConfig, TlsVerification};
pub use controller::{ConnectionState, Controller};
pub use core_error::CoreError;
pub use store::DataStore;
pub use stream::EntityStream;

pub use model::{
    AclRule, Admin, Alarm, Client, ClientType, Country, Device, DeviceState, DeviceType,
    DpiApplication, DpiCategory, EntityId, Event, EventCategory, EventSeverity, FirewallPolicy,
    FirewallZone, HealthSummary, MacAddress, NatPolicy, NatType, Network, RadiusProfile, Site,
    SysInfo, SystemInfo, TrafficMatchingList, VpnServer, VpnTunnel, WanInterface,
};
