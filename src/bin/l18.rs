/// Develop a program that reverses the string given to it
/// (for example: "главрыба" — "абырвалг").
/// Symbols can be unicode.

fn reverse_string(string: &str) -> String {
    string.chars().rev().collect()
}

fn main() {
    println!("{}", reverse_string("главрыба"));
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::*;

    fn assert_eq(actual: &str, expected: &Expect) {
        expected.assert_eq(actual);
    }

    #[test]
    fn smoke_test() {
        assert_eq(&reverse_string("главрыба"), &expect!["абырвалг"]);
        assert_eq(&reverse_string("йцукен гшщзх"), &expect!["хзщшг некуцй"]);
        assert_eq(&reverse_string("qwerty uiop["), &expect!["[poiu ytrewq"]);
        assert_eq(&reverse_string("qwerty гшщзх"), &expect!["хзщшг ytrewq"]);
    }
}
