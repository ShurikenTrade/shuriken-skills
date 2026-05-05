# shuriken-skills

Agent-consumable integration guidance for the [Shuriken](https://app.shuriken.trade) platform.

This repository packages guidance skills that help any LLM-backed agent reason about integrating with Shuriken: how to authenticate, how to scope permissions, how to call the API or SDK. It is consumed by Shuriken's internal NLP stack (via the Rust crate at the repo root) and by external coding/agentic assistants (via their native plugin loaders).

## What's in here

- `skills/` — one directory per skill, each containing a `SKILL.md` in the Claude Code format (`name`, `description` frontmatter + markdown body). External agents read this tree.
- `src/`, `Cargo.toml` — a thin Rust crate (`shuriken-skills`) that bakes the skill markdown into a binary at compile time. Consumed internally by `shuriken-api`.

## Installing for external agents

The repo ships native plugin/extension manifests for every major coding agent. Install via your agent's own plugin command — skills surface as `shuriken:<skill-name>` (e.g. `shuriken:api-integration`).

### Claude Code

```
/plugin marketplace add ShurikenTrade/shuriken-skills
/plugin install shuriken@shuriken
```

### OpenAI Codex CLI

```
codex plugin marketplace add ShurikenTrade/shuriken-skills
codex plugin install shuriken@shuriken-skills
```

### GitHub Copilot CLI

```
copilot plugin marketplace add ShurikenTrade/shuriken-skills
copilot plugin install shuriken@shuriken-skills
```

### Gemini CLI

```
gemini extensions install https://github.com/ShurikenTrade/shuriken-skills
```

### Cursor

Use Cursor's `/add-plugin` from the Agent chat and point at this repo, or install from the Cursor Marketplace once we are listed.

### OpenCode

Add to the `plugin` array in `opencode.json`:

```json
{
  "plugin": ["shuriken-skills@git+https://github.com/ShurikenTrade/shuriken-skills.git"]
}
```

### Other agents (AGENTS.md fallback)

Cursor (rules), Aider, Zed, Windsurf, Cline, and Roo Code auto-detect `AGENTS.md` at the repo root. Clone the repo into the consuming project (or a parent directory) and the agent will pick up the skill index automatically.

## Publishing (maintainers)

Each marketplace consumes the manifests already committed to this repo:

| Marketplace | Manifest | Publish path |
|---|---|---|
| Claude Code (third-party) | `.claude-plugin/plugin.json` + `.claude-plugin/marketplace.json` | Tag a release; users run `/plugin marketplace add ShurikenTrade/shuriken-skills`. |
| Claude Code (official directory) | same as above | Submit at <https://claude.ai/settings/plugins/submit> for human review. |
| Codex | `.codex-plugin/plugin.json` | Tag a release; users run `codex plugin marketplace add ShurikenTrade/shuriken-skills`. PR to community lists when desired. |
| Copilot CLI (third-party) | `.claude-plugin/marketplace.json` (Copilot reads this path too) | Tag a release; users run `copilot plugin marketplace add ShurikenTrade/shuriken-skills`. |
| Copilot CLI (default marketplace) | same | Open a PR to <https://github.com/github/awesome-copilot> to land in the marketplace shipped with Copilot CLI. |
| Gemini CLI | `gemini-extension.json` + `GEMINI.md` | Tag a release. Add the `gemini-cli-extension` GitHub topic to the repo so Google's crawler indexes it for <https://geminicli.com/extensions/browse/>. |
| Cursor | `.cursor-plugin/plugin.json` | Submit at <https://cursor.com/marketplace/publish> for review. |
| OpenCode | `package.json` + `.opencode/plugins/shuriken-skills.js` | No registry; users install by git URL. PR to <https://github.com/awesome-opencode/awesome-opencode> for discovery. |

Release procedure:

1. Bump `version` in `Cargo.toml`, `package.json`, `.claude-plugin/plugin.json`, `.claude-plugin/marketplace.json`, `.codex-plugin/plugin.json`, `.cursor-plugin/plugin.json`, and `gemini-extension.json` (keep them in lockstep).
2. `git tag vX.Y.Z && git push --tags`.
3. Cut a GitHub Release (Gemini CLI prefers release archives over full clones).
4. For first-time listings: submit the Anthropic and Cursor forms, and PR `awesome-copilot` and `awesome-opencode`.

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
