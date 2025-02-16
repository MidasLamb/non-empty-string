#[macro_export]
/// Creates a `NonEmptyString` from a string literal at compile time.
///
/// This macro ensures that the provided string is **not empty** at compile time,
/// preventing runtime errors due to empty strings.
///
/// # Examples
///
/// ```
/// use non_empty_string::{non_empty, NonEmptyString};
///
/// let s: NonEmptyString = non_empty!("Hello, Rust!");
/// assert_eq!(s, NonEmptyString::new("Hello, Rust!".to_string()).unwrap());
/// ```
///
/// # Compile-time Failure
///
/// If an empty string is provided, this macro will cause a **compile-time error**.
///
/// ```compile_fail
/// use non_empty_string::non_empty;
///
/// let s = non_empty!("");
/// ```
macro_rules! non_empty {
    ($s:expr) => {{
        // Compile-time assertion to ensure the string is non-empty
        const _: () = assert!(!$s.is_empty(), "String cannot be empty");

        // Create a NonEmptyString, unsafely wrapping since we've checked it's valid
        unsafe { NonEmptyString::new_unchecked($s.to_string()) }
    }};
}

#[cfg(test)]
mod tests {
    use crate::NonEmptyString;

    #[test]
    fn test_non_empty_string_macro_valid() {
        let s = non_empty!("Test String");
        assert_eq!(s, NonEmptyString::try_from("Test String").unwrap());
    }
}
