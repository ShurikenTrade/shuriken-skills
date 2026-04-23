//! Shuriken skills — agent-consumable integration guidance.
//!
//! See individual skill bodies under `skills/<name>/SKILL.md`.

mod frontmatter;

use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;

static SKILLS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/skills");

pub const NAMESPACE: &str = "shuriken";

#[derive(Debug)]
pub struct Skill {
    pub name: &'static str,
    pub description: &'static str,
    pub body: &'static str,
}

impl Skill {
    pub fn qualified_name(&self) -> String {
        format!("{NAMESPACE}:{}", self.name)
    }
}

static SKILLS: Lazy<Vec<Skill>> = Lazy::new(load_skills);

fn load_skills() -> Vec<Skill> {
    let mut out = Vec::new();
    for entry in SKILLS_DIR.dirs() {
        // include_dir returns paths relative to the embedded root; look up SKILL.md inside each skill directory.
        let skill_md_path = format!("{}/SKILL.md", entry.path().display());
        let skill_md = match SKILLS_DIR.get_file(&skill_md_path) {
            Some(f) => f,
            None => continue, // directories without SKILL.md are ignored
        };

        let raw = skill_md
            .contents_utf8()
            .expect("SKILL.md must be valid UTF-8");

        let parsed = frontmatter::parse(raw)
            .unwrap_or_else(|e| panic!("invalid frontmatter in {}: {e}", skill_md.path().display()));

        // Leak the strings so the Skill struct can hold &'static str.
        // This runs once at process start; bounded by compile-time corpus size.
        out.push(Skill {
            name: Box::leak(parsed.frontmatter.name.into_boxed_str()),
            description: Box::leak(parsed.frontmatter.description.into_boxed_str()),
            body: Box::leak(parsed.body.to_string().into_boxed_str()),
        });
    }
    out.sort_by(|a, b| a.name.cmp(b.name));
    out
}

pub fn list() -> &'static [Skill] {
    &SKILLS
}

pub fn get(name: &str) -> Option<&'static Skill> {
    let local = name
        .strip_prefix(&format!("{NAMESPACE}:"))
        .unwrap_or(name);
    list().iter().find(|s| s.name == local)
}

pub fn render_index() -> String {
    let mut out = String::from("# Shuriken skills\n\n");
    out.push_str(
        "The following guidance skills are available. Call `get_skill(name)` to load the full body of any that match the user's question.\n\n",
    );
    for skill in list() {
        // Skip internal fixture skills in production rendering.
        if skill.name.starts_with("__") {
            continue;
        }
        out.push_str(&format!(
            "- `{}` — {}\n",
            skill.qualified_name(),
            skill.description,
        ));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_is_loaded() {
        let fixture = list().iter().find(|s| s.name == "__fixture__");
        assert!(fixture.is_some(), "fixture skill should be loaded");
    }

    #[test]
    fn qualified_name_prepends_namespace() {
        let skills = list();
        let any = skills.first().expect("at least one skill loaded");
        assert!(any.qualified_name().starts_with("shuriken:"));
    }

    #[test]
    fn get_by_local_name() {
        assert!(get("__fixture__").is_some());
    }

    #[test]
    fn get_by_qualified_name() {
        assert!(get("shuriken:__fixture__").is_some());
    }

    #[test]
    fn get_unknown_returns_none() {
        assert!(get("does-not-exist").is_none());
    }

    #[test]
    fn render_index_contains_qualified_names_and_skips_fixtures() {
        let rendered = render_index();
        assert!(rendered.contains("# Shuriken skills"));
        assert!(rendered.contains("get_skill"));
        // Fixture is filtered out in production rendering:
        assert!(
            !rendered.contains("__fixture__"),
            "fixture should not appear in render_index output"
        );
    }
}
