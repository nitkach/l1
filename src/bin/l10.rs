use std::time::Duration;

/// Develop a pipeline of numbers using channels.
/// Numbers from an array initialized 1..N are written to the first channel
/// with pauses. The first thread or task reads from this channel and writes the square
/// of the resulting number to the second channel.
/// The second thread or task reads from the second channel and outputs to stdout.

async fn numbers_pipeline(length: u32) {
    let array = (1..=length).collect::<Vec<_>>();

    // channel 1: write number -> receive number
    let (sender_writer, receiver_mediator) = tokio::sync::mpsc::channel::<u32>(16);
    // channel 2: send square number -> receiver square number
    let (sender_mediator, receiver_reader) = tokio::sync::mpsc::channel::<u32>(16);

    // first task: for receiving numbers, square it and send to second channel
    let receiver_mediator_join = tokio::spawn(async {
        let mut receiver_mediator = receiver_mediator;
        let sender_mediator = sender_mediator;
        loop {
            if let Some(num) = receiver_mediator.recv().await {
                let num = num * num;
                if sender_mediator.send(num).await.is_err() {
                    println!(
                        "Reader receivers dropped - channel is closed and no more messages left."
                    );
                    break;
                };
            } else {
                println!("Writer senders dropped - channel is closed and no more messages left.");
                break;
            }
        }
    });

    // second task: for receiving squared numbers and print it to stdout
    let receiver_reader_join = tokio::spawn(async {
        let mut receiver_reader = receiver_reader;
        loop {
            if let Some(num) = receiver_reader.recv().await {
                println!("Received: {num}");
            } else {
                println!("Mediator senders dropped - channel is closed and no more messages left.");
                break;
            }
        }
    });

    // send numbers
    for number in array {
        if sender_writer.send(number).await.is_err() {
            println!("Receiver of a channel is disconnected - the channel is closed");
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    // necessary drop! if not, first channel in first task will continue awaiting
    // for receiving numbers
    drop(sender_writer);
    receiver_mediator_join.await.unwrap();
    receiver_reader_join.await.unwrap();
}

#[tokio::main]
async fn main() {
    numbers_pipeline(20).await;
}
