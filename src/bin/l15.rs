use std::{cmp::Ordering, fmt::Debug};

/// Implement quick array sorting (quicksort) using the built-in methods of the language.

fn built_in_quicksort<T: Ord>(array: &mut [T]) {
    array.sort();
}

// good explanation: https://www.youtube.com/watch?v=9KBwdDEwal8
fn quicksort<T: Ord + Debug>(array: &mut [T]) {
    let len = array.len();

    if len <= 1 {
        return;
    }

    let start = 0;
    let end = array.len() - 1;

    if start >= end {
        return;
    }

    quicksort_inner(array, start, end, &mut std::cmp::Ord::cmp);
}

fn quicksort_by<T: Ord + Debug, F>(array: &mut [T], mut f: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let start = 0;
    let end = array.len() - 1;

    if start >= end {
        return;
    }

    quicksort_inner(array, start, end, &mut f);
}

fn quicksort_inner<T: Ord + Debug, F>(array: &mut [T], start: usize, end: usize, f: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if start >= end {
        return;
    }
    let pivot_index = partition(array, start, end, f);

    quicksort_inner(array, pivot_index + 1, end, f);
    if pivot_index > 0 {
        quicksort_inner(array, start, pivot_index - 1, f);
    };
}

fn partition<T: Ord + Debug, F>(array: &mut [T], start: usize, end: usize, f: &mut F) -> usize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut i = start;
    let mut j = end - 1;

    while i < j {
        while i < end && matches!(f(&array[i], &array[end]), std::cmp::Ordering::Less) {
            i += 1;
        }

        while j > start && !matches!(f(&array[j], &array[end]), Ordering::Less) {
            j -= 1;
        }

        if i < j {
            array.swap(i, j);
        }
    }

    if f(&array[i], &array[end]) == Ordering::Greater {
        array.swap(i, end);
    }

    i
}

fn main() {
    let mut array = vec![6, 3, 9, 1, 4, 2, 7, 8, 5];
    quicksort(&mut array);
    println!("         quicksort: {array:?}");

    array = vec![6, 3, 9, 1, 4, 2, 7, 8, 5];
    quicksort_by(&mut array, std::cmp::Ord::cmp);
    println!("      quicksort_by: {array:?}");

    array = vec![6, 3, 9, 1, 4, 2, 7, 8, 5];
    built_in_quicksort(&mut array);
    println!("built_in_quicksort: {array:?}");
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use expect_test::{expect, Expect};

    use super::*;

    fn assert_eq<T: Debug>(actual: &[T], expected: &Expect) {
        expected.assert_eq(&format!("{actual:?}"));
    }

    #[test]
    fn smoke_test() {
        let mut array = vec![6, 3, 9, 1, 4, 2, 7, 8, 5];

        quicksort(&mut array);

        assert_eq(&array, &expect!["[1, 2, 3, 4, 5, 6, 7, 8, 9]"]);
    }

    #[test]
    fn empty_array() {
        let mut array = Vec::<i32>::new();

        quicksort(&mut array);

        assert_eq(&array, &expect!["[]"]);
    }

    #[test]
    fn one_element() {
        let mut array = vec![1];

        quicksort(&mut array);

        assert_eq(&array, &expect!["[1]"]);
    }

    #[test]
    fn two_elements() {
        let mut array = vec![2, 1];

        quicksort(&mut array);

        assert_eq(&array, &expect!["[1, 2]"]);
    }

    #[test]
    fn three_elements() {
        let mut array = vec![3, 2, 1];

        quicksort(&mut array);

        assert_eq(&array, &expect!["[1, 2, 3]"]);
    }

    #[test]
    fn repeating_elements() {
        let mut array = vec![3, 2, 3, 3, 2, 1, 1, 2, 2];

        quicksort(&mut array);

        assert_eq(&array, &expect!["[1, 1, 2, 2, 2, 2, 3, 3, 3]"]);
    }

    #[derive(Ord, Eq, PartialOrd)]
    struct ComplexStructure {
        compared_by: i32,
        other: usize,
    }

    impl Debug for ComplexStructure {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&format!("{{ {}, {} }}", self.compared_by, self.other))
        }
    }

    impl PartialEq for ComplexStructure {
        fn eq(&self, other: &Self) -> bool {
            self.compared_by == other.compared_by
        }
    }

    impl ComplexStructure {
        fn new(other: usize, compared_by: i32) -> Self {
            Self { compared_by, other }
        }
    }

    #[test]
    fn inner_ord() {
        let mut array = [6, 3, 9, 1, 4, 2, 7, 8, 5]
            .into_iter()
            .enumerate()
            .map(|(other, compared_by)| ComplexStructure::new(other, compared_by))
            .collect::<Vec<ComplexStructure>>();

        quicksort_by(&mut array, |a, b| a.compared_by.cmp(&b.compared_by));

        assert_eq(&array, &expect!["[{ 1, 3 }, { 2, 5 }, { 3, 1 }, { 4, 4 }, { 5, 8 }, { 6, 0 }, { 7, 6 }, { 8, 7 }, { 9, 2 }]"]);
    }

    #[test]
    fn inner_repeating_ord() {
        //         other: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9,10,11,12,13,14,15,16]
        //   compared_by: [3, 2, 3, 3, 2, 1, 1, 2, 2, 1, 3, 1, 1, 2, 3, 2, 2]

        // actual
        // after sorting
        //         other: [12,11, 9, 6, 5, 4,16, 7, 8,15,13, 1, 0,10,14, 2, 3]
        //   compared_by: [ 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3]
        let mut array = [3, 2, 3, 3, 2, 1, 1, 2, 2, 1, 3, 1, 1, 2, 3, 2, 2]
            .into_iter()
            .enumerate()
            .map(|(other, compared_by)| ComplexStructure::new(other, compared_by))
            .collect::<Vec<ComplexStructure>>();

        quicksort_by(&mut array, |a, b| a.compared_by.cmp(&b.compared_by));

        assert_eq(&array, &expect!["[{ 1, 12 }, { 1, 11 }, { 1, 9 }, { 1, 6 }, { 1, 5 }, { 2, 4 }, { 2, 16 }, { 2, 7 }, { 2, 8 }, { 2, 15 }, { 2, 13 }, { 2, 1 }, { 3, 0 }, { 3, 10 }, { 3, 14 }, { 3, 2 }, { 3, 3 }]"]);
    }

    #[test]
    fn built_in_quicksort_inner_repeating_ord() {
        let mut array = [3, 2, 3, 3, 2, 1, 1, 2, 2, 1, 3, 1, 1, 2, 3, 2, 2]
            .into_iter()
            .enumerate()
            .map(|(other, compared_by)| ComplexStructure::new(other, compared_by))
            .collect::<Vec<ComplexStructure>>();

        array.sort_by(|a, b| a.compared_by.cmp(&b.compared_by));

        assert_eq(&array, &expect!["[{ 1, 5 }, { 1, 6 }, { 1, 9 }, { 1, 11 }, { 1, 12 }, { 2, 1 }, { 2, 4 }, { 2, 7 }, { 2, 8 }, { 2, 13 }, { 2, 15 }, { 2, 16 }, { 3, 0 }, { 3, 2 }, { 3, 3 }, { 3, 10 }, { 3, 14 }]"]);
    }
}
