/// Characters that can be inside of the namespace of a `NamespacedKey`.\
/// As a regular expression, this is `[a-z0-9_\\-.]+`.
pub static VALID_NAMESPACE_CHARACTERS: &[char; 39] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z', '_', '-', '.',
];

/// Characters that can be inside of the path of a `NamespacedKey`.\
/// As a regular expression, this is `[a-z0-9_\\-./]+`.
pub static VALID_PATH_CHARACTERS: &[char; 40] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y', 'z', '_', '-', '.', '/',
];
