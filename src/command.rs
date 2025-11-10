use regex::Regex;

use crate::roller::{self, get_average, get_sum, make_dice_rolls};

fn get_command_dice(command: &str) -> (u32, u32) {
    let components: Vec<&str> = command.split('d').collect();

    let number_of_dice: u32 = components[0].parse().unwrap();
    let sides_of_dice: u32 = components[1].parse().unwrap();

    return (number_of_dice, sides_of_dice);
}

fn is_dice_command_valid(command: &str) -> bool {
    let regex = Regex::new(r"^([0-9]+)?d[0-9]+").unwrap(); // Matches two words
    return regex.is_match(command);
}

fn validate_and_get_commands(command: &str) -> Option<(&str, &str)> {
    let components: Vec<&str> = command.split(' ').collect();

    if components.len() < 2 {
        return Some(("", command));
    }

    let comp1: &str = components[0];
    let comp2: &str = components[1];

    println!("{}, {}", &comp1, &comp2);

    let keywords: [&'static str; 3] = ["average", "results", "table"];

    if keywords.contains(&comp1) {
        return Some((&comp1, &comp2));
    } else {
        return None;
    }
}

pub async fn execute_command(command: &str) -> Result<&'static str, &'static str> {
    let commands = validate_and_get_commands(command);

    if commands != None {
        //let plusComponents: Vec<&str> = command.split('+').collect();

        let keyword = commands.unwrap().0;
        let command = commands.unwrap().1;

        let plus_commands: Vec<&str> = command.split('+').collect();
   
        let mut positive_results: Vec<Vec<u32>> = Vec::new();
        let mut negative_results: Vec<Vec<u32>> = Vec::new();

        for plus_command in plus_commands {

            // if someone puts a + leading a die command with nothing before it 
            // it will result in an empty string first command sequence
            if plus_command == "" {
                continue;
            }

            let minus_commands: Vec<&str> = plus_command.split('-').collect();
            let minus_commands_first_element = minus_commands[0];

            for minus_command in minus_commands {

                // the first command if not an empty string must be the plus command that was split on
                // if it is an empty string it must mean this is a minus or series of minus commands
                if (minus_command == minus_commands_first_element) && minus_command != "" {
                    positive_results.push(execute_dice_command(minus_command).await);
                    continue;
                }

                negative_results.push(execute_dice_command(minus_command).await);
            }

        let mut final_results: Vec<u32> = roller::combine_results_add(&positive_results);
        let negative_combined_results: Vec<u32> = roller::combine_results_add(&negative_results);
       
        let final_sum: u32 = get_sum(&final_results);
        
    
            println!("The length is {}", final_results.len());
            println!("{:?}", final_results);
        }
/* */
        

        return Ok("Command is ok");
    } else {
        return Err("Command is not well formed");
    }
}

pub async fn execute_dice_command(command: &str) -> Vec<u32> {
    let die_commands = get_command_dice(command);
    println!("The dice are, {}, {}", die_commands.0, die_commands.1);
    let results = make_dice_rolls(100, 100, die_commands.0, die_commands.1).await;
    let avg = get_average(&results);
    println!("The average is {}", avg);
    let sum = get_sum(&results);
    println!("The sum is {}", sum);

    return results;
}
