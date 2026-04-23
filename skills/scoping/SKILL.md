---
name: scoping
description: Use when a user asks what an agent key can do, how Shuriken permissions work, or which scopes they need for a given integration.
---

# Scoping an agent key

Every agent key carries a set of scopes. A scope is a capability grant — read tokens, execute trades, read positions, deliver notifications. The server enforces scopes on every tool call and API endpoint; a call outside the granted scope fails with a structured authorization error.

## Principles

- **Least privilege.** Grant only what the integration needs. A read-only analytics dashboard should never hold write scopes.
- **Separate read from write.** Read scopes are generally safe; write/execute scopes move money or send messages and deserve more scrutiny.
- **Per-integration, not per-user-role.** A scope is a capability, not a persona. Reason about what the integration *does*, not who the user is.

## Scope categories

Authoritative scope names live in the docs; these are the categories to reason about:

- **Read scopes** — querying token info, positions, wallet balances, alpha signals. Safe for analytics and display use cases.
- **Trading scopes** — planning trades, executing trades, cancelling orders. Required for any integration that moves funds.
- **Delivery scopes** — sending notifications (Telegram, email) on behalf of the user. Rate-limited.
- **Administrative scopes** — managing the user's agent keys themselves, linked wallets, etc. Typically not needed by third-party integrations.

Fetch `https://docs.shuriken.trade/llms.txt` for the current authoritative scope list and the exact names to pass when minting a key.

## Common scope combinations

- *Read-only analytics dashboard*: read scopes only.
- *Automated trading bot*: read scopes + trading scopes.
- *Notification/alert service*: read scopes + delivery scopes.
- *Full-service managed agent*: all non-administrative scopes, granted per-user at consent time.

## When a call fails on scope

The error response names the missing scope. Two valid responses:
1. If the scope is genuinely needed, guide the user to rotate the key with the added scope.
2. If the scope should not be needed (the call is read-only but erroring on a write scope), something in the integration is calling the wrong endpoint — investigate before granting.

## Pointers

- Platform documentation (including scope reference): fetch `https://docs.shuriken.trade/llms.txt` and search for "scopes"
- Related skills: `shuriken:agent-keys`, `shuriken:api-integration`
