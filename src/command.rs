use regex::Regex;

fn get_command_dice(command: &str) -> (i32, i32) {

    let components: Vec<&str> = command.split('d').collect();

    let number_of_dice = components[0].parse().unwrap();
    let sides_of_dice = components[1].parse().unwrap();

    return (number_of_dice, sides_of_dice);
}

fn is_dice_command_valid(command: &str) -> bool {
        let regex = Regex::new(r"^([0-9]+)?d[0-9]+").unwrap(); // Matches two words
        return regex.is_match(command);
}

fn is_command_valid(command: &str) -> bool {

    let components: Vec<&str> = command.split(' ').collect();

    if components.len() < 2 {
        return is_dice_command_valid(command);
    }
  
    return match components[0] {
        "average" => is_dice_command_valid(components[1]),
        "sum" => is_dice_command_valid(components[1]),
        "results" => is_dice_command_valid(components[1]),
        "table" => is_dice_command_valid(components[1]),
        _ => false
    }

}

pub fn execute_command(command: &str) -> Result<&'static str, &'static str> {

    if is_command_valid(command) {

        let dice = get_command_dice(command);
        println!("The dice are, {}, {}", dice.0, dice.1);

        return Ok("Command is ok");
    }
    else {
        return Err("Command is not well formed");
    }

}
