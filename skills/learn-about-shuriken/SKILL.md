---
name: learn-about-shuriken
description: Use when a user asks conceptual questions about Shuriken — what it does, what features exist, how a feature works, or "what is X on Shuriken?" Does not cover programmatic integration (that's `shuriken:api-integration`).
---

# Learning about Shuriken

This skill is a thin pointer. The Shuriken feature surface evolves quickly — do not answer from cached knowledge about what features exist, how they work, or what's supported.

## Approach

1. **Fetch the current platform docs index first.** Call `fetch_shuriken_docs` (defaults to `llms.txt`). That index lists every concept, feature area, REST endpoint group, SDK, and guide available right now.
2. **Follow the links that match the user's question.** If the index references a specific page, fetch that page with `fetch_shuriken_docs` and a `path` argument.
3. **Only answer from content you just fetched.** Cached beliefs about feature X existing, or feature Y working a certain way, are unreliable and may be outdated.

## When this skill is the wrong choice

- User is writing code or asking about endpoints, request shapes, SDK usage → use `shuriken:api-integration` instead.
- User is asking about agent key creation / scope selection → use `shuriken:agent-keys` or `shuriken:scoping`.

## Pointers

- Platform documentation index: `fetch_shuriken_docs` (defaults to `llms.txt`) — lists concepts, features, REST API, SDKs, guides.
- Expanded single-document dump: `fetch_shuriken_docs` with `path: "llms-full.txt"` — only when the index doesn't point at a specific page and you need everything inline.
- Related skills: `shuriken:api-integration`, `shuriken:agent-keys`, `shuriken:scoping`.
