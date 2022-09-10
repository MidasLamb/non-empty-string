use std::fmt;

use serde::de::{self, Unexpected, Visitor};

use crate::NonEmptyString;

struct NonEmptyStringVisitor;

impl<'de> de::Deserialize<'de> for NonEmptyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(NonEmptyStringVisitor)
    }
}

pub enum DeserializeError {}

type Result<T, E = DeserializeError> = std::result::Result<T, E>;

impl<'de> Visitor<'de> for NonEmptyStringVisitor {
    type Value = NonEmptyString;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_string(value.to_owned())
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        NonEmptyString::new(value).map_err(|e| de::Error::invalid_value(Unexpected::Str(&e), &self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use assert_matches::assert_matches;
    use serde_json::json;

    #[test]
    fn deserialize_works() {
        let e: Result<NonEmptyString, _> = serde_json::from_value(json!("abc"));

        let expected = NonEmptyString("abc".to_owned());

        assert_matches!(e, Ok(v) if v == expected)
    }

    #[test]
    fn deserialize_empty_fails() {
        let e: Result<NonEmptyString, _> = serde_json::from_value(json!(""));

        assert!(e.is_err());
        // assert_matches!(e, Ok(expected))
    }
}
