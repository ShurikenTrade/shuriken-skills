# shuriken-skills

Agent-consumable integration guidance for the [Shuriken](https://shuriken.trade) platform.

This repository packages guidance skills that help any LLM-backed agent reason about integrating with Shuriken: how to authenticate, how to scope permissions, how to call the API or SDK. It is consumed by Shuriken's internal NLP stack (via the Rust crate at the repo root) and by external coding/agentic assistants (via their native plugin loaders).

## What's in here

- `skills/` — one directory per skill, each containing a `SKILL.md` in the Claude Code format (`name`, `description` frontmatter + markdown body). External agents read this tree.
- `src/`, `Cargo.toml` — a thin Rust crate (`shuriken-skills`) that bakes the skill markdown into a binary at compile time. Consumed internally by `shuriken-api`.

## Installing for external agents

### Claude Code

Clone the repo into Claude Code's plugin directory:

```bash
mkdir -p ~/.claude/plugins
git clone https://github.com/ShurikenTrade/shuriken-skills ~/.claude/plugins/shuriken-skills
```

Claude Code auto-discovers skills from plugin directories and namespaces them by plugin ID, so you will see skills surfaced as `shuriken:api-integration`, `shuriken:agent-keys`, etc.

### Other agents (Phase 2)

Support for Codex, Gemini CLI, Hermes, and other agents is planned. Until then, any agent that can read `SKILL.md`-style markdown from a directory should be able to consume `skills/` by pointing at it directly.

## Skills included (v0.1)

- **`shuriken:api-integration`** — How to integrate with the Shuriken API or SDK.
- **`shuriken:agent-keys`** — How to authenticate as an agent.
- **`shuriken:scoping`** — How to reason about agent-key scopes and least-privilege.

## Using the Rust crate

```toml
[dependencies]
shuriken-skills = { git = "https://github.com/ShurikenTrade/shuriken-skills", tag = "v0.1.0" }
```

```rust
use shuriken_skills::{list, get, render_index};

// List all skills
for skill in list() {
    println!("{}: {}", skill.qualified_name(), skill.description);
}

// Fetch a skill body
let skill = get("shuriken:api-integration").expect("skill exists");
println!("{}", skill.body);

// Render the index block for a system prompt
println!("{}", render_index());
```

## License

MIT. See `LICENSE`.
