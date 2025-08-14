use futures::future::join_all;
use rand::{Rng, rngs::ThreadRng};
use std::sync::Arc;

pub fn roll_dice(number: u32, sides: u32) -> u32 {
    let mut rng: ThreadRng = rand::rng();
    let mut total: u32 = 0;

    for _ in 0..number {
        let die_roll: u32 = rng.random_range(1..=sides);
        println!("die roll is {}", die_roll);
        total += die_roll;
    }

    return total;
}

pub async fn average(
    max_runs: u32,
    interval: u32,
    no_change_tolerance: u32,
    number: u32,
    sides: u32,
) {
    let interval_size: Vec<u32> = (0..interval).collect(); // Create a vector of interval sizes
    let interval_size_arc = Arc::new(interval_size);

    let tasks: Vec<_> = (0..interval)
        .map(|_| {
            let interval_size_clone: Arc<Vec<u32>> = Arc::clone(&interval_size_arc); // Clone the Arc for each task

            tokio::spawn(async move {
                let results = &interval_size_clone.iter().map(|_| roll_dice(number, sides));
            })
        })
        .collect();

    // Wait for tasks to complete
    let _ = join_all(tasks).await;
}
