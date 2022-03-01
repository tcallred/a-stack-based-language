use crate::parser::*;
use colored::*;
use ndarray::arr2;
use ndarray::Array2;
use ndarray::{array, s};

use crate::parser::parse;

type Value = Array2<Number>;
pub type MonadicFn = fn(Value) -> Value;
type DyadicFn = fn(Value, Value) -> Value;
type StackFn = fn(Vec<Value>) -> Value;

// Monadic Fns ------------------------
fn negate(v: Value) -> Value {
    v * -1.0
}

fn reverse(v: Value) -> Value {
    v.slice_move(s![0.., 0..;-1])
}

fn length(v: Value) -> Value {
    array![[v.len() as Number]]
}

fn sum(v: Value) -> Value {
    array![[v.sum()]]
}

fn product(v: Value) -> Value {
    array![[v.product()]]
}

fn iota(v: Value) -> Value {
    let n = v[[0, 0]] as usize;
    let mut arr = ndarray::Array2::<Number>::zeros((1, n));
    for i in 1..=n {
        arr[[0, i - 1]] = i as Number;
    }

    arr
}

// Dyadic Fns -------------------------
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

// All Stack Fns -------------------------
fn concat_array(values: Vec<Value>) -> Value {
    let mut new_arr = ndarray::Array2::<Number>::zeros((1, values.len()));
    for (i, v) in values.iter().enumerate() {
        new_arr[[0, i]] = v[[0, 0]];
    }
    return new_arr;
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
            Num(val) => self.push(arr2(&[[val]])),
            Word(w) => self.execute_word(w),
        }
    }
    fn execute_word(self, word: &str) -> Self {
        match word {
            "+" => self.execute_dyadic(word, add),
            "-" => self.execute_dyadic(word, subtract),
            "*" => self.execute_dyadic(word, multiply),
            "/" => self.execute_dyadic(word, divide),
            "negate" | "neg" => self.execute_monadic(word, negate),
            "reverse" | "rev" => self.execute_monadic(word, reverse),
            "length" | "len" => self.execute_monadic(word, length),
            "sum" => self.execute_monadic(word, sum),
            "product" | "prod" => self.execute_monadic(word, product),
            "iota" => self.execute_monadic(word, iota),
            "right" | "dup" => self.right(),
            "left" => self.left(),
            "commute" => self.commute(),
            "|" => self.execute_all(concat_array),
            _ => {
                eprintln!("{}", format!("Unrecognized word `{}`", word).red());
                self
            }
        }
    }
    fn execute_monadic(self, word: &str, f: MonadicFn) -> Self {
        if self.rep.len() < 1 {
            eprintln!("{}", format!("`{}` requires one argument.", word).red());
            eprintln!("{}", format!("The stack: {:?}", self.rep).red());

            return self;
        }
        let (v1, stack1) = self.pop();
        stack1.push(f(v1.unwrap()))
    }
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
    fn execute_all(self, f: StackFn) -> Self {
        let mut stack = self;
        let mut vals = vec![];
        while stack.rep.len() > 0 && stack.peek(0).unwrap().len() == 1 {
            let (v, new_stack) = stack.pop();
            vals.push(v.unwrap());
            stack = new_stack;
        }
        stack.push(f(vals.into_iter().rev().collect()))
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

pub fn execute_ln(ln: &str) {
    let stack = Stack::new();
    let tokens = ln.split_whitespace();
    let exprs = tokens.map(parse);

    let new_stack = exprs.fold(stack, |s, e| s.execute(e));

    for val in new_stack.rep.iter().rev() {
        if val.len() == 1 {
            println!("{}", format!("{}", val[[0, 0]]).purple())
        } else {
            println!("{}", format!("{}", val).purple());
        }
    }
}
