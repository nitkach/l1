use itertools::Itertools;
use std::io::BufRead;

/// A sequence of lines is received at the input through the standard
/// input stream, the lines can be repeated.
/// It is necessary to output lines to the standard
/// output stream, eliminating repetitions, without using `std::collections::*`.

fn main() {
    let handle = std::io::stdin().lock();

    if let Err(err) = handle
        .lines()
        .process_results(|iter| iter.dedup().for_each(|string| println!("{string}")))
    {
        panic!("{err}");
    }
}
