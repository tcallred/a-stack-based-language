use std::io::{stdin, stdout, Write};
use colored::*;

type Value = f64;

enum Parsed<'a>{
    Val(Value),
    Word(&'a str)
}

#[derive(Debug)]
struct Stack {
    rep: Vec<Value>
}

impl Stack {
    fn new() -> Self {
        Stack { rep: Vec::with_capacity(10) }
    }
    fn push(mut self, val: Value) -> Self {
        self.rep.push(val);
        self
    }
    fn peek(&self, n: usize) -> Option<&Value> {
        self.rep.get(self.rep.len() - 1 - n)
    }
    fn pop(mut self) -> (Option<Value>, Self) {
        let val = self.rep.pop();

        (val, self)
    }
    fn execute(self, word: &str) -> Self {
        use Parsed::*;

        let parsed = match word.parse::<Value>() {
            Ok(val) => Val(val),
            Err(_) => Word(word),
        };

        match parsed {
            Val(val) => self.push(val),
            Word(w) => self.execute_word(w)
        }
        
    }
    fn execute_word(self, word: &str) -> Self {
        match word {
            "+" => {
                if self.rep.len() < 2 {
                    eprintln!("{}", format!("`{}` requires two arguments.", word).red());
                    return self;
                }
                let (v1, stack1) = self.pop(); 
                let (v2, stack2) = stack1.pop();
                stack2.push(v1.unwrap() + v2.unwrap())
            }
            _ => {
                eprintln!("{}", format!("Unrecognized word `{}`", word).red());
                self
            }
        }
    }

}

fn execute_ln(ln: &str) -> Value {
    let stack = Stack::new();
    let words = ln.split(' ');

    let new_stack = words.fold(stack, |s, w| s.execute(w));

    println!("{:?}", new_stack);
    
    new_stack.peek(0).unwrap().clone()
}

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
            let res = execute_ln(trimmed_line);
            println!("{}", res);
        }

        line.clear();
    }
}

fn main() {
    println!("{}\n", "A stack based programming language. Enjoy!".blue());
    run_prompt();
    println!();
    println!("{}", "Goodbye.".blue());
}
