#[allow(dead_code)]
enum Expr {
    Number(i32),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(lhs, rhs) => eval(lhs) + eval(rhs),
        Expr::Subtract(lhs, rhs) => eval(lhs) - eval(rhs), 
        Expr::Multiply(lhs, rhs) => eval(lhs) - eval(rhs), 
        Expr::Divide(lhs, rhs) => eval(lhs) - eval(rhs), 
    }
}

fn main() {
    let expr = Expr::Add(
        Box::new(Expr::Multiply(
                Box::new(Expr::Number(3)),
                Box::new(Expr::Number(4)),
        )),
        Box::new(Expr::Number(5)),
    );

    println!("Result: {}", eval(&expr));
}
