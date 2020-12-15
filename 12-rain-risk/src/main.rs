use std::str::FromStr;
use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F
}

#[derive(Debug)]
struct ParseActionError {
    s: String
}

impl fmt::Display for ParseActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid action `{}`, must be one of N, S, E, W, L, R, F", self.s)
    }
}

impl FromStr for Action {
    type Err = ParseActionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Self::N),
            "S" => Ok(Self::S),
            "E" => Ok(Self::E),
            "W" => Ok(Self::W),
            "L" => Ok(Self::L),
            "R" => Ok(Self::R),
            "F" => Ok(Self::F),
            _ => Err(ParseActionError {s: s.to_string()})
        }
    }
}

#[derive(Debug)]
struct Instruction {
    action: Action,
    distance: u32
}

#[derive(Debug)]
enum ParseInstructionErr {
    Action(ParseActionError),
    Int(ParseIntError)
}

impl FromStr for Instruction {
    type Err = ParseInstructionErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action_str, distance_str) = s.split_at(1);
        let action = action_str.parse().map_err(|e| ParseInstructionErr::Action(e))?;
        let distance: u32 = distance_str.parse().map_err(|e| ParseInstructionErr::Int(e))?;
        Ok(Instruction { action, distance })
    }
}

fn main() {
    let input_filename = &std::env::args().collect::<Vec<String>>()[1];
    let input = std::fs::read_to_string(input_filename).unwrap();

    let instructions = input.split('\n').filter(|i| !i.is_empty()).collect::<Vec<&str>>();
    for instruction in &instructions {
        let instruction: Instruction = instruction.parse().unwrap();
        println!("{:?}", instruction);
    }
}
