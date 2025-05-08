use crate::{
    Keyed,
    constants::{VALID_NAMESPACE_CHARACTERS, VALID_PATH_CHARACTERS},
    error::InvalidKeyError,
    util::{check_string, make_underline_message},
};
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

/// `NamespacedKey` is an identifier composed of a namespace and a path
///
/// # Examples
///
/// ```
/// # use namespacedkey::NamespacedKey;
///
/// let key_result = NamespacedKey::new("namespace", "path");
///
/// let key = match key_result {
///     Ok(key) => key,
///     Err(error) => panic!("Problem creating key: {error:?}"),
/// };
///
/// assert_eq!(key.to_string(), "namespace:path");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespacedKey {
    namespace: String,
    path: String,
}

impl NamespacedKey {
    /// Creates a new `NamespacedKey` from a `namespace` and a `path`.
    pub fn new<N, P>(namespace: N, path: P) -> Result<Self, InvalidKeyError>
    where
        N: AsRef<str>,
        P: AsRef<str>,
    {
        let ns = namespace.as_ref();
        if let Some(indices) = check_string(ns, VALID_NAMESPACE_CHARACTERS) {
            return Err(InvalidKeyError::new(ns, path.as_ref()).with_message(
                make_underline_message(
                    "Illegal characters in namespace:",
                    ns,
                    indices,
                    '^',
                ),
            ));
        }

        let p = path.as_ref();
        if let Some(indices) = check_string(p, VALID_PATH_CHARACTERS) {
            return Err(InvalidKeyError::new(ns, p).with_message(
                make_underline_message(
                    "Illegal characters in path:",
                    p,
                    indices,
                    '^',
                ),
            ));
        }

        Ok(Self {
            namespace: ns.to_owned(),
            path: p.to_owned(),
        })
    }

    /// Gets the namespace of this `NamespacedKey`.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    /// Gets the path of this `NamespacedKey`.
    pub fn path(&self) -> &str {
        &self.path
    }

    // Creates a representation of this `NamespacedKey` as a string, separating
    // the namespace and path using the `separator` character.
    pub fn as_string(&self, separator: char) -> String {
        format!("{}{}{}", &self.namespace, separator, &self.path)
    }
}

impl Keyed for NamespacedKey {
    fn key(&self) -> &Self {
        self
    }
}

impl Display for NamespacedKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_string(':'))
    }
}

impl<N, P> TryFrom<(N, P)> for NamespacedKey
where
    N: AsRef<str>,
    P: AsRef<str>,
{
    type Error = InvalidKeyError;

    fn try_from((ns, p): (N, P)) -> Result<Self, Self::Error> {
        NamespacedKey::new(ns, p)
    }
}
