# Installing Shuriken skills for OpenCode

Add the plugin to the `plugin` array in your `opencode.json` (global at `~/.config/opencode/opencode.json` or per-project):

```json
{
  "plugin": ["shuriken-skills@git+https://github.com/ShurikenTrade/shuriken-skills.git"]
}
```

Restart OpenCode. The plugin registers the bundled `skills/` directory with OpenCode's skill loader.

Verify by asking: "What Shuriken skills are available?" — your agent should be able to list `agent-keys`, `scoping`, `api-integration`, and `learn-about-shuriken` via the native `skill` tool.

To pin a specific version:

```json
{
  "plugin": ["shuriken-skills@git+https://github.com/ShurikenTrade/shuriken-skills.git#v0.2.0"]
}
```
