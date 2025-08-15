use futures::future::join_all;
use rand::{Rng, rngs::ThreadRng};
use std::{ops::Range, sync::Arc};

pub fn roll_dice(number: u32, sides: u32) -> u32 {
    let mut rng: ThreadRng = rand::rng();
    let mut total: u32 = 0;

    for _ in 0..number {
        let die_roll: u32 = rng.random_range(1..=sides);
        total += die_roll;
    }

    return total;
}

pub fn get_average(results: Vec<u32>) -> f64 {
    let sum_results: u32 = results.iter().sum();
    let num_results: f64 = results.iter().count() as f64;
    return f64::from(sum_results) / num_results;
}

pub async fn make_dice_rolls(
    max_runs: u32,
    interval: u32,
    batch: u32,
    no_change_tolerance: u32,
    number: u32,
    sides: u32,
) -> Vec<u32> {
    let batch_size: Range<u32> = 0..batch; // Create a vector of interval sizes
    let number_clone = number;
    let sides_clone = sides;

    let tasks: Vec<_> = (0..interval)
        .map(|_| {

            let batch_size_clone = batch_size.clone();

            // batch together multiple die results into each task so that each task does more than one roll
            tokio::spawn(async move {
                let results: Vec<u32> = batch_size_clone
                    .map(|_| roll_dice(number_clone, sides_clone))
                    .collect();
                return results;
            })
        })
        .collect();

    // Wait for tasks to complete
    let final_results: Vec<u32> = join_all(tasks)
        .await
        .into_iter()
        .filter_map(|result| result.ok()) // Filter out any errors
        .flat_map(|result| result) // Flatten the results into a single vector
        .collect();

        return final_results;

}
