use std::{str::FromStr, collections::HashSet};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

struct State {
    seg_pos: [Pos; 10],
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
        seg_pos: [Pos { x: 0, y: 0 }; 10],
        tail_history: HashSet::new()
    };

    state.tail_history.insert(Pos { x: 0, y: 0 });

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

    state.seg_pos[0].x += xd;
    state.seg_pos[0].y += yd;

    for i in 1..10 {
        let xdiff = state.seg_pos[i-1].x - state.seg_pos[i].x;
        let ydiff = state.seg_pos[i-1].y - state.seg_pos[i].y;

        if xdiff > 1 {
            state.seg_pos[i].x += 1;
            state.seg_pos[i].y += ydiff.signum();
        } else if xdiff < -1 {
            state.seg_pos[i].x -= 1;
            state.seg_pos[i].y += ydiff.signum();
        } else if ydiff > 1 {
            state.seg_pos[i].y += 1;
            state.seg_pos[i].x += xdiff.signum();
        } else if ydiff < -1 {
            state.seg_pos[i].y -= 1;
            state.seg_pos[i].x += xdiff.signum();
        }
    }

    state.tail_history.insert(state.seg_pos[9].clone());
}
