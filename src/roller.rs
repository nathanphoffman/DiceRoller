use futures::future::join_all;
use rand::{Rng, rngs::ThreadRng};
use std::{ops::Range, sync::Arc};

use crate::utility::round_to_one_place;

pub fn roll_dice(number: u32, sides: u32) -> u32 {
    let mut rng: ThreadRng = rand::rng();
    let mut total: u32 = 0;

    for _ in 0..number {
        let die_roll: u32 = rng.random_range(1..=sides);
        total += die_roll;
    }

    return total;
}

pub fn get_average(results: &Vec<u32>) -> f64 {
    let sum_results: u32 = results.iter().sum();
    let num_results: f64 = results.iter().count() as f64;
    let avg = f64::from(sum_results) / num_results;
    return round_to_one_place(avg);
}

pub fn get_sum(results: &Vec<u32>) -> u32 {
    let sum_results: u32 = results.iter().sum();
    return sum_results;
}
// I should do these with a reduce (Rust calls that folding)
pub fn combine_results_add(results: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut combined_results: Vec<u32> = Vec::new();

    if results.len() == 0 {
        return combined_results;
    }

    let num_results: usize = results[0].len();

    for i in 0..num_results {
        let mut total: u32 = 0;
        for result_set in results {
            total += result_set[i];
        }
        combined_results.push(total);
    }

    return combined_results;
}

pub fn combine_results_subtract(results: &Vec<Vec<u32>>) -> Vec<u32> {
    let mut combined_results: Vec<u32> = Vec::new();

    if results.len() == 0 {
        return combined_results;
    }

    let num_results: usize = results[0].len();

    for i in 0..num_results {
        let mut total: i32 = results[0][i] as i32;
        for result_set in &results[1..] {
            total -= result_set[i] as i32;
        }
        combined_results.push(total.max(0) as u32); // Ensure no negative results
    }

    return combined_results;
}

pub async fn make_dice_rolls(
    interval: u32,
    batch: u32,
    number: u32,
    sides: u32,
) -> Vec<u32> {
    let batch_size: Range<u32> = 0..batch; // Create a vector of interval sizes

    let tasks: Vec<_> = (0..interval)
        .map(|_| {

            let batch_size_clone = batch_size.clone();

            // batch together multiple die results into each task so that each task does more than one roll
            tokio::spawn(async move {
                let results: Vec<u32> = batch_size_clone
                    .map(|_| roll_dice(number, sides))
                    .collect();
                return results;
            })
        })
        .collect();

    // Wait for tasks to complete
    let final_results: Vec<u32> = join_all(tasks)
        .await
        .into_iter()
        .filter_map(|result| result.ok()) 
        .flat_map(|result| result) 
        .collect();

        return final_results;

}
