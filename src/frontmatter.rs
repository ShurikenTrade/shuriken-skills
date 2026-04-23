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

    // Find the end of the closing `---` delimiter and return the tail as a
    // borrowed slice of `raw`, so callers can hold &'static str when `raw`
    // is &'static.
    let body_offset = raw
        .find("---")
        .and_then(|_| raw[3..].find("---"))
        .map(|i| i + 6) // skip both delimiters
        .ok_or_else(|| "malformed frontmatter delimiters".to_string())?;

    let body = raw[body_offset..].trim_start_matches('\n');

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
}
