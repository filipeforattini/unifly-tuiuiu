# unifly-ts

First implementation wave of the `unifly` TypeScript rebuild using `tuiuiu.js`.

## Goal

This app exists next to the Rust workspace to validate a TUI-first direction:

- runtime and domain modeled in TypeScript
- terminal UI built with `tuiuiu.js`
- initial focus on observability and operator workflows
- architecture ready to absorb Integration API and Session API behavior

## Current State

- core domain and runtime contracts
- reactive store with snapshots
- `DemoController` with synthetic live behavior
- initial `RealController` for live read-only bootstrapping
- dashboard and priority views already navigable

## Run

```bash
cd apps/unifly-ts
pnpm install
pnpm start
```

Real read-only mode:

```bash
UNIFLY_TS_MODE=real \
UNIFI_CONTROLLER=https://192.168.1.1 \
UNIFI_SITE=default \
UNIFI_API_KEY=... \
pnpm start
```

If real bootstrap fails, the app falls back to demo mode and exposes the failure in the runtime panel.

## Shortcuts

- `1-5` switch screens
- `r` force refresh
- `d` toggle demo pulse
- `q` or `Esc` quit
