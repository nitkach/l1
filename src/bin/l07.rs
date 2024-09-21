use std::time::Duration;

/// Implement several ways to stop threads and tokio tasks:
/// closing the channel, `tokio_util::CancellationToken`

// using channel closing and scoped threads
fn closing_scoped_threads() {
    let (sender, receiver) = std::sync::mpsc::sync_channel::<i32>(16);

    std::thread::scope(|s| {
        s.spawn(move || loop {
            if let Ok(num) = receiver.recv() {
                println!("Received: {num}");
            } else {
                println!("Sender of channel is disconnected - the channel is closed");
                break;
            }
        });

        let mut data = 0;
        loop {
            if sender.send(data).is_err() {
                println!("Receiver of a channel is disconnected - the channel is closed");
                break;
            }
            data += 1;

            if data > 50 {
                println!("All data sended!");
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        drop(sender);
    });
}

// using channel closing with threads
fn closing_joined_threads() {
    let (sender, receiver) = std::sync::mpsc::sync_channel::<i32>(16);

    let receiver_join = std::thread::spawn(move || loop {
        if let Ok(num) = receiver.recv() {
            println!("Received: {num}");
        } else {
            println!("Sender of channel is disconnected - the channel is closed");
            break;
        }
    });

    let mut data = 0;
    loop {
        if sender.send(data).is_err() {
            println!("Receiver of a channel is disconnected - the channel is closed");
            break;
        }
        data += 1;

        if data > 50 {
            println!("All data sended!");
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    drop(sender);
    receiver_join.join().unwrap();
}

// using tokio task and closing channel
async fn closing_tasks() {
    let (sender, receiver) = tokio::sync::mpsc::channel::<i32>(16);

    let receiver_join = tokio::spawn(async move {
        let mut receiver = receiver;
        loop {
            if receiver.is_closed() {
                println!("Sender of channel is disconnected - the channel is closed");
                break;
            }
            if let Some(num) = receiver.recv().await {
                println!("Received: {num}");
            }
        }
        println!("Exited event loop");
    });

    let mut data = 0;
    loop {
        if sender.send(data).await.is_err() {
            println!("Receiver of a channel is disconnected - the channel is closed");
            break;
        }
        data += 1;

        if data > 50 {
            println!("All data sended!");
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    drop(sender);
    receiver_join.await.unwrap();
}

// using tokio task and cancellation token
async fn cancellation_token_tasks() {
    let (sender, receiver) = tokio::sync::mpsc::channel::<i32>(16);
    let cancellation_token = tokio_util::sync::CancellationToken::new();

    let token_clone = cancellation_token.child_token();

    let receiver_join = tokio::spawn(async move {
        let mut receiver = receiver;
        loop {
            tokio::select! {
                Some(num) = receiver.recv() => {
                    println!("Received: {num}");
                },
                () = token_clone.cancelled() => {
                    println!("Cancel token was cancelled");
                    break;
                }
            };
        }
        println!("Exited event loop");
    });

    let mut data = 0;
    loop {
        if sender.send(data).await.is_err() {
            println!("Receiver of a channel is disconnected - the channel is closed");
            break;
        }
        data += 1;

        if data > 50 {
            println!("All data sended!");
            cancellation_token.cancel();
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    drop(sender);
    receiver_join.await.unwrap();
}

#[tokio::main]
async fn main() {
    closing_scoped_threads();
    closing_joined_threads();
    closing_tasks().await;
    cancellation_token_tasks().await;
}
