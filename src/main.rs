use a_stack_based_language::interpreter::*;
use colored::*;
use std::io::{stdin, stdout, Write};
use a_stack_based_language::tokenizer::*;

fn run_prompt() {
    let mut line = String::new();

    loop {
        print!("{}", "> ".green().bold());
        let _ = stdout().flush().unwrap();

        stdin().read_line(&mut line).unwrap();

        let trimmed_line = line.trim();
        if trimmed_line.len() == 0 {
            break;
        } else {
            for token in tokenize(String::from(trimmed_line)).iter() {
                println!("{:?}", token);
            }
        }

        line.clear();
    }
}

fn main() {
    println!("{}\n", "A stack based programming language. Enjoy!".blue());
    run_prompt();
    println!("{}", "Goodbye.".blue());
}
