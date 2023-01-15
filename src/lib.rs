#![doc(issue_tracker_base_url = "https://github.com/MidasLamb/non-empty-string/issues/")]

//! # NonEmptyString
//! A simple wrapper type for `String`s that ensures that the string inside is not `.empty()`, meaning that the length > 0.

// Test the items in the readme file.
#[cfg(doctest)]
mod test_readme {
    #[doc = include_str!("../README.md")]
    mod something {}
}

use delegate::delegate;
use std::fmt::Display;
use std::ops::Deref;

#[cfg(feature = "serde")]
mod serde_support;

/// A simple String wrapper type, similar to NonZeroUsize and friends.
/// Guarantees that the String contained inside is not of length 0.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonEmptyString(String);

#[allow(clippy::len_without_is_empty)] // is_empty would always returns false so it seems a bit silly to have it.
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

    // These are safe methods that can simply be forwarded.
    delegate! {
        to self.0 {
            /// Is forwarded to the inner String.
            /// See [`String::into_bytes`]
            pub fn into_bytes(self) -> Vec<u8>;

            /// Is forwarded to the inner String.
            /// See [`String::as_str`]
            pub fn as_str(&self) -> &str;

            /// Is forwarded to the inner String.
            /// See [`String::push_str`]
            pub fn push_str(&mut self, string: &str);

            /// Is forwarded to the inner String.
            /// See [`String::capacity`]
            pub fn capacity(&self) -> usize;

            /// Is forwarded to the inner String.
            /// See [`String::reserve`]
            pub fn reserve(&mut self, additional: usize);

            /// Is forwarded to the inner String.
            /// See [`String::reserve_exact`]
            pub fn reserve_exact(&mut self, additional: usize);

            // For some reason we cannot delegate the following:
            // pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError>

            /// Is forwarded to the inner String.
            /// See [`String::try_reserve_exact`]
            pub fn try_reserve_exact(
                &mut self,
                additional: usize
            ) -> Result<(), std::collections::TryReserveError>;

            /// Is forwarded to the inner String.
            /// See std::string::String::[`(&`]
            pub fn shrink_to_fit(&mut self);

            /// Is forwarded to the inner String.
            /// See [`String::shrink_to`]
            pub fn shrink_to(&mut self, min_capacity: usize);

            /// Is forwarded to the inner String.
            /// See [`String::push`]
            pub fn push(&mut self, ch: char);

            /// Is forwarded to the inner String.
            /// See [`String::as_bytes`]
            pub fn as_bytes(&self) -> &[u8];

            /// Is forwarded to the inner String.
            /// See [`String::insert`]
            pub fn insert(&mut self, idx: usize, ch: char);

            /// Is forwarded to the inner String.
            /// See [`String::insert_str`]
            pub fn insert_str(&mut self, idx: usize, string: &str);

            /// Is forwarded to the inner String.
            /// See [`String::len`]
            pub fn len(&self) -> usize;

            /// Is forwarded to the inner String.
            /// See [`String::into_boxed_str`]
            pub fn into_boxed_str(self) -> Box<str>;
        }
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<String> for NonEmptyString {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl Deref for NonEmptyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'s> TryFrom<&'s str> for NonEmptyString {
    type Error = &'s str;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(value);
        }

        Ok(NonEmptyString(value.to_owned()))
    }
}

impl TryFrom<String> for NonEmptyString {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NonEmptyString::new(value)
    }
}

impl Display for NonEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_returns_err() {
        assert_eq!(NonEmptyString::new("".to_owned()), Err("".to_owned()));
    }

    #[test]
    fn non_empty_string_returns_ok() {
        assert!(NonEmptyString::new("string".to_owned()).is_ok())
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
        assert!(nes.len() > 0);
    }

    #[test]
    fn format_test() {
        let str = NonEmptyString::new("string".to_owned()).unwrap();
        println!("{}", &str);
        assert_eq!(String::from("string"), str.to_string())
    }


    fn print_str(str: &str) {
        println!("{str}")
    }

    #[test]
    fn deref_works() {
        let str = "My String";
        let non_empty_string = NonEmptyString::try_from(str).unwrap();
        print_str(&non_empty_string);
        assert_eq!(str, non_empty_string.deref());
    }
}
