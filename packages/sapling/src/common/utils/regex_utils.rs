pub fn contains_chars_re(chars: &[&str]) -> String {
    format!("[{}]", chars.join(""))
}

pub fn not_contains_chars_re(chars: &[&str]) -> String {
    format!("[^{}]", chars.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_contains_chars_regex() {
        let test_data = vec![
            (vec!["a"], "[a]"),
            (vec!["a", "b"], "[ab]"),
            (vec!["a-z", "A-Z"], "[a-zA-Z]")
        ];

        let actual_expected = test_data.iter()
            .map(|(chars, expected)| {
                let actual = contains_chars_re(chars);

                (actual, expected)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(&actual, expected);
        }
    }

    #[test]
    fn creates_not_contains_chars_regex() {
        let test_data = vec![
            (vec!["a"], "[^a]"),
            (vec!["a", "b"], "[^ab]"),
            (vec!["a-z", "A-Z"], "[^a-zA-Z]")
        ];

        let actual_expected = test_data.iter()
            .map(|(chars, expected)| {
                let actual = not_contains_chars_re(chars);

                (actual, expected)
            });

        for (actual, expected) in actual_expected {
            assert_eq!(&actual, expected);
        }
    }
}