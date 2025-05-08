use namespacedkey::error::InvalidKeyError;

#[test]
fn keeps_constructor_parameters() {
    let error = InvalidKeyError::new("hello", "world");
    assert_eq!(error.namespace(), "hello");
    assert_eq!(error.path(), "world");
    assert_eq!(
        error.to_string(),
        "Invalid namespaced key (namespace = 'hello', path = 'world')"
    );
}

#[test]
fn respects_custom_message() {
    let error = InvalidKeyError::new("foo", "bar").with_message("something...");
    assert_eq!(error.to_string(), "something...");
}
