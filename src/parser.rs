pub type Number = f64;

pub enum Expr<'a> {
    Num(Number),
    Word(&'a str),
}

pub fn parse(token: &str) -> Expr {
    use Expr::*;
    match token.parse::<Number>() {
        Ok(val) => Num(val),
        Err(_) => Word(token),
    }
}
