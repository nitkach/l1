use std::time::Duration;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

/// Develop a program that will sequentially send values ​​to a channel,
/// and read from the other side of the channel.
/// After N seconds, the program should terminate.

#[tokio::main]
async fn main() {
    let working_time = 3;

    let (sender, mut receiver) = tokio::sync::mpsc::channel::<i32>(16);
    let cancellation_token = CancellationToken::new();

    // spawn a task that sends values
    let sender_join = {
        let sender = sender.clone();
        let cancellation_token = cancellation_token.clone();
        let mut counter = 0;
        tokio::spawn(async move {
            loop {
                if cancellation_token.is_cancelled() {
                    println!("Cancellation token has been cancelled.");
                    break;
                }

                if sender.send(counter).await.is_err() {
                    println!("Receiver dropped");
                    break;
                };

                counter += 1;
                sleep(Duration::from_millis(50)).await;
            }
        })
    };

    // spawn a task that receives values
    let receiver_join = tokio::spawn(async move {
        loop {
            if let Some(num) = receiver.recv().await {
                println!("Received: {num}");
            } else {
                println!("All senders dropped - channel is closed and no more messages left.");
                break;
            }
        }
    });

    // while sleep, the tasks are running and sending/receiving values
    sleep(Duration::from_secs(working_time)).await;

    drop(sender);
    println!("Sender dropped");
    sleep(Duration::from_millis(500)).await;

    cancellation_token.cancel();

    sleep(Duration::from_secs(1)).await;

    sender_join.await.unwrap();
    receiver_join.await.unwrap();
}
