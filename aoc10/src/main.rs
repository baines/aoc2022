use std::str::FromStr;

struct State {
    cycle: i32,
    x: i32,
    signals: Vec<i32>,
    screen: [[char; 40]; 6],
}

enum Cmd {
    NoOp,
    AddX(i32)
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        
        if parts[0] == "noop" {
            return Ok(Cmd::NoOp);
        }

        if parts[0] == "addx" {
            return Ok(Cmd::AddX(parts[1].parse().unwrap()));
        }

        return Err(())
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut state = State {
        cycle: 1,
        x: 1,
        signals: vec![0],
        screen: [['.'; 40]; 6],
    };

    input.lines().for_each(|line| {
        let cmd: Cmd = line.parse().unwrap();

        match cmd {
            Cmd::NoOp => {
                next_cycle(&mut state);
            },
            Cmd::AddX(n) => {
                next_cycle(&mut state);
                next_cycle(&mut state);
                state.x += n;
            },
        }
    });

    // part 1
    let mut signal = 0;
    for i in (20..state.signals.len()).step_by(40) {
        //println!("i = {}, sig = {}", i, state.signals[i]);
        signal += state.signals[i];
    }

    println!("signal = {}", signal);

    // part 2

    for y in 0..6 {
        for x in 0..40 {
            print!("{}", state.screen[y][x]);
        }
        print!("\n");
    }
}

fn next_cycle(state: &mut State) {
    state.signals.push(state.x * state.cycle);

    let scrx = ((state.cycle - 1) % 40) as usize;
    let scry = ((state.cycle - 1) / 40) as usize;

    let derp = scrx as i32;

    if (derp - state.x).abs() < 2 {
        state.screen[scry][scrx] = '#';
    } else {
        state.screen[scry][scrx] = '.';
    }

    state.cycle += 1;
}
