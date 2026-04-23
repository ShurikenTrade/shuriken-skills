---
name: api-integration
description: Use when a user asks how to integrate with the Shuriken API or SDK, build an agent, or call the Shuriken platform programmatically. Covers proficiency assessment, choice between quickstart template and naked SDK, auth, error handling, and points to llms.txt and the OpenAPI spec for authoritative reference.
---

# Integrating with the Shuriken API

This skill tells you how to approach any question about programmatically integrating with Shuriken. You are the reasoning layer; the authoritative reference content lives in our docs.

## Approach

### 1. Establish proficiency before anything else

How you convey information depends on where the user is coming from. Before writing code or pointing at docs, figure out:

- **Experience level.** Are they a first-time Shuriken user, a developer new to crypto tooling, or an experienced integrator already familiar with our platform? Adjust vocabulary and assumed context accordingly — a seasoned integrator does not need the "what is an agent key" primer; a newcomer very much does.
- **Language and stack.** TypeScript or Rust are the supported first-class SDKs. Python/Go/other — they will use raw HTTP against the REST API.
- **Crypto fluency.** Do they know what a wallet, RPC, slippage, chain ID are? If not, surface those concepts before deep integration details.

If any of this is ambiguous, ask one clarifying question before proceeding. Do not fire off a long answer against assumptions.

### 2. Route the user: quickstart vs. naked SDK

There are two integration paths. Help the user pick the right one.

**Quickstart template** — a pre-wired project scaffold that includes the SDK, env-var setup, auth wiring, a minimal example flow, and dev tooling. Best for:

- New users getting their first integration running end-to-end.
- Anyone who wants a working starting point instead of assembling the pieces themselves.
- Demos, hackathons, internal tooling spikes.

Links:
- TypeScript quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-ts
- Rust quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-rs

**Naked SDK** — add the SDK as a dependency to an existing project and wire it yourself. Best for:

- Integrating into an existing codebase with its own conventions.
- Users who already understand the SDK surface and want full control over structure.
- Library authors building higher-level abstractions on top.

Links:
- TypeScript SDK: https://github.com/ShurikenTrade/shuriken-sdk-ts
- Rust SDK: https://github.com/ShurikenTrade/shuriken-sdk-rs

If the user hasn't indicated which path suits them, ask. Default recommendation: quickstart for first-time users, naked SDK for everyone else.

### 3. Point at the authoritative reference

Regardless of path, the source of truth for endpoints, schemas, and error shapes is our docs, not this skill. Fetch:

- `https://docs.shuriken.trade/llms.txt` for current documentation (or `llms-full.txt` for the expanded version).
- `https://docs.shuriken.trade/api-reference/openapi.json` for exact endpoint signatures and schemas.

Never guess an endpoint from memory — the API evolves.

### 4. Surface error handling explicitly

The Shuriken API uses structured error responses with machine-readable codes. Don't hand-wave; point the user at the error taxonomy in the docs and show them how to branch on the code, not the message text.

## Best practices

- **Use agent keys, not raw session tokens.** Agent keys are scoped, revocable, and designed for programmatic use. See `shuriken:agent-keys`.
- **Request only the scopes you need.** Broader scopes are a liability. See `shuriken:scoping`.
- **Honour rate limits.** Back off on 429; the response headers tell you the retry window.
- **Don't poll when you can subscribe.** For live data (trades, signals), use the streaming endpoints over repeated REST polls.

## When to ask clarifying questions

Before writing code, confirm:

- Proficiency level (see above).
- Which chain(s) the user is integrating against (Solana, EVM L1s, Base, BSC, Monad — support matrix varies).
- Whether they're building for themselves or acting on behalf of other users (single-agent vs managed-agent patterns).
- What deployment shape they need (one-off script, long-running service, serverless function).
- Quickstart vs. naked SDK path.

## Pointers

- TypeScript SDK: https://github.com/ShurikenTrade/shuriken-sdk-ts
- Rust SDK: https://github.com/ShurikenTrade/shuriken-sdk-rs
- TypeScript quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-ts
- Rust quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-rs
- REST API reference: fetch `https://docs.shuriken.trade/llms.txt`
- OpenAPI spec: fetch `https://docs.shuriken.trade/api-reference/openapi.json`
- Related skills: `shuriken:agent-keys`, `shuriken:scoping`
