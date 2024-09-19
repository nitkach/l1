/// Write a program that will calculate the squares of the numbers taken from
/// the array in parallel (initialize the array 1..N) and output them to stdout.
///
/// The numbers can be output in any order.
///
/// Use only the standard library.

const N: usize = 50;

fn main() {
    let array = (1..=N).collect::<Vec<_>>();

    // scope allows scoped threads to borrow non-'static data
    std::thread::scope(|s| {
        for num in array {
            s.spawn(move || println!("{}^2 = {}", num, num * num));
        }
    });
}
