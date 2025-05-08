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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespacedKey {
    namespace: String,
    path: String,
}

impl NamespacedKey {
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

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }

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
