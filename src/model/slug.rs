use std::str::FromStr;

#[derive(Debug, thiserror::Error)]
pub enum SlugError {
    #[error("slug must not be empty")]
    Empty,
    #[error("slug must be lowercase letters, digits, and hyphens only (e.g. todoke-toctou)")]
    InvalidChar,
    #[error("slug must not start or end with a hyphen")]
    EdgeHyphen,
}

#[derive(Clone, Debug)]
pub struct Slug(String);

impl FromStr for Slug {
    type Err = SlugError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(SlugError::Empty);
        }
        if s.starts_with('-') || s.ends_with('-') {
            return Err(SlugError::EdgeHyphen);
        }
        if !s
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(SlugError::InvalidChar);
        }
        Ok(Slug(s.to_string()))
    }
}

impl Slug {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
