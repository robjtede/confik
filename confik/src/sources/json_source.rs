use std::{borrow::Cow, error::Error, fmt};

use crate::{ConfigurationBuilder, Source};

/// A [`Source`] containing raw JSON data.
#[derive(Clone)]
pub struct JsonSource<'a> {
    contents: Cow<'a, str>,
    allow_secrets: bool,
}

impl<'a> JsonSource<'a> {
    /// Creates a [`Source`] containing raw JSON data.
    pub fn new(contents: impl Into<Cow<'a, str>>) -> Self {
        Self {
            contents: contents.into(),
            allow_secrets: false,
        }
    }

    /// Allows this source to contain secrets.
    pub fn allow_secrets(mut self) -> Self {
        self.allow_secrets = true;
        self
    }
}

impl Source for JsonSource<'_> {
    fn allows_secrets(&self) -> bool {
        self.allow_secrets
    }

    fn provide<T: ConfigurationBuilder>(&self) -> Result<T, Box<dyn Error + Sync + Send>> {
        Ok(serde_json::from_str(&self.contents)?)
    }
}

impl fmt::Debug for JsonSource<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JsonSource")
            .field("allow_secrets", &self.allow_secrets)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let source = JsonSource::new("{}");
        assert!(!source.allows_secrets());
    }

    #[test]
    fn clone() {
        let source = JsonSource::new("{}").allow_secrets();
        assert!(source.allows_secrets());
        assert!(source.clone().allow_secrets);
    }
}
