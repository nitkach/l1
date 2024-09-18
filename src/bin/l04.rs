use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

/// Implement permanent data recording in the channel (main stream).
/// Implement a set of N workers that read arbitrary data from a channel
/// and output it to stdout.
/// It is necessary to be able to select the number of workers at the start.

fn main() {
    let num_workers = 5;
    let (sender, receiver) = crossbeam_channel::unbounded::<i32>();

    for worker_number in 0..num_workers {
        let receiver = receiver.clone();
        thread::spawn(move || loop {
            let message = receiver.recv();

            if let Ok(num) = message {
                println!("Worker {worker_number} received {num}");
            } else {
                println!("Channel has been closed, worker {worker_number} stopping");
                break;
            }
        });
    }

    let mut data = 0;
    loop {
        sender.send(data).unwrap();
        data += 1;
        thread::sleep(Duration::from_millis(200));
        if data > 50 {
            break;
        }
    }

    drop(sender);

    thread::sleep(Duration::from_secs(2));
}

fn foo() {
    let num_workers = 20;
    let (sender, receiver) = std::sync::mpsc::channel::<i32>();
    let receiver = Arc::new(Mutex::new(receiver));

    for worker_number in 0..num_workers {
        let receiver = receiver.clone();
        thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            if let Ok(num) = message {
                println!("Worker {worker_number} received {num}");
            } else {
                println!("Channel has been closed, worker {worker_number} stopping");
                break;
            }
        });
    }

    let mut data = 0;
    loop {
        sender.send(data).unwrap();
        data += 1;
        thread::sleep(Duration::from_millis(200));
        if data > 50 {
            break;
        }
    }

    drop(sender);

    thread::sleep(Duration::from_secs(2));
}
