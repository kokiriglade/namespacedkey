use namespacedkey::util::check_string;

#[test]
fn returns_none_if_all_chars_are_valid() {
    let input = "abcde";
    let allowed = &['a', 'b', 'c', 'd', 'e'];
    assert_eq!(check_string(input, allowed), None);
}

#[test]
fn returns_indices_of_invalid_chars() {
    let input = "hello, world!";
    let allowed = &['h', 'e', 'l', 'o', 'w', 'r', 'd'];
    assert_eq!(check_string(input, allowed), Some(vec![5, 6, 12]));
    assert_ne!(check_string(input, allowed), Some(vec![6, 5, 12]));
}
