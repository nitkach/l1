use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};

/// Implement a counter structure that will be incremented in a competitive environment.
/// Upon completion, the program should output the final value of the counter.

struct AtomicCounter(AtomicU32);

impl AtomicCounter {
    fn new(number: u32) -> Self {
        Self(AtomicU32::new(number))
    }

    fn increase(&self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }

    fn get_value(self) -> u32 {
        self.0.into_inner()
    }
}

async fn task(workers_number: usize) {
    let counter = Arc::new(AtomicCounter::new(0));

    let mut joins = Vec::new();

    for i in 1..=workers_number {
        let counter = Arc::clone(&counter);
        let join = tokio::spawn(async move {
            for _ in 0..100 {
                counter.increase();
                println!("The value is increased by the worker {i}");
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        });

        joins.push(join);
    }

    for join in joins {
        join.await.unwrap();
    }

    if let Some(counter) = Arc::into_inner(counter) {
        println!(
            "Counter value after increasing from all workers: {}",
            counter.get_value()
        );
    } else {
        eprint!("Cannot acquire counter from Arc");
    }
}

#[tokio::main]
async fn main() {
    task(10).await;
}
