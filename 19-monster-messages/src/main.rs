use std::collections::HashMap;
use std::str::FromStr;

fn input() -> String {
    let input_filename = &std::env::args().collect::<Vec<_>>()[1];
    std::fs::read_to_string(input_filename).unwrap()
}

#[derive(Debug)]
struct Rules {
    rules: HashMap::<usize, Rule>
}

impl Rules {
    fn new() -> Rules {
        Rules { rules: HashMap::<usize, Rule>::new() }
    }

    fn parse_from(&mut self, s: &str) {
        let split = s.split(": ").collect::<Vec<_>>();
        let num_str = split[0];
        let pattern = split[1];

        self.rules.insert(num_str.parse().unwrap(), pattern.parse().unwrap());
    }
}

#[derive(Debug)]
enum Rule {
    Sequence(Box<Rule>, Box<Rule>),
    Alternative(Box<Rule>, Box<Rule>),
    Literal(char),
    Reference(usize),
}

#[derive(Debug)]
struct ParseRuleError {
}

#[derive(Debug)]
enum Token {
    Literal(char),
    Reference(usize),
    Or,
    Sequence
}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = Vec::new();

        let mut chars = s.chars();

        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    tokens.push(Token::Literal(chars.next().unwrap()));
                    break;
                },
                '0'..='9' => tokens.push(Token::Reference((c as usize) - 0x30)),
                '|' => {
                    tokens.pop();
                    tokens.push(Token::Or);
                    chars.next();
                },
                ' ' => tokens.push(Token::Sequence),
                _ => return Err(Self::Err {})
            }
        }

        // println!("tokens: {:?}", tokens);

        let mut rule_stack = Vec::new();
        let mut connective_stack = Vec::new();

        for t in tokens {
            match t {
                Token::Literal(c) => return Ok(Rule::Literal(c)),
                Token::Reference(n) => {
                    if rule_stack.is_empty() {
                        rule_stack.push(Rule::Reference(n));
                    } else {
                        match connective_stack.last() {
                            Some(Token::Sequence) => {
                                connective_stack.pop();
                                let other_rule = rule_stack.pop().unwrap();
                                rule_stack.push(Rule::Sequence(
                                        Box::new(other_rule),
                                        Box::new(Rule::Reference(n)),
                                    ));
                            },
                            Some(Token::Or) | None => {
                                rule_stack.push(Rule::Reference(n));
                            },
                            _ => panic!("This should be impossible")
                        }
                    }
                },
                Token::Or => connective_stack.push(Token::Or),
                Token::Sequence => connective_stack.push(Token::Sequence)
            }
        }

        // println!("rules: {:?}, connectives: {:?}", rule_stack, connective_stack);

        match connective_stack.pop() {
            Some(Token::Or) => {
                let rule_b = rule_stack.pop().unwrap();
                let rule_a = rule_stack.pop().unwrap();
                rule_stack.push(Rule::Alternative(
                        Box::new(rule_a),
                        Box::new(rule_b)
                    ));
            },
            None => {},
            _ => panic!("Syntax Error")
        }

        return Ok(rule_stack.pop().unwrap());
    }
}

fn main() {
    let input = input();

    let mut rules = Rules::new();
    let mut messages = Vec::new();

    let mut rules_section = true;

    for line in input.lines() {
        match line {
            "" => rules_section = false,
            _ => {
                if rules_section {
                    rules.parse_from(line);
                } else {
                    messages.push(line);
                }
            }
        }
    }

    println!("rules: {:?}\n\nmessages: {:?}", rules, messages);
}
