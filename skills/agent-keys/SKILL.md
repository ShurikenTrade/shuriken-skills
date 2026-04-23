---
name: agent-keys
description: Use when a user asks how to authenticate as an agent, create or rotate API credentials, or understand the lifecycle of agent keys on Shuriken.
---

# Using agent keys

Agent keys are Shuriken's credential primitive for programmatic access. They authenticate a caller, carry a set of scopes defining what the caller can do, and are revocable without affecting the owning user's session.

## Approach

1. **One key per integration, not per user action.** An agent key represents a long-lived integration. Do not create a key per request.
2. **Grant the minimum scope.** See `shuriken:scoping` for how scopes are structured and how to reason about least-privilege.
3. **Treat keys as secrets.** Store them in environment variables or secret managers. Never hard-code. Never log. Never commit.

## Key lifecycle

- **Create** at [app.shuriken.trade/agents](https://app.shuriken.trade/agents) (the authenticated user's agent-key management page), or via the agent-key management API once bootstrapped. The key is displayed once at creation — capture it immediately.
- **Use** the same agent key across every surface:
  - REST — pass the key in the `Authorization: Bearer <key>` header on every request.
  - WebSocket — authenticate the connection with the same agent key (subject to the key's scopes, just like REST). The streaming endpoints share the same credential model; there is no separate websocket token.
  - SDKs — both the TypeScript and Rust SDKs accept the agent key directly in the client constructor. Prefer the SDK path when the user's language is supported; it handles header/connection wiring for you.
- **Rotate** periodically and after any suspected compromise. Rotation means: create the new key, deploy it to the consuming service, then revoke the old key. Not the reverse.
- **Revoke** when an integration is retired, a contractor departs, or the key is exposed. Revocation is immediate.

## One user, one integration

An agent key belongs to a single Shuriken user. An integration holds that user's key (or the few keys that user has created for different purposes). There is no multi-tenant flow where an app mints keys on behalf of many users — every key is owned by the Shuriken account that created it.

## Pointers

- Platform documentation (including key management): fetch `https://docs.shuriken.trade/llms.txt` and search for "agent keys"
- OpenAPI paths: `/v1/agent-keys/*` — fetch `https://docs.shuriken.trade/api-reference/openapi.json` for current signatures
- Related skills: `shuriken:scoping`, `shuriken:api-integration`
