use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    marker::PhantomData,
    str::FromStr,
    sync::OnceLock,
};

use internment::Intern;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The default namespace string when none is provided.
pub const DEFAULT_NAMESPACE: &str = "unspecified";

/// The separator character between the namespace and value.
pub const DEFAULT_SEPARATOR: char = ':';

static LEGAL_VALUE: OnceLock<HashSet<char>> = OnceLock::new();
static LEGAL_NS: OnceLock<HashSet<char>> = OnceLock::new();

/// Returns the set of legal characters for [`Identifier`] values.
pub fn legal_value_chars() -> &'static HashSet<char> {
    LEGAL_VALUE.get_or_init(|| {
        "0123456789abcdefghijklmnopqrstuvwxyz_-./".chars().collect()
    })
}

/// Returns the set of legal characters for [`Identifier`] namespaces.
pub fn legal_namespace_chars() -> &'static HashSet<char> {
    LEGAL_NS.get_or_init(|| {
        "0123456789abcdefghijklmnopqrstuvwxyz_-.".chars().collect()
    })
}

/// An identifier consisting of a `namespace` and a `value`.
///
/// # Examples
///
/// ```
/// use namespacedkey_core::{Identifier, IdentifierUntyped};
/// use std::str::FromStr;
///
/// let loc: IdentifierUntyped = Identifier::from_str("game:item/sword").unwrap();
/// //       ^^^^^^^^^^^^^^^^^ alias for `Identifier<()>`
/// assert_eq!(loc.namespace(), "game");
/// assert_eq!(loc.value, "item/sword");
/// assert_eq!(loc.to_string(), "game:item/sword");
///
/// let fallback: IdentifierUntyped = Identifier::from_str("thing").unwrap();
/// assert_eq!(fallback.namespace(), "unspecified"); // uses default
/// assert_eq!(fallback.to_string(), "unspecified:thing");
/// ```
///
/// # Performance
///
/// Cloning an `Identifier` is cheap: the `namespace` is automatically
/// [interned][internment], so multiple instances sharing the same namespace
/// do not duplicate memory or perform allocations (for the namespace portion,
/// at least).
///
/// [internment]: https://docs.rs/internment/latest/internment/
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "", deserialize = "")))]
#[cfg_attr(feature = "serde", serde(try_from = "String", into = "String"))]
pub struct Identifier<T> {
    pub namespace: Intern<String>,
    pub value: String,
    #[cfg_attr(feature = "serde", serde(skip))]
    type_marker: PhantomData<T>,
}

pub type IdentifierUntyped = Identifier<()>;

impl<T> Clone for Identifier<T> {
    fn clone(&self) -> Self {
        Identifier {
            namespace: self.namespace,
            value: self.value.clone(),
            type_marker: PhantomData,
        }
    }
}

impl<T> PartialEq for Identifier<T> {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.value == other.value
    }
}

impl<T> Eq for Identifier<T> {}

impl<T> Hash for Identifier<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.namespace.hash(state);
        self.value.hash(state);
    }
}

impl<T> PartialOrd for Identifier<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Identifier<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.namespace.cmp(&other.namespace) {
            Ordering::Equal => self.value.cmp(&other.value),
            non_eq => non_eq,
        }
    }
}

impl<T> Identifier<T> {
    /// Returns the namespace as a string slice.
    pub fn namespace(&self) -> &str {
        self.namespace.as_str()
    }

    /// Returns the namespace as a String.
    pub fn namespace_string(&self) -> String {
        (*self.namespace).clone()
    }

    pub fn new<S: Into<String>>(
        namespace: S,
        value: S,
    ) -> Result<Self, ParseError> {
        let namespace = namespace.into();
        let value = value.into();

        if value.is_empty() {
            return Err(ParseError::EmptyValue);
        }

        // Collect *all* bad chars in the namespace
        let bad_ns: Vec<(usize, char)> = namespace
            .char_indices()
            .filter(|&(_, ch)| !legal_namespace_chars().contains(&ch))
            .collect();
        if !bad_ns.is_empty() {
            return Err(ParseError::IllegalCharsInNamespace(namespace, bad_ns));
        }

        // Collect *all* bad chars in the value
        let bad_val: Vec<(usize, char)> = value
            .char_indices()
            .filter(|&(_, ch)| !legal_value_chars().contains(&ch))
            .collect();
        if !bad_val.is_empty() {
            return Err(ParseError::IllegalCharsInValue(value, bad_val));
        }

        let ns = if namespace.is_empty() {
            DEFAULT_NAMESPACE.to_string()
        } else {
            namespace
        };

        Ok(Identifier {
            namespace: Intern::new(ns),
            value,
            type_marker: PhantomData,
        })
    }

