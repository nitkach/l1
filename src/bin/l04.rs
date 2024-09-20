use std::time::Duration;

/// Implement permanent data recording in the channel (main stream).
/// Implement a set of N workers that read arbitrary data from a channel
/// and output it to stdout.
/// It is necessary to be able to select the number of workers at the start.

async fn task_tokio_broadcast(workers: usize) {
    let (sender, _) = tokio::sync::broadcast::channel::<i32>(16);
    for worker_number in 1..=workers {
        let receiver = sender.subscribe();
        tokio::spawn(create_worker(worker_number, receiver));
    }

    let mut data = 0;
    loop {
        if sender.send(data).is_err() {
            break;
        }
        data += 1;
        tokio::time::sleep(Duration::from_millis(200)).await;
        if data > 50 {
            break;
        }
    }

    drop(sender);

    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn create_worker(worker_number: usize, mut receiver: tokio::sync::broadcast::Receiver<i32>) {
    loop {
        let message = receiver.recv().await;

        match message {
            Ok(num) => {
                println!("Worker №{worker_number} received {num}");
            }
            Err(err) => {
                println!("Worker №{worker_number} encountered error: {err}");
                break;
            }
        };
    }
}

#[tokio::main]
async fn main() {
    let workers = 5;
    task_tokio_broadcast(workers).await;
}
