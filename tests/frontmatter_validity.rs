//! Ensures every skill directory has a valid SKILL.md with required frontmatter.

use shuriken_skills::list;

#[test]
fn all_skills_have_name_and_description() {
    let skills = list();
    assert!(!skills.is_empty(), "no skills loaded");

    for s in skills {
        assert!(!s.name.is_empty(), "skill has empty name");
        assert!(
            !s.description.is_empty(),
            "skill {} has empty description",
            s.name
        );
    }
}

#[test]
fn description_embeds_use_when_guidance() {
    for s in list() {
        if s.name.starts_with("__") {
            continue; // skip test fixtures
        }
        let d = s.description.to_lowercase();
        assert!(
            d.contains("use when") || d.contains("when "),
            "skill {} description should embed 'use when' trigger: {}",
            s.name,
            s.description
        );
    }
}
