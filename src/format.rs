pub fn get_dashes(longest: usize, name_len: usize) -> String {
    "-".repeat(longest - name_len + 3)
}

pub fn add_padding(text: String, size: usize) -> String {
    " ".repeat(size - text.len()) + &text
}
