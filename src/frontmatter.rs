//! Parses YAML frontmatter from SKILL.md files.
//!
//! The returned [`Parsed`] holds the body as a borrowed slice of the input,
//! so callers can hold `&'static str` when the input is `&'static`. This
//! invariant is depended on by [`crate::load_skills`] which feeds in content
//! from `include_dir!`.

use gray_matter::{engine::YAML, Matter};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Frontmatter {
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq)]
pub struct Parsed<'a> {
    pub frontmatter: Frontmatter,
    pub body: &'a str,
}

pub fn parse(raw: &str) -> Result<Parsed<'_>, String> {
    let matter = Matter::<YAML>::new();
    let parsed = matter.parse(raw);

    let data = parsed
        .data
        .ok_or_else(|| "missing frontmatter block".to_string())?;

    let frontmatter: Frontmatter = data
        .deserialize()
        .map_err(|e| format!("invalid frontmatter: {e}"))?;

    if !raw.starts_with("---") {
        return Err("missing frontmatter block".to_string());
    }

    // gray_matter has already validated the frontmatter block; the closing `---`
    // must be present for `data` to have been Some. Locate the end of the
    // closing delimiter to return the body as a borrowed slice of `raw`.
    let after_opener = &raw[3..];
    let closing_offset = after_opener
        .find("---")
        .expect("closing --- must exist if gray_matter parsed frontmatter");
    let body_offset = 3 + closing_offset + 3;
    let body = raw[body_offset..].trim_start_matches(&['\r', '\n'][..]);

    Ok(Parsed { frontmatter, body })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "---\nname: foo\ndescription: hello world\n---\n\n# body\n\ntext\n";

    #[test]
    fn parses_name_and_description() {
        let parsed = parse(SAMPLE).unwrap();
        assert_eq!(parsed.frontmatter.name, "foo");
        assert_eq!(parsed.frontmatter.description, "hello world");
    }

    #[test]
    fn body_is_everything_after_frontmatter() {
        let parsed = parse(SAMPLE).unwrap();
        assert!(parsed.body.contains("# body"));
        assert!(parsed.body.contains("text"));
    }

    #[test]
    fn missing_required_field_is_error() {
        let raw = "---\nname: foo\n---\n\nbody\n";
        assert!(parse(raw).is_err());
    }

    #[test]
    fn no_frontmatter_is_error() {
        let raw = "just a body with no frontmatter";
        assert!(parse(raw).is_err());
    }

    #[test]
    fn body_lifetime_is_tied_to_input() {
        const STATIC_INPUT: &str = "---\nname: foo\ndescription: bar\n---\n\nbody\n";
        let parsed = parse(STATIC_INPUT).unwrap();
        // This compile-time check locks in the invariant that body borrows
        // from `raw`: if parse ever starts returning an owned String, this
        // will fail to compile because you can't coerce String to &'static str.
        let _static_body: &'static str = parsed.body;
    }
}
