use std::str::FromStr;

use namespacedkey::{Identifier, define_identifier};

#[test]
fn define_identifier_works() {
    define_identifier!(
        foobar => "foo:bar"
    );

    assert_eq!(id_foobar(), Identifier::<()>::from_str("foo:bar").unwrap())
}
