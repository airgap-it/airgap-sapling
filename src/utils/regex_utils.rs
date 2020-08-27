pub fn contains_chars_re(chars: &[&str]) -> String {
  format!("[{}]", chars.join(""))
}

pub fn not_contains_chars_re(chars: &[&str]) -> String {
  format!("[^{}]", chars.join(""))
}