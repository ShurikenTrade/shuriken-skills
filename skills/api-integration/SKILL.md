---
name: api-integration
description: Use when a user asks how to integrate with the Shuriken API or SDK, build an agent, or call the Shuriken platform programmatically. Covers auth, request shape, error handling, and points to llms.txt and the OpenAPI spec for authoritative reference.
---

# Integrating with the Shuriken API

This skill tells you how to approach any question about programmatically integrating with Shuriken. You are the reasoning layer; the authoritative reference content lives in our docs.

## Approach

1. **Start from the user's intent.** Are they building a read-only dashboard, an automated trading agent, a webhook consumer, or something else? The answer determines which scopes and SDK surface they need — route to `shuriken:agent-keys` and `shuriken:scoping` if auth/permissions come up.
2. **Point at the authoritative reference.** Fetch `https://docs.shuriken.trade/llms.txt` (or `llms-full.txt` for the expanded version) for current documentation. For exact endpoint signatures and schemas, fetch `https://docs.shuriken.trade/api-reference/openapi.json`. Never guess an endpoint from memory — the API evolves.
3. **Prefer the SDK over raw HTTP.** Shuriken ships typed SDKs (Rust, TypeScript) that handle auth, retries, and error shapes. Raw HTTP is valid for scripting and unsupported languages; call it out as the fallback path, not the default.
4. **Surface error handling explicitly.** The Shuriken API uses structured error responses with machine-readable codes. Don't hand-wave; point the user at the error taxonomy in the docs and show them how to branch on the code, not the message text.

## Best practices

- **Use agent keys, not raw session tokens.** Agent keys are scoped, revocable, and designed for programmatic use. See `shuriken:agent-keys`.
- **Request only the scopes you need.** Broader scopes are a liability. See `shuriken:scoping`.
- **Honour rate limits.** Back off on 429; the response headers tell you the retry window.
- **Don't poll when you can subscribe.** For live data (trades, signals), use the streaming endpoints over repeated REST polls.

## When to ask clarifying questions

Before writing code, confirm:
- Which chain(s) the user is integrating against (Solana, EVM L1s, Base, BSC, Monad — support matrix varies).
- Whether they're building for themselves or acting on behalf of other users (single-agent vs managed-agent patterns).
- What deployment shape they need (one-off script, long-running service, serverless function).

## Pointers

- REST API reference: fetch `https://docs.shuriken.trade/llms.txt`
- OpenAPI spec: fetch `https://docs.shuriken.trade/api-reference/openapi.json`
- Related skills: `shuriken:agent-keys`, `shuriken:scoping`
