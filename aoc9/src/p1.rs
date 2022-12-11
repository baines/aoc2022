use std::{str::FromStr, collections::HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

struct State {
    head_pos: Pos,
    tail_pos: Pos,
    tail_history: HashSet<Pos>,
}

enum HeadMove {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for HeadMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = s.split(" ").collect();
        let count: i32 = parts[1].parse().unwrap();

        if parts[0] == "U" { return Ok(HeadMove::Up(count)); } 
        if parts[0] == "D" { return Ok(HeadMove::Down(count)); } 
        if parts[0] == "L" { return Ok(HeadMove::Left(count)); } 
        if parts[0] == "R" { return Ok(HeadMove::Right(count)); } 

        return Err(());
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut state = State {
        head_pos: Pos {
            x: 0,
            y: 0,
        },
        tail_pos: Pos {
            x: 0,
            y: 0
        },
        tail_history: HashSet::new()
    };

    state.tail_history.insert(state.tail_pos.clone());

    input.lines().for_each(|line| {
        let m: HeadMove = line.parse().unwrap();
        apply_move(&mut state, &m);
    });

    println!("tail move count = {}", state.tail_history.len());
}

fn apply_move(state: &mut State, m: &HeadMove) {
    match m {
        HeadMove::Up(n) => {
            (0..*n).for_each(|_|{ apply_move_single(state, 0, -1); });
        },
        HeadMove::Down(n) => {
            (0..*n).for_each(|_|{ apply_move_single(state, 0, 1); });
        },
        HeadMove::Left(n) => {
            (0..*n).for_each(|_|{ apply_move_single(state, -1, 0); });
        },
        HeadMove::Right(n) => {
            (0..*n).for_each(|_|{ apply_move_single(state, 1, 0); });
        }
    }
}

fn apply_move_single(state: &mut State, xd: i32, yd: i32) {
    state.head_pos.x += xd;
    state.head_pos.y += yd;

    let xdiff = state.head_pos.x - state.tail_pos.x;
    let ydiff = state.head_pos.y - state.tail_pos.y;

    if xdiff > 1 {
        state.tail_pos.x += 1;
        state.tail_pos.y += ydiff;
    } else if xdiff < -1 {
        state.tail_pos.x -= 1;
        state.tail_pos.y += ydiff;
    } else if ydiff > 1 {
        state.tail_pos.y += 1;
        state.tail_pos.x += xdiff;
    } else if ydiff < -1 {
        state.tail_pos.y -= 1;
        state.tail_pos.x += xdiff;
    }

    state.tail_history.insert(state.tail_pos.clone());
}
