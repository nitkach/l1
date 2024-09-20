use std::{collections::HashSet, hash::Hash};

fn sets_intersection<T: Eq + Hash + Clone>(
    first_set: &HashSet<T>,
    second_set: &HashSet<T>,
) -> HashSet<T> {
    first_set.intersection(&second_set).cloned().collect()
}

fn main() {
    let first_set = HashSet::from_iter(vec![1, 2, 3, 4, 5]);
    let second_set = HashSet::from_iter(vec![3, 5, 6, 7]);

    let intersection = sets_intersection(&first_set, &second_set);

    print!("{{ ");
    for number in intersection {
        print!("{number} ");
    }
    println!("}}");
}
