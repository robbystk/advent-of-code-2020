fn input() -> String {
    let input_filename = &std::env::args().collect::<Vec<String>>()[1];
    std::fs::read_to_string(input_filename).unwrap()
}

#[derive(Debug)]
enum Token {
    OParen,
    CParen,
    Plus,
    Times,
    Value(i32)
}

fn tokenize_line(input: &str) -> Vec<Token> {
    use Token::*;

    let mut rv = Vec::new();
    for c in input.chars() {
        match c {
            ' ' => {},
            '(' => rv.push(OParen),
            ')' => rv.push(CParen),
            '+' => rv.push(Plus),
            '*' => rv.push(Times),
            '0'..='9' => rv.push(Value(c as i32 - 0x30)),
            _ => panic!("invalid character: `{}`", c)
        }
    }
    return rv;
}

#[derive(Debug)]
enum Expression {
    Value(i32),
    Operation(Box<Operation>)
}

#[derive(Debug)]
enum Operator {
    Plus,
    Times
}

#[derive(Debug)]
struct Operation {
    oper: Operator,
    lhs: Expression,
    rhs: Expression
}

fn main() {
    let input = input();
    let mut expressions = Vec::new();
    for line in input.lines() {
        expressions.push(tokenize_line(line));
    }
    println!("{:?}", expressions);
}
