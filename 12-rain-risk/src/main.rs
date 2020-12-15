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
    distance: i32
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
        let distance: i32 = distance_str.parse().map_err(|e| ParseInstructionErr::Int(e))?;
        Ok(Instruction { action, distance })
    }
}

// integer sin and cosine.  Only intended to work on positive multiples of 90 degrees
fn int_sin(t: i32) -> i32 {
    match (t / 90) % 4 {
        0 => 0,
        1 => 1,
        2 => 0,
        3 => -1,
        _ => panic!("invalid angle {}", t)
    }
}

fn int_cos(t: i32) -> i32 {
    match (t / 90) % 4 {
        0 => 1,
        1 => 0,
        2 => -1,
        3 => 0,
        _ => panic!("invalid angle {}", t)
    }
}

fn main() {
    let input_filename = &std::env::args().collect::<Vec<String>>()[1];
    let input = std::fs::read_to_string(input_filename).unwrap();

    let instructions = input.split('\n').filter(|i| !i.is_empty()).map(|s| s.parse().unwrap()).collect::<Vec<Instruction>>();

    let mut position = (0, 0);
    let mut orientation = (1, 0);   // east

    for i in instructions {
        match i.action {
            Action::N => position.1 += i.distance,
            Action::S => position.1 -= i.distance,
            Action::E => position.0 += i.distance,
            Action::W => position.0 -= i.distance,
            Action::L => orientation = (int_cos(i.distance) * orientation.0 - int_sin(i.distance) * orientation.1, int_sin(i.distance) * orientation.0 + int_cos(i.distance) * orientation.1),
            Action::R => orientation = (int_cos(i.distance) * orientation.0 + int_sin(i.distance) * orientation.1, int_cos(i.distance) * orientation.1 - int_sin(i.distance) * orientation.0),
            Action::F => {
                position.0 += i.distance * orientation.0;
                position.1 += i.distance * orientation.1;
            }
        };
        // println!("({}, {}), ({}, {})", position.0, position.1, orientation.0, orientation.1);
    }

    println!("{}", position.0.abs() + position.1.abs());
}
