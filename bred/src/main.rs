mod term;

use std::io;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match term::Term::from_str(&input) {
                Ok(t) => {
                    println!("{}", term::reduce(&t));
                },
                Err(error) => {
                    println!("error: {:?}", error);
                }
            }
        }
        Err(error) => {
            println!("error: {}", error);
        }
    }
}