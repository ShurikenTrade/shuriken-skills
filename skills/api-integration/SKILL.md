---
name: api-integration
description: Use when a user asks how to integrate with the Shuriken API or SDK, build an agent, or call the Shuriken platform programmatically. Covers proficiency assessment, choice between quickstart template and direct SDK integration, auth, error handling, and points to llms.txt and the OpenAPI spec for authoritative reference.
---

# Integrating with the Shuriken API

This skill tells you how to approach any question about programmatically integrating with Shuriken. You are the reasoning layer; the authoritative reference content lives in our docs.

## Approach

### 1. Establish technical proficiency before anything else

How you convey information depends on the user's programming and software-engineering background. Before writing code or pointing at docs, figure out roughly where they sit on this spectrum:

- **Absolute beginner.** New to programming, or scripting at the level of "I can run a CLI command if someone tells me exactly what to type." Needs hand-holding, concrete copy-pasteable examples, environment setup spelled out step by step.
- **Intermediate / power user.** Comfortable with a terminal, editing config files, running scripts in a language they know. Can follow a README but will get stuck on non-obvious toolchain setup. Wants working examples they can modify.
- **Software engineer.** Fluent in at least one language, comfortable reading types and API surfaces, will probably skim a README and dive into code. Wants reference material and precise signatures, not prose explanations.

Calibrate vocabulary, level of detail, and how much you show vs. tell to match this level. A software engineer does not need the "what is an environment variable" primer; an absolute beginner very much does. If the level is ambiguous, ask one clarifying question before proceeding — do not fire off a long answer against assumptions.

### 2. Route the user: quickstart vs. direct SDK integration

There are two integration paths. Help the user pick the right one.

**Quickstart template** — a pre-wired project scaffold that ships runnable examples. Once the user supplies an agent key via env var, the examples work out of the box. Best for:

- Users who want to play around quickly without assembling a project themselves.
- Users who need more hand-holding — the examples act as guided, working reference code.
- Demos, hackathons, internal tooling spikes, first-time evaluations.

Links:
- TypeScript quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-ts
- Rust quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-rs

**Direct SDK integration** — add the SDK as a dependency to an existing project and wire it yourself. Best for:

- Users who already understand the SDK / API surface and want full control over structure.
- Integrating into an existing codebase with its own conventions and tooling.
- Library authors building higher-level abstractions on top.

Links:
- TypeScript SDK: https://github.com/ShurikenTrade/shuriken-sdk-ts
- Rust SDK: https://github.com/ShurikenTrade/shuriken-sdk-rs

If the user hasn't indicated which path suits them, ask. Rough default: quickstart for absolute beginners and intermediates, direct SDK integration for software engineers who are already comfortable with the surface.

### 3. Point at the authoritative reference

Regardless of path, the source of truth for endpoints, schemas, and error shapes is our docs, not this skill. Fetch:

- `https://docs.shuriken.trade/llms.txt` for the current platform documentation index — concepts, REST API, SDKs, and guides (or `llms-full.txt` for the expanded version).
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

- Technical proficiency level (see above).
- Which chain(s) the user is integrating against (Solana, EVM L1s, Base, BSC, Monad — support matrix varies).
- Whether they're building for themselves or acting on behalf of other users (single-agent vs managed-agent patterns).
- What deployment shape they need (one-off script, long-running service, serverless function).
- Quickstart vs. direct SDK integration path.

## Pointers

- TypeScript SDK: https://github.com/ShurikenTrade/shuriken-sdk-ts
- Rust SDK: https://github.com/ShurikenTrade/shuriken-sdk-rs
- TypeScript quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-ts
- Rust quickstart: https://github.com/ShurikenTrade/shuriken-quickstart-rs
- Platform documentation (concepts, REST API, SDKs, guides): fetch `https://docs.shuriken.trade/llms.txt` (or `llms-full.txt` for the expanded version)
- OpenAPI spec: fetch `https://docs.shuriken.trade/api-reference/openapi.json`
- Related skills: `shuriken:agent-keys`, `shuriken:scoping`
