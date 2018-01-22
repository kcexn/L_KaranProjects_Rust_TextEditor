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
