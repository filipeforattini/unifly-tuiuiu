//! Unified binary crate for UniFi network management.
//!
//! Provides two feature-gated binaries:
//! - `unifly` (CLI) — kubectl-style command-line interface
//! - `unifly-tui` (TUI) — real-time terminal dashboard

pub mod config;

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "tui")]
pub mod tui;
