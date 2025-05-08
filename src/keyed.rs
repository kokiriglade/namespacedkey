use crate::NamespacedKey;

/// A trait for types that expose a [`NamespacedKey`].
///
/// Implementors of this trait can return a reference to their underlying
/// `NamespacedKey`.
///
/// # Examples
///
/// ```
/// # use namespacedkey::{Keyed, NamespacedKey};
/// struct MyType {
///     key: NamespacedKey,
/// }
///
/// impl Keyed for MyType {
///     fn key(&self) -> &NamespacedKey {
///         &self.key
///     }
/// }
///
/// let my = MyType {
///     key: NamespacedKey::new("foo", "bar").unwrap(),
/// };
/// assert_eq!(my.key().namespace(), "foo");
/// assert_eq!(my.key().path(), "bar");
/// ```
pub trait Keyed {
    /// Gets the `NamespacedKey`.
    fn key(&self) -> &NamespacedKey;
}
