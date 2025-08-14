use std::io::{self, Write};
use rand::{rngs::ThreadRng, Rng};

mod command;
mod roller;
mod utility;

fn main() {
    println!("Type dice command like '2d6' to roll dice.");
    println!("'table %command%' will display a balanced table of preformed balanced outputs");
    println!("'average %command%' will run permutations and output the average");
    println!("'results %command%' will run permutations and output the odds of each result");
    println!("'atleast %command%' will run permutations and output the odds of the lowest totals");
    println!("'atmost %command%' will run permutations and output the odds of the highest totals");

    io::stdout().flush().unwrap();

    //println!("{}", roll_die(3));

    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(command::execute_command("10d20"));

     if result.is_ok() {
         println!("program ran successfully");
     }

    // let mut input = String::new();

    //     io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");
}

