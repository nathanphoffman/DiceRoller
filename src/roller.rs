use rand::{Rng, rngs::ThreadRng};

pub fn roll_dice(number: i32, sides: i32) -> i32 {
    let mut rng: ThreadRng = rand::rng();
    let mut total: i32 = 0;

    for _ in 0..number {
        let die_roll: i32 = rng.random_range(1..=sides);
        println!("die roll is {}", die_roll);
        total += die_roll;
    }

    return total;
}

pub async fn average(max_runs: i32, no_change_tolerance: i32, number: i32, sides: i32) {

    let task1 = tokio::spawn(async {
        // Simulate some asynchronous work

        println!("Task 1 is running");

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        println!("Task 1 is complete");
    });

    // Wait for tasks to complete
    let _ = tokio::join!(task1);
}
