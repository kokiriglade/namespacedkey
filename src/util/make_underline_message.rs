use unicode_width::UnicodeWidthChar;

/// Makes a string that "underlines" characters in a string, `input`, denoted
/// by `bad_indices`, preceded by a `label`, using the given `character`.
pub fn make_underline_message(
    label: &str,
    input: &str,
    bad_indices: Vec<usize>,
    character: char,
) -> String {
    // Print the label and the input line
    let mut out = String::new();
    out.push_str(label);
    out.push(' ');
    out.push_str(input);
    out.push('\n');

    // Compute indentation to align the "underline" line
    let mut prefix = " ".to_string();
    for ch in label.chars() {
        let w = UnicodeWidthChar::width(ch).unwrap_or(1);
        prefix.push_str(&" ".repeat(w));
    }

    // Now build the "underline" line, marking only bad indices
    let mut underline = String::new();
    let mut byte_to_column = Vec::new();
    let mut col = 0;
    for (i, ch) in input.char_indices() {
        let w = UnicodeWidthChar::width(ch).unwrap_or(1);
        // Record starting column of this char
        byte_to_column.push((i, col, w));
        col += w;
    }
    underline.push_str(&" ".repeat(prefix.len()));
    // Place characters at columns matching bad_indices
    let mut underline_vec = vec![' '; col];
    for bad in bad_indices {
        if let Some(&(_, start_col, _)) =
            byte_to_column.iter().find(|&&(b, _, _)| b == bad)
        {
            underline_vec[start_col] = character;
            // Leave the rest as spaces
        }
    }
    for c in underline_vec {
        underline.push(c);
    }
    out.push_str(&underline);
    out
}
