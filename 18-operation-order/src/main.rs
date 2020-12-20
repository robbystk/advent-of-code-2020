fn input() -> String {
    let input_filename = &std::env::args().collect::<Vec<String>>()[1];
    std::fs::read_to_string(input_filename).unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Token {
    OParen,
    CParen,
    Operator(Operator),
    Value(u64)
}

fn tokenize_line(input: &str) -> Vec<Token> {
    let mut rv = Vec::new();
    for c in input.chars() {
        match c {
            ' ' => {},
            '(' => rv.push(Token::OParen),
            ')' => rv.push(Token::CParen),
            '+' => rv.push(Token::Operator(Operator::Plus)),
            '*' => rv.push(Token::Operator(Operator::Times)),
            '0'..='9' => rv.push(Token::Value(c as u64 - 0x30)),
            _ => panic!("invalid character: `{}`", c)
        }
    }
    return rv;
}

#[derive(Debug)]
enum Expression {
    Value(u64),
    Operation(Box<Operation>)
}

impl Expression {
    fn eval(self: &Expression) -> u64 {
        use Expression::*;

        match self {
            Value(n) => *n,
            Operation(oper) => (*oper).eval()
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Operator {
    Plus,
    Times
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    lhs: Expression,
    rhs: Expression
}

impl Operation {
    fn eval(self: &Operation) -> u64 {
        use Operator::*;

        match self.operator {
            Plus => self.lhs.eval() + self.rhs.eval(),
            Times => self.lhs.eval() * self.rhs.eval()
        }
    }
}

fn parse_expression(tokens: &Vec<Token>) -> Expression {
    let mut tokens = tokens.iter().peekable();
    let mut next_token = tokens.peek();
    let mut expression_stack = Vec::new();
    let mut token_stack = Vec::new();
    while next_token.is_some() {
        let current_token = tokens.next();
        next_token = tokens.peek();

        match current_token {
            Some(Token::Value(n)) => {
                match token_stack.pop() {
                    Some(Token::Operator(oper)) => {
                        let lhs = expression_stack.pop().unwrap();
                        expression_stack.push(Expression::Operation(Box::new(Operation {
                            operator: oper,
                            lhs: lhs,
                            rhs: Expression::Value(*n)
                        })));
                    },
                    None => {
                        expression_stack.push(Expression::Value(*n))
                    },
                    Some(Token::OParen) => {
                        token_stack.push(Token::OParen);
                        expression_stack.push(Expression::Value(*n));
                    }
                    Some(Token::Value(_)) | Some(Token::CParen) => panic!("syntax error")
                }
            }
            Some(Token::Operator(oper)) => {
                token_stack.push(Token::Operator(*oper));
            },
            Some(Token::OParen) => {
                token_stack.push(Token::OParen);
            },
            Some(Token::CParen) => {
                match token_stack.pop() {
                    Some(Token::OParen) => {},
                    _ => panic!("Syntax error: unmatched parentheses")
                }

                let mut last_token = token_stack.last().map(|t| *t);
                loop {
                    match last_token {
                        Some(Token::Operator(oper)) => {
                            token_stack.pop();
                            let rhs = expression_stack.pop().unwrap();
                            let lhs = expression_stack.pop().unwrap();
                            expression_stack.push(Expression::Operation(Box::new(Operation {
                                operator: oper,
                                lhs: lhs,
                                rhs: rhs
                            })));
                            last_token = token_stack.last().map(|t| *t);
                        },
                        _ => break
                    }
                }
            }
            None => panic!("this is actually impossible")
        }

        // println!("expr: {:?}, token: {:?}", expression_stack, token_stack);

    }

    return expression_stack.pop().unwrap();
}

fn main() {
    let input = input();
    let mut expressions = Vec::new();
    for line in input.lines() {
        expressions.push(tokenize_line(line));
    }
    println!("{:?}", expressions);

    let parsed = expressions.iter().map(parse_expression).collect::<Vec<_>>();
    println!("parsed: {:?}", parsed);

    let sum: u64 = parsed.iter().map(|e| e.eval()).sum();

    println!("{}", sum);
}
