use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct EmptyString;

impl Display for EmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "zero-value string")
    }
}

impl std::error::Error for EmptyString {}
