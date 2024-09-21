use std::time::Duration;
use tokio::signal::ctrl_c;

/// The program should end neatly by pressing Ctrl+C.
/// Select and justify a way to complete the work of all workers.
/// Use tokio and flume (or another similar library for spmc/mpmc channels).

#[tokio::main]
async fn main() {
    let workers_number = 10;
    let mut workers_joins = Vec::new();
    let (sender, _) = tokio::sync::broadcast::channel::<i32>(16);

    // spawning workers
    for i in 0..workers_number {
        // creating receiver
        let mut worker_receiver = sender.subscribe();
        let join = tokio::spawn(async move {
            loop {
                tokio::select! {
                    result = worker_receiver.recv() => {
                        match result {
                            Ok(num) => {
                                println!("Worker {i} received: {num}");
                            }
                            Err(err) => {
                                println!("{err}");
                                break;
                            }
                        }
                    },
                    // awaiting for Ctrl+C signal
                    _ = ctrl_c() => {
                        println!("Ctrl+C signal received: worker {i} stopped.");
                        break;
                    }
                }
            }
        });
        workers_joins.push(join);
    }

    for i in 1.. {
        if sender.send(i).is_err() {
            println!("Receiver of a channel is disconnected - the channel is closed");
            break;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

    drop(sender);
    for join in workers_joins {
        join.await.unwrap();
    }
}
