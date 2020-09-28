pub fn strip_trailing_char(string: &str) -> String {
    let mut s = string.chars().rev();

    if s.next().is_none() {
        "".to_string()
    } else {
        let s = s.rev();
        s.collect::<String>()
    }
}
