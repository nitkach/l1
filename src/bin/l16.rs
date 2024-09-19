use std::fmt::Debug;

fn binary_search<T: Ord>(items: &[T], target: &T) -> Result<usize, usize> {
    //  Ok(_) -> target index in items, returning corresponding index
    // Err(_) -> target not found in items, returning index where a matching
    //           element could be inserted while maintaining sorted order
    items.binary_search(target)
}

fn binary_search_2<T: Ord + Debug>(items: &[T], target: &T) -> Option<usize> {
    let mut left = 0;
    let mut right = items.len().checked_sub(1)?;

    while left <= right {
        let middle = left + (right - left) / 2;

        let item = items.get(middle).unwrap();
        match dbg!(target.cmp(item)) {
            std::cmp::Ordering::Less => right = middle.checked_sub(1)?,
            std::cmp::Ordering::Equal => return Some(middle),
            std::cmp::Ordering::Greater => left = middle.checked_add(1)?,
        }
    }
    None
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        let vec = (1..=10).collect::<Vec<_>>();
        for target in 1..=10 {
            assert_eq!(binary_search_2(&vec, dbg!(&target)).unwrap(), target - 1);
        }
    }
}
