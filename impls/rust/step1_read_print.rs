use std::io::{self, Write};

use crate::printer;
use crate::reader;

fn eval(arg: String) -> String {
    arg
}

fn rep(arg: String) -> String {
    printer::print_str(eval(reader::read_str(arg)))
}

fn main() {
    loop {
        let mut input = String::new();

        print!("user> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                print!("{}", rep(input));
            }
            Err(_) => println!("error"),
        }
    }
}
