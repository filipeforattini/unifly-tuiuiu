# unifly-tuiuiu

This fork is a focused **viability study** for rebuilding the `unifly` terminal experience with **TypeScript + `tuiuiu.js`**, while keeping the original Rust codebase as the baseline.

## What This Repository Is

This is no longer a general-purpose distribution repo for the original project.

It exists to answer one engineering question:

> Can a `tuiuiu.js`-based TUI architecture deliver a more powerful, more iterable, and more expressive terminal experience than the current Rust implementation?

The repository is intentionally reduced to two active areas:

- `crates/`
  the original Rust implementation, preserved as the baseline
- `apps/unifly-ts/`
  the TypeScript/TUIUIU experimental implementation

Everything else was removed to keep the repo centered on the study.

## Repository Layout

```text
.
├── apps/
│   └── unifly-ts/      # TUI-first study in TypeScript + tuiuiu.js
├── crates/
│   ├── unifly/         # original Rust app
│   └── unifly-api/     # original Rust API/runtime core
├── Cargo.toml
└── README.md
```

## How To Run

### 1. Clone the repository

```bash
git clone git@github.com:filipeforattini/unifly-tuiuiu.git
cd unifly-tuiuiu
```

### 2. Run the TypeScript / `tuiuiu.js` study

Requirements:

- `node >= 20`
- `pnpm`

From the repository root:

```bash
cd apps/unifly-ts
pnpm install
pnpm start
```

What this does:

- installs the local TS study dependencies
- starts the `unifly-ts` TUI
- launches the study in demo mode by default

TUI shortcuts:

- `1-5` switch screens
- `r` force refresh
- `d` toggle demo pulse
- `q` or `Esc` quit

### 3. Run the TypeScript app against a real controller

This mode uses live UniFi reads through Integration API + Session HTTP with `X-API-KEY`.

From `apps/unifly-ts`:

```bash
UNIFLY_TS_MODE=real \
UNIFI_CONTROLLER=https://192.168.1.1 \
UNIFI_SITE=default \
UNIFI_API_KEY=... \
pnpm start
```

If bootstrap fails, the app falls back to demo mode while keeping the failure visible in the UI.

### 4. Run the Rust baseline

Requirements:

- `rustup`
- compatible Rust toolchain for the workspace
- `just`

From the repository root, start the original Rust TUI:

```bash
just tui
```

Run the original Rust CLI instead:

```bash
just cli devices list
```

Run the Rust test/check flow:

```bash
just check
```

In practice:

- `just tui` runs the original Rust terminal application
- `just cli ...` runs the original Rust CLI entrypoint
- `just check` runs the Rust workspace validation flow

## What Is Already Implemented In `unifly-ts`

- isolated TS workspace under `apps/unifly-ts`
- explicit domain and runtime contracts
- reactive `DataStore`
- `DemoController` for UX iteration
- initial `RealController` for live read-only mode
- separate Integration API and Session API HTTP clients
- normalized runtime snapshot feeding a single TUI
- navigable TUI with dashboard, devices, clients, networks, and events
- explicit demo/live/fallback runtime metadata visible inside the UI

## Practical Implementation Comparison

The point of this fork is not abstract discussion. It is implementation comparison.

### 1. App bootstrap

Rust baseline:

[`crates/unifly/src/tui/mod.rs`](/home/cyber/Work/FF/unifly-tuiuiu/crates/unifly/src/tui/mod.rs)

```rust
pub async fn launch(global: &GlobalOpts, args: TuiArgs) -> Result<()> {
    terminal::install_hooks()?;

    let _log_guard = setup_tracing(global.verbose, &args.log_file);

    let config_theme = config::load_config().ok().and_then(|c| c.defaults.theme);
    let theme_name = args.theme.as_deref().or(config_theme.as_deref());
    theme::initialize(theme_name);

    let controller = build_controller_direct(global)
        .or_else(|| build_controller_from_config(global.profile.as_deref()));

    let mut app = app::App::new(controller);
    app.run().await?;

    Ok(())
}
```

`tuiuiu.js` study:

[`apps/unifly-ts/src/ui/app.ts`](/home/cyber/Work/FF/unifly-tuiuiu/apps/unifly-ts/src/ui/app.ts)

