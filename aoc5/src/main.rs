use regex::Regex;

#[derive(Clone, Copy)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut parsing_layout = true;
    let mut state: Vec<String> = vec![];
    let mut moves: Vec<Move> = vec![];

    input.lines().for_each(|line| {
        if line.is_empty() {
            return;
        }

        if parsing_layout {
            parsing_layout = parse_layout_line(&mut state, &line);
        } else {
            //println!("line = {}", line);
            moves.push(parse_move(line).unwrap());
        }
    });

    let result = apply_moves_p2(&mut state, &moves);

    println!("{}", result);
}

fn parse_layout_line(state: &mut Vec<String>, line: &str) -> bool {
    let mut col = 0;
    let mut index = 0;

    let chars: Vec<char> = line.chars().collect();

    while index < chars.len() {
        let c = chars[index];
        if c == '[' {
            let label = chars[index+1];

            if state.len() < col + 1 {
                state.resize(col + 1, "".to_string());
            }

            state[col].push(label);

            col += 1;
            index += 4;
        } else if c == ' ' {

            // hit the line with the column numbers
            if chars[index+1] != ' ' {
                return false
            }

            index += 4;
            col += 1
        }
    }

    return true;
}

fn parse_move(line: &str) -> Option<Move> {
    let rx = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").ok()?;
    let bits = rx.captures(line)?;

    let from: usize = bits.get(2)?.as_str().parse().ok()?;
    let to: usize = bits.get(3)?.as_str().parse().ok()?;

    let m = Move {
        from: from - 1,
        to: to - 1,
        count: bits.get(1)?.as_str().parse().ok()?
    };

    return Some(m);
}

/*
fn apply_moves_p1(state: &mut Vec<String>, moves: &Vec<Move>) -> String {

    for m in moves {
        for _ in 0..m.count {
            let value = state[m.from].remove(0);
            state[m.to].insert(0, value);
        }
    }

    return state.iter().filter_map(|s| {
        return s.chars().next()
    }).collect();
}*/

fn apply_moves_p2(state: &mut Vec<String>, moves: &Vec<Move>) -> String {

    for m in moves {
        let mut values: Vec<char> = vec![];

        for _ in 0..m.count {
            values.push(state[m.from].remove(0));
        }

        for i in (0..m.count).rev() {
            state[m.to].insert(0, values[i]);
        }
    }

    return state.iter().filter_map(|s| {
        return s.chars().next()
    }).collect();
}
