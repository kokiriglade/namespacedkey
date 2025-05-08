/// Ensures a string only contains characters in an allowlist.\
/// Returns the indices of the illegal characters, if any.
pub fn check_string(string: &str, allowlist: &[char]) -> Option<Vec<usize>> {
    let illegal_indices: Vec<usize> = string
        .char_indices()
        .filter(|&(_, ch)| !allowlist.contains(&ch))
        .map(|(idx, _)| idx)
        .collect();

    if illegal_indices.is_empty() {
        None
    } else {
        Some(illegal_indices)
    }
}
