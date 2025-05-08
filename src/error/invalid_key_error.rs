use std::fmt;

/// An error returned when a namespaced key is invalid.
///
/// This error carries the `namespace` and `path` that were tested, and
/// optionally a custom `message` for more detail.
///
/// # Examples
///
/// ```
/// # use namespacedkey::error::InvalidKeyError;
///
/// let err = InvalidKeyError::new("foo", "bar")
///     .with_message("missing 'bar' segment");
/// ```
#[derive(Debug)]
pub struct InvalidKeyError {
    namespace: String,
    path: String,
    message: Option<String>,
}

impl InvalidKeyError {
    /// Creates a new `InvalidKeyError`.
    ///
    /// ```
    /// # use namespacedkey::error::InvalidKeyError;
    ///
    /// let err = InvalidKeyError::new("my_ns", "some/path");
    /// ```
    pub fn new<N: Into<String>, P: Into<String>>(
        namespace: N,
        path: P,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            path: path.into(),
            message: None,
        }
    }

    /// Set this error's message
    ///
    /// ```
    /// # use namespacedkey::error::InvalidKeyError;
    ///
    /// let err = InvalidKeyError::new("ns", "key")
    ///     .with_message("something went wrong");
    /// ```
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Gets the namespace involved in this error.
    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    // Gets the path involved in this error.
    pub fn path(&self) -> &str {
        &self.path
    }
}

impl fmt::Display for InvalidKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(message) => write!(f, "{}", message),
            None => write!(
                f,
                "Invalid namespaced key (namespace = '{}', path = '{}')",
                self.namespace(),
                self.path()
            ),
        }
    }
}

impl std::error::Error for InvalidKeyError {}
