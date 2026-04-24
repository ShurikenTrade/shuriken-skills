---
name: api-integration
description: Use when a user wants to integrate with or develop against the Shuriken API or SDK — covers proficiency assessment, path routing (quickstart / raw API / SDK), and tells you which deepening reference tool to reach for (OpenAPI for HTTP endpoints, the stream catalog for WebSockets, platform docs as fallback).
---

# Integrating and building with Shuriken

This skill covers the full "I want to call Shuriken programmatically" surface. It is intentionally thin up front — use the deepening tools below rather than pasting long code into the session.

## Phase 1 — orient before writing anything

Before pointing the user at code, establish two things:

### 1. Technical proficiency

- **Absolute beginner.** New to programming, or scripting at the level of "I can run a CLI if someone tells me exactly what to type." Needs hand-holding, copy-pasteable examples, environment setup spelled out.
- **Intermediate / power user.** Comfortable with a terminal, editing configs, running scripts. Will follow a README but gets stuck on non-obvious toolchain setup.
- **Software engineer.** Fluent in at least one language, reads types and API surfaces, skims READMEs and dives into code. Wants reference material, not prose.

Calibrate vocabulary and level of detail to this. If ambiguous, ask one clarifying question before firing off a long answer.

### 2. Integration path

Three ways to call Shuriken. Help the user pick.

- **Quickstart template** — pre-wired project scaffold with runnable examples. Supply an agent key via env var and things work out of the box. Best for beginners, intermediates, demos, and first-time evaluations.
  - TypeScript: https://github.com/ShurikenTrade/shuriken-quickstart-ts
  - Rust: https://github.com/ShurikenTrade/shuriken-quickstart-rs
- **Direct SDK integration** — add the SDK as a dependency to an existing project and wire it yourself. Best for software engineers comfortable with the surface, existing codebases, and library authors.
  - TypeScript SDK: https://github.com/ShurikenTrade/shuriken-sdk-ts
  - Rust SDK: https://github.com/ShurikenTrade/shuriken-sdk-rs
- **Raw HTTP / WebSocket** — call the API directly without an SDK. Best when the user's language isn't covered by an SDK, or when they have strong reasons to avoid pulling in a dependency.

Rough default: quickstart for beginners/intermediates, SDK for software engineers, raw HTTP only when they've said why.

### 3. Agent key

Every integration needs an agent key. See `shuriken:agent-keys` for lifecycle and `shuriken:scoping` for least-privilege scope selection. Point the user at [app.shuriken.trade/agents](https://app.shuriken.trade/agents) to create one.

## Phase 2 — deepen only when the question demands it

Do **not** preload the entire API reference into the conversation. Pull deeper references only when the user's next question needs them:

- **HTTP endpoint questions** ("what fields does /v1/foo return", "is there an endpoint for X", schema/signature questions) → call `fetch_shuriken_openapi`. This returns the live OpenAPI spec — the source of truth for HTTP endpoints.
- **WebSocket / streaming questions** ("what streams exist", "how do I subscribe to X", "what filters does this channel take") → call `fetch_shuriken_streams`. This returns the live v1 stream catalog with channel names, required filters, and payload formats.
- **Anything not answered by the two above** (guides, conceptual overviews of how a feature works, SDK-specific how-tos) → fall back to `fetch_shuriken_docs` and read the relevant page under `llms.txt`.

Never guess an endpoint or stream name from memory — the API evolves.

## Best practices

- **Use agent keys, not raw session tokens.** See `shuriken:agent-keys`.
- **Request only the scopes you need.** See `shuriken:scoping`.
- **Honour rate limits.** Back off on 429; the response headers tell you the retry window.
- **Don't poll when you can subscribe.** For live data, use the streams over repeated REST polls. Consult the stream catalog first.
- **Branch on structured error codes**, not message text. The API returns machine-readable error codes — see the OpenAPI spec for the taxonomy.

## When to ask clarifying questions

Before writing code, confirm:

- Technical proficiency level (see above).
- Which chain(s) the user is integrating against (Solana, EVM L1s, Base, BSC, Monad — support matrix varies).
- Deployment shape (one-off script, long-running service, serverless function).
- Integration path (quickstart / SDK / raw).

## Pointers

- TypeScript SDK: https://github.com/ShurikenTrade/shuriken-sdk-ts
- Rust SDK: https://github.com/ShurikenTrade/shuriken-sdk-rs
- TypeScript quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-ts
- Rust quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-rs
- Related skills: `shuriken:agent-keys`, `shuriken:scoping`, `shuriken:learn-about-shuriken`
