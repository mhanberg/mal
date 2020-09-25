use std::io::{self, Write};

fn read(arg: String) -> String {
    arg
}

fn eval(arg: String) -> String {
    arg
}

fn print(arg: String) -> String {
    arg
}

fn rep(arg: String) -> String {
    print(eval(read(arg)))
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
