/// Remove the n-th element from the vector.

fn remove_nth_1<T>(items: &mut Vec<T>, nth: usize) {
    items.remove(nth);
}

fn remove_nth_2<T>(items: Vec<T>, nth: usize) -> Vec<T> {
    items
        .into_iter()
        .enumerate()
        .filter_map(|(index, item)| (index != nth).then_some(item))
        .collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use expect_test::{expect, Expect};

    use super::*;

    fn remove_and_assert_eq<T: Eq + Debug>(mut actual: Vec<T>, nth: usize, expected: &Expect) {
        remove_nth_1(&mut actual, nth);
        expected.assert_eq(&format!("{actual:?}"));
    }

    #[test]
    fn smoke_test_1() {
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 0, &expect!["[2, 3, 4, 5]"]);
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 1, &expect!["[1, 3, 4, 5]"]);
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 2, &expect!["[1, 2, 4, 5]"]);
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 3, &expect!["[1, 2, 3, 5]"]);
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 4, &expect!["[1, 2, 3, 4]"]);
    }

    #[test]
    fn smoke_test_2() {
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 0, &expect!["[2, 3, 4, 5]"]);
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 1, &expect!["[1, 3, 4, 5]"]);
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 2, &expect!["[1, 2, 4, 5]"]);
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 3, &expect!["[1, 2, 3, 5]"]);
        remove_and_assert_eq(vec![1, 2, 3, 4, 5], 4, &expect!["[1, 2, 3, 4]"]);
    }
}
