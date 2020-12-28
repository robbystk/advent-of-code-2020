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

    fn match_against(& self, s: &str) -> bool {
        let top_rule = self.rules.get(&0).unwrap();

        // setup
        let chars = s.chars().collect::<Vec<_>>();
        let mut index = 0 as usize;

        let matches = top_rule.match_against(&self, &chars, &mut index, true);
        matches && index == chars.len()
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
    fn match_against(& self, rules: &Rules, chars: &Vec<char>, from_index: &mut usize, should_reach_end: bool) -> bool {
        // let to_go = &chars[*from_index..];
        // println!("matching {:?} against {:?}", self, to_go);
        match self {
            Rule::Literal(c) => {
                if *from_index >= chars.len() {
                    return false;
                }
                let current = chars[*from_index];
                let matched = current == *c;
                if matched {
                    *from_index += 1;
                }
                let reached_end = *from_index == chars.len();

                let match_msg = if matched {
                    format!("matched {}", c)
                } else {
                    format!("wanted {}, got {:?}", c, current)
                };
                let end_condition_message = if should_reach_end {
                    "wanted to reach end"
                } else {
                    "did not want to reach end"
                };
                let end_reached_message = if reached_end {
                    "reached end"
                } else {
                    "did not reach end"
                };
                // println!("{}; {}, and {}", match_msg, end_condition_message, end_reached_message);

                if should_reach_end {
                    matched && reached_end
                } else {
                    matched
                }
            },
            Rule::Reference(n) => rules.rules.get(&n).unwrap().match_against(&rules, &chars, from_index, should_reach_end),
            Rule::Sequence(a, b) => a.match_against(&rules, &chars, from_index, false) && b.match_against(&rules, &chars, from_index, should_reach_end),
            Rule::Alternative(a, b) => {
                // store the state in case we need to go back
                let backup_index = *from_index;
                if a.match_against(&rules, &chars, from_index, should_reach_end) {
                    true
                } else {
                    *from_index = backup_index;
                    b.match_against(&rules, &chars, from_index, should_reach_end)
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

    let valid_messages = messages.iter().filter(|msg| rules.match_against(&msg)).collect::<Vec<_>>();

    println!("valid: {:?}", valid_messages);

    let valid_count = valid_messages.len();

    println!("valid: {:?}", valid_count);
}
