pub fn strip_trailing_char(string: String) -> String {
    let mut s = string.chars().rev();

    if let Some(_) = s.next() {
        let s = s.rev();
        s.collect::<String>()
    } else {
        "".to_string()
    }
}
