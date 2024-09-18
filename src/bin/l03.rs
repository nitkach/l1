/// An array of numbers is given (initialize it 1..N).
/// Using parallel calculations, find the sum of the squares of these numbers
/// and output to stdout.
///
/// Use only the standard library and the thread and mpsc modules.

const N: usize = 50;

fn main() {
    let (sender, receiver) = std::sync::mpsc::channel();

    let array = (1..=N).collect::<Vec<_>>();

    std::thread::scope(|s| {
        for num in array {
            let sender = sender.clone();
            s.spawn(move || {
                sender.send(num * num).unwrap();
            });
        }
    });

    // all senders (the original and its clones) need to be dropped
    // for the receiver to stop blocking to receive messages with Receiver::recv
    drop(sender);

    println!("{}", receiver.iter().sum::<usize>());
}