    /// Parses a string into an [`Identifier`], defaulting the namespace if omitted.
    pub fn parse<S: Into<String>>(s: S) -> Result<Self, ParseError> {
        let s = s.into();
        let mut parts = s.splitn(2, DEFAULT_SEPARATOR);
        let before = parts.next().unwrap_or("");
        let after = parts.next().unwrap_or(before);
        let (namespace, value) = if before == after {
            ("", before)
        } else {
            (before, after)
        };

        Self::new(namespace, value)
    }

    /// Change the phantom type to `U`.
    pub fn cast<U>(self) -> Identifier<U> {
        Identifier {
            namespace: self.namespace,
            value: self.value,
            type_marker: PhantomData,
        }
    }

    /// Erase type data.
    pub fn erase(self) -> Identifier<()> {
        self.cast::<()>()
    }
}

impl<T> Display for Identifier<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}{}", self.namespace, DEFAULT_SEPARATOR, self.value)
    }
}

/// Error type returned when an [`Identifier`] cannot be parsed.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// No value after the separator.
    EmptyValue,

    /// One or more illegal characters in the namespace.
    IllegalCharsInNamespace(String, Vec<(usize, char)>),

    /// One or more illegal characters in the value.
    IllegalCharsInValue(String, Vec<(usize, char)>),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ParseError::EmptyValue => {
                write!(f, "empty value")
            }
            ParseError::IllegalCharsInNamespace(ns, bad) => {
                write!(f, "illegal character(s) in namespace {ns:?}:")?;
                for (idx, ch) in bad {
                    write!(f, " `{ch}`@{idx}")?;
                }
                Ok(())
            }
            ParseError::IllegalCharsInValue(val, bad) => {
                write!(f, "illegal character(s) in value {val:?}:")?;
                for (idx, ch) in bad {
                    write!(f, " `{ch}`@{idx}")?;
                }
                Ok(())
            }
        }
    }
}

impl<T> TryFrom<String> for Identifier<T> {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Identifier::parse(s)
    }
}

impl<T> From<Identifier<T>> for String {
    fn from(id: Identifier<T>) -> String {
        id.to_string()
    }
}

impl<T> FromStr for Identifier<T> {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Identifier::parse(s.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_NAMESPACE, Identifier, ParseError};
    use std::str::FromStr;

    #[test]
    fn parse_valid_full() {
        let rl = Identifier::<()>::from_str("foo:bar_baz").unwrap();
        assert_eq!(rl.namespace.as_ref(), "foo");
        assert_eq!(rl.value, "bar_baz");
    }

    #[test]
    fn parse_valid_default_ns() {
        let rl = Identifier::<()>::from_str(":stone").unwrap();
        assert_eq!(rl.namespace.as_ref(), DEFAULT_NAMESPACE);
        assert_eq!(rl.value, "stone");
    }

    #[test]
    fn parse_missing_separator_defaults() {
        let rl = Identifier::<()>::from_str("no_sep").unwrap();
        assert_eq!(rl.namespace.as_ref(), DEFAULT_NAMESPACE);
        assert_eq!(rl.value, "no_sep");
    }

    #[test]
    fn parse_illegal_ns_char_multiple() {
        let input = "b@d/ns:stone";
        let err = Identifier::<()>::from_str(input).unwrap_err();
        match err {
            ParseError::IllegalCharsInNamespace(ns, bad) => {
                assert_eq!(ns, "b@d/ns");
                assert_eq!(bad, vec![(1, '@'), (3, '/')]);
            }
            _ => panic!("expected IllegalCharsInNamespace"),
        }
    }

    #[test]
    fn parse_illegal_value_char_multiple() {
        let input = "namespacedkey:ba g!d";
        let err = Identifier::<()>::from_str(input).unwrap_err();
        match err {
            ParseError::IllegalCharsInValue(val, bad) => {
                assert_eq!(val, "ba g!d");
                assert_eq!(bad, vec![(2, ' '), (4, '!')]);
            }
            _ => panic!("expected IllegalCharsInValue"),
        }
    }

    #[test]
    fn parse_empty_value() {
        let input = "namespace:";
        let err = Identifier::<()>::from_str(input).unwrap_err();
        match err {
            ParseError::EmptyValue => {}
            _ => panic!("expected EmptyValue"),
        }
    }
}