```ts
export async function launchApp(controller: Controller): Promise<void> {
  setTheme(darkTheme);
  await controller.connect();

  const { waitUntilExit } = render(() => App({ controller }), { fullHeight: true });
  await waitUntilExit();
  await controller.disconnect();
}
```

Practical difference:

- the Rust entrypoint handles more boot concerns in one place: hooks, tracing, config, theme, controller construction, app lifecycle
- the TS version is thinner because controller construction and runtime mode selection were pushed out of the UI layer
- the TS shape is easier to recompose quickly while the Rust version is currently more operationally mature

### 2. Reactive UI wiring

Rust baseline:

- the architecture is split across `Controller`, `DataStore`, `data_bridge`, `App`, screens, and screen state
- this is robust, but moving data from transport to render tends to involve more plumbing across modules

`tuiuiu.js` study:

[`apps/unifly-ts/src/ui/app.ts`](/home/cyber/Work/FF/unifly-tuiuiu/apps/unifly-ts/src/ui/app.ts)

```ts
const [snapshot, setSnapshot] = useState<ControllerSnapshot>(props.controller.store.current());

useEffect(() => props.controller.store.subscribe((nextSnapshot) => setSnapshot(nextSnapshot)));
```

Practical difference:

- the TS version makes the store-to-view wiring extremely obvious
- the Rust version gives you stronger compile-time guarantees, but the path from runtime state to screen rendering is less lightweight to iterate on

### 3. Top-level screen composition

Rust baseline:

- composition is spread across the TUI app runtime, screen traits, and Ratatui render code
- powerful, but more structural and lower-level

`tuiuiu.js` study:

[`apps/unifly-ts/src/ui/app.ts`](/home/cyber/Work/FF/unifly-tuiuiu/apps/unifly-ts/src/ui/app.ts)

```ts
return Screen(
  {},
  Header(...),
  Main(
    { padding: 1 },
    Box(
      { flexDirection: 'row', gap: 1, height: 'fill' },
      NavigationPanel(screen()),
      ContentPanel(current, screen()),
    ),
  ),
  Footer(...),
);
```

Practical difference:

- the `tuiuiu.js` version reads much closer to the final layout
- the Rust version exposes more of the rendering mechanics
- for layout experimentation, the TS version is materially faster to reshape

### 4. Runtime transparency inside the UI

`tuiuiu.js` study:

[`apps/unifly-ts/src/ui/app.ts`](/home/cyber/Work/FF/unifly-tuiuiu/apps/unifly-ts/src/ui/app.ts)

```ts
Panel(
  { title: 'Runtime', height: 10 },
  Box(
    { flexDirection: 'column', gap: 1 },
    Text({}, `Mode: ${snapshot.runtime.appMode}`),
    Text({}, `Source: ${snapshot.runtime.dataSource}`),
    Text({}, `Controller: ${snapshot.runtime.controllerUrl}`),
    Text({}, `Site: ${snapshot.runtime.site}`),
    Text(
      { color: snapshot.runtime.lastError ? 'error' : 'mutedForeground' },
      snapshot.runtime.lastError ?? snapshot.runtime.statusMessage,
    ),
  ),
)
```

Practical difference:

- this study deliberately surfaces whether the app is running on demo data, live data, or fallback-demo data
- that makes the experiment honest: the UI should never “look live” when it is not
- this kind of instrumentation was cheap to add in the TS version because the runtime snapshot shape is simple and directly consumed by the view

## Current Assessment

### Where Rust is still stronger

- broader real feature coverage
- deeper UniFi domain handling
- more mature runtime and operational behavior

### Where `tuiuiu.js` is already proving useful

- faster layout iteration
- simpler screen composition
- easier runtime-state visibility in the UI
- lower friction for trying new interaction models

## Success Criteria For The Study

This study is successful if `unifly-ts` proves that:

- JS/TS can preserve domain clarity without collapsing into weakly-typed glue
- the TUI can evolve faster than the Rust baseline
- the resulting interface can become more expressive than the current Ratatui version
- dual-API UniFi support remains manageable in a TS runtime
