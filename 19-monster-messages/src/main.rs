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

    fn match_against(& self, mut chars: &mut std::str::Chars) -> bool {
        let top_rule = self.rules.get(&0).unwrap();
        let matches = top_rule.match_against(&self, &mut chars);
        matches && chars.next().is_none()
    }
}

#[derive(Debug)]
enum Rule {
    Sequence(Box<Rule>, Box<Rule>),
    Alternative(Box<Rule>, Box<Rule>),
    Literal(char),
    Reference(usize),
}

impl Rule {
    fn match_against(& self, rules: &Rules, mut chars: &mut std::str::Chars) -> bool {
        match self {
            Rule::Literal(c) => {
                chars.next().unwrap() == *c
            },
            Rule::Reference(n) => rules.rules.get(&n).unwrap().match_against(&rules, &mut chars),
            Rule::Sequence(a, b) => a.match_against(&rules, &mut chars) && b.match_against(&rules, &mut chars),
            Rule::Alternative(a, b) => {
                // store the state in case we need to go back
                let backup = chars.as_str();
                if a.match_against(&rules, &mut backup.chars()) {
                    // match again to advance chars
                    // not the best way to do it
                    a.match_against(&rules, &mut chars)
                } else {
                    b.match_against(&rules, &mut chars)
                }
            }
        }
    }
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

        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '"' => {
                    tokens.push(Token::Literal(chars.next().unwrap()));
                    break;
                },
                '0'..='9' => {
                    let mut num_str = String::new();
                    num_str.push(c);
                    while let Some(true) = chars.peek().map(|c| c.is_ascii_digit()) {
                        num_str.push(chars.next().unwrap());
                    }
                    tokens.push(Token::Reference(num_str.parse().unwrap()));
                },
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

    // println!("rules: {:?}\n\nmessages: {:?}", rules, messages);

    let valid = messages.iter().map(|msg| rules.match_against(&mut msg.chars())).collect::<Vec<_>>();

    let valid_count = valid.iter().filter(|i| **i).count();

    println!("valid: {:?}", valid_count);
}
