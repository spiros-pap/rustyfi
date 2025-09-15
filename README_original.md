# Rustyfi

Prototype DeFi protocol on Solana, built in Rust with Anchor.
Serves as an experimental foundation for token minting, swaps, and lending pools, inspired by Solend and Mango, but developer-focused.

---

## Table of Contents
- [Architecture](#architecture)
- [Tech Stack](#tech-stack)
- [Setup & Running Locally](#setup--running-locally)
- [Usage Guide](#usage-guide)
- [Future Extensions](#future-extensions)
- [License](#license)

---

## Architecture

### On-chain (Anchor + Rust))
- **Program:** `programs/rustyfi/src/lib.rs`
- **Core Features (Planned):**
  - Token minting and transfers.
  - Basic swap functionality.
  - Lending/borrowing pools.
  - Governance hooks for future modules.
- **Security:**
  - Account validation with PDAs
  - Anchor constraint macros for runtime safety
  - Modular instruction handlers for maintainability

### Off-chain (TypeScript + Mocha)
- Local simulation tests for all on-chain instructions.
- Automated validation of account states and balances.
- Anchor workspace configuration for smooth deployment.

---

## Tech Stack

**On-chain**
- Rust
- Anchor framework
- Solana Program Library (SPL)

**Off-chain**
- TypeScript
- Mocha testing framework
- Solana Web3.js
- Anchor CLI

**Dev Tools**
- Solana CLI
- Local Solana validator
- Rust toolchain

---

## Setup & Running Locally

### 1) Install dependencies
`rustup update`
`npm install -g @project-serum/anchor-cli`
`npm install`

### 2) Build the program
`anchor build`

### 3) Start a local validator
`solana-test-validator`

### 4) Deploy the program locally
`anchor deploy`

### 5) Run tests
`anchor test`

### Usage Guide

1.Build & Deploy:
  Use `anchor build` and `anchor deploy` to compile and push the program.

2.Run Tests:
  Use `anchor test` to simulate program execution and validate logic.

3.Modify Program Logic:
  Update `programs/rustyfi/src/lib.rs` and rerun the build + deploy steps.

### Future Extensions

**Core DeFi Modules**
- SPL token mint and transfer logic
- AMM-based swap functionality
- Lending and borrowing with interest accrual
- Basic governance module

**Security**
- PDA-based account ownership.
- On-chain audits and program constraints.

**Integrations**
- Serum DEX integration for swaps.
- Pyth or Switchboard price oracles.

**Deployment**
- Devnet deployment scripts.
- CI/CD pipeline with Anchor tests.
- CI/CD with tests & build.

### License
MIT


