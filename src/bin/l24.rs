/// Develop a program that checks that all characters in a string are unique
/// (true if unique, false etc). The check function must be case-insensitive.
/// For example:
/// abcd — true abCdefAaf — false aabcd — false

fn chars_unique(string: &str) -> bool {
    let x = string
        .chars()
        .flat_map(char::to_lowercase)
        .collect::<std::collections::BTreeSet<_>>();

    x.iter().len() == string.len()
}

fn main() {
    println!("{}", chars_unique("abcd"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        assert!(chars_unique("abcd"));
        assert!(!chars_unique("abCdefAaf"));
        assert!(!chars_unique("aabcd"));
    }
}
