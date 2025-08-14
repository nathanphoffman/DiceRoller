use regex::Regex;

use crate::roller::{self, make_dice_rolls, get_average};
use crate::utility::{round_two_decimal};

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
  
    let comp1 : &str = components[0];
    let comp2 : &str = components[1];

    println!("{}, {}", &comp1, &comp2);

    let keywords: [&'static str; 3] = ["average", "results", "table"];

    if keywords.contains(&comp1) {
        return Some((&comp1, &comp2));
    }
    else {
        return None;
    }

}

pub async fn execute_command(command: &str) -> Result<&'static str, &'static str> {

    let commands = validate_and_get_commands(command);

    if commands != None {

        let keyword = commands.unwrap().0;
        let command = commands.unwrap().1;

        let die_commands = get_command_dice(command);
        println!("The dice are, {}, {}", die_commands.0, die_commands.1);

        let results = make_dice_rolls(1, 100, 10_000, 0, die_commands.0, die_commands.1).await;
        let avg = get_average(results);
        println!("The average is {}", round_two_decimal(avg));

        //let total = roller::roll_dice(die_commands.0, die_commands.1);
        //println!("The result of the die roll is, {}", total);
    
        return Ok("Command is ok");
    }
    else {
        return Err("Command is not well formed");
    }

}
