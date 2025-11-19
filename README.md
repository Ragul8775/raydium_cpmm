# raydium_cpmm

A small Anchor-based Solana program stub for Raydium CPMM integration with a CPI to `raydium-cpmm-cpi`. This repo contains the on-chain program, tests, and deployment runbooks.

## Contents
- Program: [programs/raydium_cpmm/src/lib.rs](programs/raydium_cpmm/src/lib.rs) — main program with an `initialize` instruction ([`initialize`](programs/raydium_cpmm/src/lib.rs)).
- Instructions: [programs/raydium_cpmm/src/instructions/mod.rs](programs/raydium_cpmm/src/instructions/mod.rs) — CPI integration and `Initialize` accounts ([`Initialize`](programs/raydium_cpmm/src/instructions/mod.rs)).
- Constants: [programs/raydium_cpmm/src/constants.rs](programs/raydium_cpmm/src/constants.rs) — program seeds (`seeds`), fees (`fees`), limits (`limits`) and time constants (`time`).
- Tests: [tests/raydium_cpmm.ts](tests/raydium_cpmm.ts) — simple integration test harness using Anchor and TypeScript.
- Runbook / deployments: [runbooks/deployment/main.tx](runbooks/deployment/main.tx) — Surfpool runbook for deploying programs; see [runbooks/README.md](runbooks/README.md) and [txtx.yml](txtx.yml) for environment configuration.

## Prerequisites
- Rust and cargo
- Anchor CLI
- Node / Yarn (project uses package.json)
- Surfpool (optional) for local development infrastructure and runbooks
- Solana CLI (for keypairs)

## Build
- Anchor builds the BPF program:
  - anchor build
  - (Or) cargo build —workspace
- Program ID is in [programs/raydium_cpmm/src/lib.rs](programs/raydium_cpmm/src/lib.rs) and also referenced in [Anchor.toml](Anchor.toml) for localnet.

## Test
- Anchor Typescript test harness:
  - anchor test
  - or `yarn test` if you prefer Yarn (scripts configured via [Anchor.toml](Anchor.toml) and [package.json](package.json))
- See test harness: [tests/raydium_cpmm.ts](tests/raydium_cpmm.ts) — this calls the [`initialize`](programs/raydium_cpmm/src/lib.rs) instruction.

## Deploy
- Deploy with Surfpool (runbooks) — run `surfpool run deployment` or follow [runbooks/deployment/main.tx](runbooks/deployment/main.tx).
- Anchor deploy:
  - anchor deploy
- Runbooks reference deployment config and signers in [runbooks/deployment](runbooks/deployment), and environment in [txtx.yml](txtx.yml).

## Notes
- CPI integration to Raydium CPI is set up in [programs/raydium_cpmm/src/instructions/mod.rs](programs/raydium_cpmm/src/instructions/mod.rs) using `raydium_cpmm_cpi`.
- Program constants for fees and other limits live in [programs/raydium_cpmm/src/constants.rs](programs/raydium_cpmm/src/constants.rs).
- The sample tests use Anchor's provider: see [migrations/deploy.ts](migrations/deploy.ts) and [Anchor.toml](Anchor.toml).

## Useful references
- Anchor program entrypoint and instruction: [`initialize`](programs/raydium_cpmm/src/lib.rs)
- CPI accounts and Initialize accounts: [`Initialize`](programs/raydium_cpmm/src/instructions/mod.rs)
- Constants: [`seeds`, `fees`, `limits`, `time`](programs/raydium_cpmm/src/constants.rs)
- Runbook / Surfpool integration: [runbooks/deployment/main.tx](runbooks/deployment/main.tx) and [runbooks/README.md](runbooks/README.md)

License: see package.json (ISC).