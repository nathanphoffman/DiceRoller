fn roll_die(sides: i32) -> i32 {
    let mut rng: ThreadRng = rand::rng();
    let random_number: i32 = rng.random_range(1..=sides);
    return random_number;
}

fn roll_results(command: &str, times: i32) {
    
}
