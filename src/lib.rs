#![doc = include_str!("../README.md")]

/// A simple String wrapper type, similar to NonZeroUsize and friends.
/// Guarantees that the String contained inside is not of length 0.
#[derive(Debug, Clone)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Attempts to create a new NonEmptyString.
    /// If the given `string` is empty, `None` is returned, `Some` otherwise.
    pub fn new(string: String) -> Option<NonEmptyString> {
        if string.is_empty() {
            None
        } else {
            Some(NonEmptyString(string))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl std::convert::AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::convert::AsRef<String> for NonEmptyString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn empty_string_returns_none() {
        assert_matches!(NonEmptyString::new("".to_owned()), None);
    }

    #[test]
    fn non_empty_string_returns_some() {
        assert_matches!(NonEmptyString::new("string".to_owned()), Some(_));
    }

    #[test]
    fn what_goes_in_comes_out() {
        assert_eq!(
            NonEmptyString::new("string".to_owned())
                .unwrap()
                .into_inner(),
            "string".to_owned()
        );
    }
}
