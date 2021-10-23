use colored::*;
use std::io::{stdin, stdout, Write};

type Value = f64;
// type MonadicFn = fn(Value) -> Value;
type DyadicFn = fn(Value, Value) -> Value;

enum Expr<'a> {
    Val(Value),
    Word(&'a str),
}

fn add(v1: Value, v2: Value) -> Value {
    v1 + v2
}

fn subtract(v1: Value, v2: Value) -> Value {
    v2 - v1
}

fn multiply(v1: Value, v2: Value) -> Value {
    v1 * v2
}

fn divide(v1: Value, v2: Value) -> Value {
    v2 / v1
}

#[derive(Debug)]
struct Stack {
    rep: Vec<Value>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            rep: Vec::with_capacity(10),
        }
    }
    fn push(mut self, val: Value) -> Self {
        self.rep.push(val);
        self
    }
    fn peek(&self, n: usize) -> Option<&Value> {
        self.rep.iter().rev().nth(n)
    }
    fn pop(mut self) -> (Option<Value>, Self) {
        let val = self.rep.pop();

        (val, self)
    }
    fn execute(self, expr: Expr) -> Self {
        use Expr::*;

        match expr {
            Val(val) => self.push(val),
            Word(w) => self.execute_word(w),
        }
    }
    fn execute_word(self, word: &str) -> Self {
        match word {
            "+" => self.execute_dyadic(word, add),
            "-" => self.execute_dyadic(word, subtract),
            "*" => self.execute_dyadic(word, multiply),
            "/" => self.execute_dyadic(word, divide),
            "right" => self.right(),
            "left" => self.left(),
            "commute" => self.commute(),
            _ => {
                eprintln!("{}", format!("Unrecognized word `{}`", word).red());
                self
            }
        }
    }
    // fn execute_monadic(self, word: &str, f: MonadicFn) -> Self {
    //     if self.rep.len() < 1 {
    //         eprintln!("{}", format!("`{}` requires one argument.", word).red());
    //         eprintln!("{}", format!("The stack: {:?}", self.rep).red());

    //         return self;
    //     }
    //     let (v1, stack1) = self.pop();
    //     stack1.push(f(v1.unwrap()))
    // }
    fn execute_dyadic(self, word: &str, f: DyadicFn) -> Self {
        if self.rep.len() < 2 {
            eprintln!("{}", format!("`{}` requires two arguments.", word).red());
            eprintln!("{}", format!("The stack: {:?}", self.rep).red());

            return self;
        }
        let (v1, stack1) = self.pop();
        let (v2, stack2) = stack1.pop();
        stack2.push(f(v1.unwrap(), v2.unwrap()))
    }
    fn commute(self) -> Self {
        if self.rep.len() < 2 {
            return self;
        }
        let (v1, stack1) = self.pop();
        let (v2, stack2) = stack1.pop();
        stack2.push(v1.unwrap()).push(v2.unwrap())
    }
    fn right(self) -> Self {
        if self.rep.len() < 1 {
            return self;
        }
        let val = self.peek(0).unwrap().clone();
        self.push(val)
    }
    fn left(self) -> Self {
        if self.rep.len() < 2 {
            return self;
        }
        let val = self.peek(1).unwrap().clone();
        self.push(val)
    }
}

fn parse(token: &str) -> Expr {
    use Expr::*;
    match token.parse::<Value>() {
        Ok(val) => Val(val),
        Err(_) => Word(token),
    }
}

fn execute_ln(ln: &str) {
    let stack = Stack::new();
    let tokens = ln.split(' ');
    let exprs = tokens.map(parse);

    let new_stack = exprs.fold(stack, |s, e| s.execute(e));

    let mut i = 0;
    while let Some(val) = new_stack.peek(i) {
        println!("{}", format!("{}", val).purple());
        i += 1;
    }
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
            execute_ln(trimmed_line);
        }

        line.clear();
    }
}

fn main() {
    println!("{}\n", "A stack based programming language. Enjoy!".blue());
    run_prompt();
    println!("{}", "Goodbye.".blue());
}
