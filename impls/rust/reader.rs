use regex::{Regex, Captures};
use crate::types::*;

struct Reader {
    tokens: Vec<String>,
    position: i32,
}

impl Reader {
    fn next(&mut self) -> String {
        let current_position = self.position;
        self.position = self.position + 1;

        return self.tokens[current_position];
    }
    fn peek(&mut self) -> String {
        return self.tokens[self.position];
    }
}

pub fn read_str(code_str: String) {
    let tokens = tokenize(code_str);

    let reader = Reader {
        tokens,
        position: 0,
    };

    read_form(reader);
}

fn tokenize(&code_str: String) -> Captures {
    let regex =
        Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"#)
            .unwrap();

    return regex.captures(code_str).unwrap();
}

fn read_form(reader: Reader) -> MalType {
    let token = reader.peek();

    if token == "(" {
        read_list(reader);
    } else {
        read_atom(token);
    };
}
fn read_list(reader: Reader) -> Reader {
    let mut next_token = reader.next();
    let mut list: MalList = vec![[]];

    while next_token != ")" {
        list.push(read_form(reader));
        reader.next();
    };

    reader
}
fn read_atom(token: String) {
    let num = token.parse::<MalNumber>();

    match num {
        Ok(num) => num,
        Err(_) => MalSymbol { name: token },
    };
}
