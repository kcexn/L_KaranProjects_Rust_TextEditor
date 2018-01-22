extern crate utils;
extern crate utils_structs;

use utils_structs::TextData;

fn main() {
    let mut text = TextData::new("", vec![]);
    loop {
        let mut input = String::new();
        // Read from stdin and store in input.
        // This will panic if STDIN fails for some reason.
        utils::read_from_stdin(&mut input);    
        // set the input of the TextData structure to the STDIN input
        text.set_input(&input); 

        // Check for various commands and execute it
        // append text to the end of the text array
        // if no commands provided
        utils::check_for_commands(&mut text);
    }
}
