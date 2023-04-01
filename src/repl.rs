use std::io::stdout;
use std::io::{self, Write};

use crate::token::{Lexer, Token};

#[derive(Debug)]
pub struct Repl {}

impl Repl {
    pub fn start() {
        loop {
            print!("mon >>: ");
            stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let lexer = Lexer::new(&input);
                    let result = lexer.into_iter().collect::<Vec<Token>>();
                    println!("{:?}", result);
                }
                Err(error) => println!("error: {}", error),
            }
            stdout().flush().unwrap();
        }
    }

    fn print_tokens(tokens: Vec<Token>) {
        unimplemented!()
    }
}
