use std::collections::HashMap;

struct File {
    name: String,
    size: usize,
    is_dir: bool,
}

struct State {
    files: HashMap<String, File>,
    cwd: String,
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut state = State {
        files: HashMap::new(),
        cwd: "/".into()
    };

    state.files.insert("/".to_string(), File {
        name: "/".to_string(),
        size: 0,
        is_dir: true
    });

    input.lines().for_each(|line| {
        parse_line(&mut state, &line);
    });

    // part1(&mut state);
    part2(&mut state);
}

/*
fn part1(state: &mut State) {
    let totals = dir_totals(&state);

    let sub_100k_total = totals.values().fold(0, |total, dir_total| {
        if *dir_total > 100000 {
            return total
        }

        return total + dir_total;
    });

    println!("total = {}", sub_100k_total);
}*/

fn part2(state: &mut State) {
    let totals = dir_totals(&state);
    let available: usize = 70000000;
    let need: usize = 30000000;

    let fs_total = totals.get("/").unwrap();
    let amount_to_delete = need - (available - fs_total);

    println!("fs total = {}, need to delete = {}", fs_total, amount_to_delete);

    let mut dir_sizes: Vec<usize> = totals.values().map(|v| *v).collect();
    dir_sizes.sort();

    for size in dir_sizes {
        if size < amount_to_delete {
            continue;
        }

        println!("delete size = {}", size);
        break
    }
}

fn parse_line(state: &mut State, line: &str) {
    let parts: Vec<&str> = line.split(' ').collect();

    if parts[0] == "$" {
        if parts[1] == "cd" {
            if parts[2] == ".." {
                if state.cwd != "/" {
                    let last_slash = state.cwd[0..state.cwd.len()-1].rfind('/').unwrap();
                    state.cwd.truncate(last_slash + 1);
                }
                //println!("{}, cwd = {}", line, state.cwd);
            } else if parts[2] == "/" {
                state.cwd = "/".to_string();
            } else {
                state.cwd += parts[2];
                state.cwd.push('/');
            }
        } else if parts[1] == "ls" {

        }
    } else if parts[0] == "dir" {
        let path = format!("{}{}/", state.cwd, parts[1]);
        let file = File {
            name: parts[1].to_string(),
            size: 0,
            is_dir: true
        };

        state.files.insert(path, file);
    } else {
        let size: usize = parts[0].parse().ok().unwrap();
        let path = format!("{}{}", state.cwd, parts[1]);
        let file = File {
            name: parts[1].to_string(),
            size,
            is_dir: false,
        };

        state.files.insert(path, file);
    }
}

fn dir_totals(state: &State) -> HashMap<String, usize> {
    let mut totals: HashMap<String, usize> = HashMap::new();

    for (path, file) in &state.files {
        if file.is_dir {
            totals.insert(path.clone(), path_total(&state, &path));
        }
    }

    return totals;
}

fn path_total(state: &State, path_prefix: &str) -> usize {
    let mut total: usize = 0;

    for (path, file) in &state.files {
        if file.is_dir {
            continue;
        }

        if path.starts_with(path_prefix) {
            total += file.size;
        }
    }

    return total;
}
