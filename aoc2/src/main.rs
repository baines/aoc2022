use regex::Regex;
use std::str::FromStr;
use std::io::BufRead;

enum OpponentMove {
    Rock, Paper, Scissors
}

impl FromStr for OpponentMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        if s == "A" { return Ok(Self::Rock); }
        if s == "B" { return Ok(Self::Paper); }
        if s == "C" { return Ok(Self::Scissors); }
        return Err(())
    }
}

enum OurMove {
    Rock, Paper, Scissors
}

enum OurObjective {
    Lose, Draw, Win
}

impl FromStr for OurObjective {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        if s == "X" { return Ok(Self::Lose); }
        if s == "Y" { return Ok(Self::Draw); }
        if s == "Z" { return Ok(Self::Win); }
        return Err(())
    }
}

fn main() {
    let mut total = 0;
    let input = std::fs::read("input").unwrap();

    for line in input.lines() {
        let l = line.unwrap();
        let score = parse_line(&l).unwrap();
        total += score;
    }

    println!("score = {}\n", total);
}

fn parse_line(line: &str) -> Option<i32> {
    let rx = Regex::new(r"^([ABC]) ([XYZ])$").unwrap();
    let bits = rx.captures(line).unwrap();
    let move0: OpponentMove = bits.get(1)?.as_str().parse().ok()?;
    let obj: OurObjective = bits.get(2)?.as_str().parse().ok()?;

    let move1 = get_our_move(&move0, &obj);

    return Some(score(move0, move1));
}

fn get_our_move(opponent_move: &OpponentMove, objective: &OurObjective) -> OurMove {
    return match (opponent_move, objective) {
        (OpponentMove::Rock    , OurObjective::Lose) => OurMove::Scissors,
        (OpponentMove::Rock    , OurObjective::Draw) => OurMove::Rock,
        (OpponentMove::Rock    , OurObjective::Win) => OurMove::Paper,
        (OpponentMove::Paper   , OurObjective::Lose) => OurMove::Rock,
        (OpponentMove::Paper   , OurObjective::Draw) => OurMove::Paper,
        (OpponentMove::Paper   , OurObjective::Win) => OurMove::Scissors,
        (OpponentMove::Scissors, OurObjective::Lose) => OurMove::Paper,
        (OpponentMove::Scissors, OurObjective::Draw) => OurMove::Scissors,
        (OpponentMove::Scissors, OurObjective::Win) => OurMove::Rock,
    }
}

fn score(opponent_move: OpponentMove, our_move: OurMove) -> i32 {
    let shape_score = match our_move {
        OurMove::Rock => 1,
        OurMove::Paper => 2,
        OurMove::Scissors => 3,
    };

    let outcome_score = match (opponent_move, our_move) {
        (OpponentMove::Rock    , OurMove::Rock) => 3,
        (OpponentMove::Rock    , OurMove::Paper) => 6,
        (OpponentMove::Rock    , OurMove::Scissors) => 0,
        (OpponentMove::Paper   , OurMove::Rock) => 0,
        (OpponentMove::Paper   , OurMove::Paper) => 3,
        (OpponentMove::Paper   , OurMove::Scissors) => 6,
        (OpponentMove::Scissors, OurMove::Rock) => 6,
        (OpponentMove::Scissors, OurMove::Paper) => 0,
        (OpponentMove::Scissors, OurMove::Scissors) => 3,
    };

    return shape_score + outcome_score;
}
