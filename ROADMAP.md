# Roadmap

Version-based milestones for unifly development. No fixed timelines — shipping when it's ready.

---

## v0.1.0 — Initial Release (Current)

The foundation. A fully functional CLI + TUI for UniFi network management.

**What ships:**

- **22 CLI commands** — devices, clients, networks, WiFi, firewall policies/zones/ACLs, DNS, VPN, hotspot vouchers, DPI, RADIUS, events, stats, system info, admin management, alarms, WAN interfaces, traffic lists, sites, countries, config, shell completions
- **8-screen TUI** — Dashboard, Devices, Clients, Networks, Firewall, Topology, Events, Stats
- **Dual-API architecture** — Integration API (REST + API key) and Legacy API (session + cookie/CSRF) with automatic Hybrid negotiation
- **Real-time WebSocket events** — live event streaming with 10K rolling buffer, severity filtering, pause/scroll-back
- **Keyring credential storage** — OS-native keyring for API keys and passwords, plaintext fallback
- **5 output formats** — table, JSON, compact JSON, YAML, plain text
- **Multi-profile configuration** — named controller profiles, interactive setup wizard, environment variable overrides
- **Published library crates** — `unifly-api` and `unifly-core` on crates.io for building custom UniFi tools
- **AI agent skill** — teaches coding assistants to manage UniFi infrastructure via the CLI
- **Cross-platform distribution** — Homebrew, shell/PowerShell installers, cargo install

**Known limitations:**

- WebSocket reconnect lifecycle is broken (CancellationToken permanent after first disconnect)
- Device radio data parsing not implemented (radios field always empty)
- No port forwarding or DHCP reservation management
- Integration API snapshot refresh clears non-legacy collections each cycle

---

## v0.2.0 — Network Essentials

Fill the gaps in day-to-day network management.

- Port forwarding CRUD (CLI + TUI)
- DHCP static reservation management
- Active DHCP lease listing
- Device radio data parsing from interfaces JSON
- VPN server/tunnel create, update, delete
- TUI WiFi screen (SSID management)
- Fix controller reconnect lifecycle (CancellationToken reset on disconnect)
- WebSocket TLS consistency for self-signed certificates

---

## v0.3.0 — Enterprise & Multi-Site

Scale to multi-site deployments and deeper infrastructure control.

- Site Manager API client (`api.ui.com`) — multi-site management, ISP metrics, SD-WAN
- Switch port configuration (PoE toggle, port profiles)
- Bulk operations (batch restart, mass firmware upgrade)
- TUI DNS screen
- Cross-site device and client aggregation views
- Configuration export/import

---

## v0.4.0 — Automation & Advanced

Power-user workflows and operational tooling.

- Configuration templates (save and apply across sites)
- Scheduled operations (firmware upgrades, maintenance windows)
- Interactive shell mode (REPL)
- Report generation (HTML/PDF network reports)
- Notification and webhook integration
- WAN failover management

---

## Future / Community-Driven

Ideas on the radar, shaped by community interest.

- Plugin system for custom commands
- Custom dashboard widgets in TUI
- Prometheus/Grafana metrics export
- Ansible/Terraform provider integration

---

Have a feature request? [Open an issue](https://github.com/hyperb1iss/unifly/issues) — community input drives the roadmap.
