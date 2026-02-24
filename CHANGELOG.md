# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.1.0] — 2026-02-23

### Added

- **CLI** with 22 resource commands: `devices`, `clients`, `networks`, `wifi`, `firewall`, `dns`, `vpn`, `acl`, `admin`, `alarms`, `dpi`, `events`, `hotspot`, `radius`, `sites`, `stats`, `system`, `traffic-lists`, `wans`, `countries`, `config`, `completions`
- **TUI** with 8 real-time screens: Dashboard, Devices, Clients, Networks, Firewall, Topology, Events, Stats
- **Dual-API engine** — Integration API (REST, API key auth) and Legacy API (session-based, cookie/CSRF) with automatic Hybrid negotiation
- **WebSocket event streaming** with 10K rolling buffer, severity filtering, pause/scroll-back
- **Area-fill traffic charts** with Braille line overlay, auto-scaling axes, and period selection (1h/24h/7d/30d)
- **Dashboard** — btop-style overview with WAN traffic chart, gateway info, connectivity health, CPU/MEM gauges, network/WiFi panels, top clients, recent events
- **Device management** — list, get, restart, locate (LED flash), adopt, forget; 5-tab detail panel in TUI (Overview, Performance, Radios, Clients, Ports)
- **Client management** — list with type filtering (All/Wireless/Wired/VPN/Guest), block/unblock, kick
- **Network management** — list, get, create, update, delete VLANs; inline edit overlay in TUI
- **Firewall management** — policies, zones, ACL rules across three sub-tabs with visual rule reordering
- **Zoomable topology view** — gateway-to-AP tree with pan, zoom, fit-to-view, color-coded by device type and state
- **Historical statistics** — WAN bandwidth, client counts, DPI application/category breakdown
- **Hotspot voucher management** — create, list, delete, revoke guest vouchers
- **DNS policy management** — local DNS record CRUD
- **VPN, RADIUS, WAN interface inspection** — read-only views for VPN servers/tunnels, RADIUS profiles, WAN interfaces
- **Multi-profile configuration** — named controller profiles with interactive setup wizard (`config init`)
- **5 output formats** — table, JSON, compact JSON, YAML, plain text (`-o` flag)
- **OS keyring credential storage** — API keys and passwords stored via system keyring with plaintext fallback
- **Environment variable support** — `UNIFI_API_KEY`, `UNIFI_URL`, `UNIFI_PROFILE`, `UNIFI_SITE`, `UNIFI_OUTPUT`, `UNIFI_INSECURE`, `UNIFI_TIMEOUT`
- **Shell completions** — Bash, Zsh, Fish via `completions` command
- **SilkCircuit theme** — neon-on-dark color palette with semantic highlighting and ANSI fallback
- **Published library crates** — `unifly-api` and `unifly-core` on crates.io for building custom UniFi tools
- **AI agent skill** — teaches coding assistants UniFi infrastructure management via the CLI
- **Cross-platform distribution** — Homebrew tap, shell/PowerShell installers, cargo install, GitHub releases for Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x86_64)

### Security

- TLS verification defaults to system CA store (self-signed certs require explicit `--insecure` flag)
- Config file permissions restricted to owner (0600) on Unix
- Credential storage via OS keyring with plaintext fallback only when keyring is unavailable

[Unreleased]: https://github.com/hyperb1iss/unifly/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/hyperb1iss/unifly/releases/tag/v0.1.0
