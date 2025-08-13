use regex::Regex;

fn get_command_fn(command: &str) {

}

fn is_command_valid(command: &str) -> bool {
    let regex = Regex::new(r"^([0-9]+)?d[0-9]+").unwrap(); // Matches two words
    return regex.is_match(command);
}

pub fn execute_command(command: &str) -> Result<&'static str, &'static str> {

    if is_command_valid(command) {
        return Ok("Command is ok");
    }
    else {
        return Err("Command is not well formed");
    }

}
