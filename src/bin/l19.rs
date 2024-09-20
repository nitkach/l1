/// Develop a program that reverses words in a string.
/// Example: "snow dog sun — sun dog snow".

fn reverse_words(string: &str) -> String {
    let reversed = string.split_whitespace().rev().collect::<Vec<_>>();

    reversed.join(" ")
}

fn main() {
    println!("{}", reverse_words("foo bar-bar baz qux"));
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::*;

    fn assert_eq(actual: &str, expected: &Expect) {
        expected.assert_eq(&reverse_words(actual));
    }

    #[test]
    fn smoke_test_1() {
        assert_eq("snow dog sun", &expect!["sun dog snow"]);
        assert_eq("foo bar baz qux", &expect!["qux baz bar foo"]);
        assert_eq("quxquxqux", &expect!["quxquxqux"]);
        assert_eq("", &expect![""]);
    }
}
