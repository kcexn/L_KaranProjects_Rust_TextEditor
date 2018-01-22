extern crate utils_structs;

use std::io;
use std::process;
use utils_structs::TextData; 

pub fn read_from_stdin(mut input: &mut String) {
   match io::stdin().read_line(&mut input) {
       Ok(_n) => (),
       Err(error) => {
           println!("error: {}", error);
           panic!();
       },
   }
}

pub fn check_for_commands(input: &mut TextData){
    match input.get_input() {
        ":q\n" => exit(),
        ":p\n" => print(input.get_file()),
        ":e\n" => {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).expect("failed to read line!");
            // Need to refactor this to handle inputs properly
            let line_no: usize = input_line
                .trim()
                .parse()
                .expect("Expecting a number!");
            input.set_write_head(line_no);
            input.print_write_line(line_no); 
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).expect("failed to read line!");
            input.edit_line(&input_line);
        },
        _ => input.append_to_file(),
    }
}
                

fn exit(){
    process::exit(0);
}

fn print(input: &Vec<String>) {
    for ( i, line ) in input.iter().enumerate() {
        print!("{} {}", i, line ); 
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
