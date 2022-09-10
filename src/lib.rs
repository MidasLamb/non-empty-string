#![doc = include_str!("../README.md")]

#[cfg(feature = "serde")]
mod serde_support;

/// A simple String wrapper type, similar to NonZeroUsize and friends.
/// Guarantees that the String contained inside is not of length 0.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonEmptyString(String);

impl NonEmptyString {
    /// Attempts to create a new NonEmptyString.
    /// If the given `string` is empty, `Err` is returned, containing the original `String`, `Ok` otherwise.
    pub fn new(string: String) -> Result<NonEmptyString, String> {
        if string.is_empty() {
            Err(string)
        } else {
            Ok(NonEmptyString(string))
        }
    }

    /// Returns a reference to the contained value.
    pub fn get(&self) -> &str {
        &self.0
    }

    /// Consume the `NonEmptyString` to get the internal `String` out.
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

impl<'s> std::convert::TryFrom<&'s str> for NonEmptyString {
    type Error = ();

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(())
        } else {
            Ok(NonEmptyString::new(value.to_owned()).expect("Value is not empty"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn empty_string_returns_none() {
        assert_eq!(NonEmptyString::new("".to_owned()), Err("".to_owned()));
    }

    #[test]
    fn non_empty_string_returns_some() {
        assert_matches!(NonEmptyString::new("string".to_owned()), Ok(_));
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

    #[test]
    fn as_ref_str_works() {
        let nes = NonEmptyString::new("string".to_owned()).unwrap();
        let val: &str = nes.as_ref();
        assert_eq!(val, "string");
    }

    #[test]
    fn as_ref_string_works() {
        let nes = NonEmptyString::new("string".to_owned()).unwrap();
        let val: &String = nes.as_ref();
        assert_eq!(val, "string");
    }

    #[test]
    fn calling_string_methods_works() {
        let nes = NonEmptyString::new("string".to_owned()).unwrap();
        // `len` is a `String` method.
        assert!(nes.get().len() > 0);
    }
}
