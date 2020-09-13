pub fn strip_trailing_char(string: String) -> String {
    let mut s = string.chars().rev();
    s.next().unwrap();
    let s = s.rev();
    s.collect::<String>()
}
