# unifly-tuiuiu

Estudo de viabilidade para reconstruir a experiência terminal do `unifly` com `TypeScript + tuiuiu.js`, mantendo a implementação original em Rust como baseline técnico e funcional.

## O que este fork é

Este repositório não é mais uma distribuição geral do projeto original.

Ele existe para responder uma pergunta bem específica:

> uma arquitetura em `JS/TS` com `tuiuiu.js` consegue entregar uma experiência de terminal mais poderosa, mais iterável e mais expressiva do que a implementação atual em Rust?

Por isso o repositório agora fica reduzido a dois blocos:

- `crates/`
  código Rust original, preservado como referência e baseline
- `apps/unifly-ts/`
  nova implementação experimental em TypeScript com `tuiuiu.js`

Todo o resto foi removido para deixar o estudo direto, legível e sem ruído.

## Estrutura

```text
.
├── apps/
│   └── unifly-ts/      # estudo TUI-first em TypeScript + tuiuiu.js
├── crates/
│   ├── unifly/         # app original em Rust
│   └── unifly-api/     # core original em Rust
├── Cargo.toml
└── README.md
```

## Como rodar

### 1. Rodar o estudo em TypeScript

Requisitos:

- `node >= 20`
- `pnpm`

```bash
cd apps/unifly-ts
pnpm install
pnpm start
```

Atalhos da TUI:

- `1-5` troca de tela
- `r` força refresh
- `d` alterna o pulse do modo demo
- `q` ou `Esc` sai

### 2. Rodar o baseline em Rust

Requisitos:

- `rustup`
- toolchain compatível com o workspace
- `just`

```bash
just tui
```

ou:

```bash
just cli devices list
```

## Modos do `unifly-ts`

### Demo

Usa dados sintéticos com comportamento UniFi-like para iterar rapidamente a UX e a arquitetura do runtime.

```bash
cd apps/unifly-ts
pnpm start
```

### Real

Usa leitura real via UniFi API com `X-API-KEY`, Integration API + Session HTTP read-only, e cai para demo se o bootstrap falhar.

```bash
cd apps/unifly-ts
UNIFLY_TS_MODE=real \
UNIFI_CONTROLLER=https://192.168.1.1 \
UNIFI_SITE=default \
UNIFI_API_KEY=... \
pnpm start
```

## O que já foi implementado no `unifly-ts`

- scaffold TS isolado do workspace Rust
- contratos explícitos de domínio e runtime
- `DataStore` reativa em TS
- `DemoController` para exploração de UX
- `RealController` inicial para leitura real
- clientes HTTP separados para Integration API e Session API
- normalização de snapshot para alimentar uma TUI única
- TUI navegável com dashboard, devices, clients, networks e events
- fallback explícito de live mode para demo
- metadados de runtime visíveis na interface

## Comparação prática: implementação TS vs Rust

### 1. Estrutura de runtime

Rust:

- o baseline original concentra a orquestração em `Controller`, `DataStore`, refresh loop, subscriptions e transporte async
- excelente para segurança de tipos, previsibilidade e performance
- mais custoso para iterar rapidamente na camada visual e nos fluxos de interação

TS + `tuiuiu.js`:

- separamos `domain`, `runtime`, `transport` e `ui` desde o começo
- a TUI consome snapshots e sinais, sem vazar detalhe de transporte para a camada visual
- a iteração na UX é mais rápida porque o ciclo de mudança e teste é muito menor

### 2. Construção da interface

Rust:

- a TUI original em Ratatui é sólida, mas a composição costuma ser mais estrutural e menos fluida para experimentar layouts e interações novas
- mudar o desenho de telas ou overlays tende a exigir mais esforço de plumbing

TS + `tuiuiu.js`:

- a UI fica muito mais direta de ler e montar
- o custo de experimentar painéis, estados, navegação e visualização é menor
- a prova atual já mostra runtime/status/source/erro live com pouca cerimônia

### 3. Estratégia de integração

Rust:

- já cobre muito mais superfície funcional
- tem o domínio UniFi consolidado e mais profundo
- continua sendo a fonte de verdade para endpoint shape, auth modes e quirks

TS + `tuiuiu.js`:

- ainda está atrás em cobertura
- usa o código Rust como mapa de contratos e comportamento
- está sendo construído primeiro para provar superioridade de experiência, não paridade total de features

### 4. Onde o TS está melhorando o experimento

- bootstrap mais fácil para demo e para live read-only
- separação explícita entre modo demo, live e fallback
- TUI mais honesta sobre estado real da conexão
- base mais favorável para explorar workflows mais densos e interativos

### 5. Onde Rust ainda leva vantagem hoje

- cobertura funcional real
- maturidade da integração UniFi
- robustez geral do core existente

## Como avaliar se o estudo está funcionando

O estudo é bem-sucedido se o `unifly-ts` provar pelo menos estes pontos:

- a arquitetura JS/TS não degrada a clareza do domínio
- a TUI fica mais rápida de evoluir do que no baseline Rust
- a UX final consegue ficar mais expressiva e mais poderosa
- o custo de manter dual-API coverage em TS continua aceitável

## Próximos passos

- aumentar a cobertura do `RealController`
- aprofundar merge entre Integration e Session
- expandir workflows operacionais além de observabilidade
- comparar lado a lado os mesmos cenários de uso entre Rust e `tuiuiu.js`

