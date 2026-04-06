# unifly-ts

Primeira onda da migração do `unifly` para TypeScript com `tuiuiu.js`.

## Objetivo

Este app existe em paralelo ao workspace Rust para validar uma direção TUI-first:

- runtime e domínio em TypeScript;
- shell TUI em `tuiuiu.js`;
- foco inicial em observabilidade e workflows de operação;
- base pronta para conectar Integration API e Session API depois.

## Estado atual

- contratos centrais de domínio/runtime;
- store reativa com snapshots;
- `DemoController` com dados simulados e eventos ao vivo;
- dashboard e telas prioritárias navegáveis.

## Como rodar

```bash
cd apps/unifly-ts
pnpm install
pnpm start
```

Modo real, read-only por API key:

```bash
UNIFLY_TS_MODE=real \
UNIFI_CONTROLLER=https://192.168.1.1 \
UNIFI_SITE=default \
UNIFI_API_KEY=... \
pnpm start
```

Se o bootstrap em modo real falhar, o app cai de volta para demo e registra o erro.

## Atalhos

- `1-5`: alterna telas
- `r`: força refresh
- `d`: alterna modo demo pulse
- `q` ou `Esc`: sai

## Próximos passos

- plugar config/env real do UniFi;
- expandir os clientes HTTP reais além do read-only por API key;
- trocar partes da demo por dados normalizados da controller real;
- expandir workflows operacionais.
