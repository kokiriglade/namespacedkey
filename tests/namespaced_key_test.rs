use std::str::FromStr;

use namespacedkey::{Keyed, NamespacedKey};

#[test]
fn creates_valid_key() {
    let result = NamespacedKey::new("test", "path");
    assert!(result.is_ok());
    let key = result.unwrap();
    assert_eq!(key.namespace(), "test");
    assert_eq!(key.path(), "path");
}

#[test]
fn creates_error_when_given_invalid_namespace() {
    let result = NamespacedKey::new("!nvalid_name$pace!///", "valid_path");
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(
        error.to_string(),
        "Illegal characters in namespace: !nvalid_name$pace!///\n                                 ^           ^    ^^^^"
    );
}

#[test]
fn creates_error_when_given_invalid_path() {
    let result = NamespacedKey::new("valid_namespace", "!nvalid_path!///");
    assert!(result.is_err());
    let error = result.unwrap_err();

    assert_eq!(
        error.to_string(),
        "Illegal characters in path: !nvalid_path!///\n                            ^           ^   "
    );
}

#[test]
fn has_keyed_trait() {
    let key = NamespacedKey::new("namespace", "path").unwrap();
    assert_eq!(&key, key.key())
}

#[test]
fn display_impl_matches_to_string() {
    let key = NamespacedKey::new("foo", "bar").unwrap();
    assert_eq!(format!("{}", key), key.to_string());
}

#[test]
fn to_string() {
    let key = NamespacedKey::new("namespace", "path").unwrap();
    assert_eq!(key.to_string(), key.to_string_with_separator(':'))
}

#[test]
fn try_from_tuple() {
    let key: NamespacedKey = ("ns", "p").try_into().unwrap();
    assert_eq!(key.namespace(), "ns");
    assert_eq!(key.path(), "p");
}

#[test]
fn try_from_invalid_tuple() {
    let err = <NamespacedKey as TryFrom<(_, _)>>::try_from(("bad$", "ok"));
    assert!(err.is_err());
}

#[test]
fn various_valid_and_invalid() {
    let cases = [
        ("valid1", "good_path", true),
        ("no spaces", "ok", false),
        ("emoji😊", "путь", false),
        ("emoji😊", "п@th!", false),
    ];
    for (ns, p, should_be_ok) in cases {
        let result = NamespacedKey::new(ns, p);
        assert_eq!(result.is_ok(), should_be_ok, "ns={:?}, p={:?}", ns, p);
    }
}

#[test]
fn from_str() {
    let key_result = NamespacedKey::from_str("namespace:path");

    let key = match key_result {
        Ok(key) => key,
        Err(error) => {
            panic!("Failed to parse NamespacedKey from str: {error:?}")
        }
    };

    assert_eq!(key.namespace(), "namespace");
    assert_eq!(key.path(), "path");
}
