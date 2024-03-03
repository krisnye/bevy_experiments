pub fn pascal_case_to_readable(s: &str) -> String {
    let mut result = String::new();
    for (i, char) in s.chars().enumerate() {
        // If it's an uppercase letter and not the first character, prepend a space
        if char.is_uppercase() && i != 0 {
            result.push(' ');
        }
        result.push(char);
    }
    result
}