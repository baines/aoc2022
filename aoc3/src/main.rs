#![feature(iter_array_chunks)]

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let total: u64 = input.lines().array_chunks::<3>().map(|lines| {
        return parse_lines(&lines);
    }).sum();

    println!("score = {}\n", total);
}

fn score(item: i32) -> i32 {
    let a_lo = 'a' as i32;
    let a_up = 'A' as i32;

    if item < a_lo {
        return ((item as i32) - a_up) + 26;
    } else {
        return (item as i32) - a_lo;
    }
}

fn line_to_mask(str: &str) -> u64 {
    let mut mask: u64 = 0;

    for c in str.chars() {
        mask |= 1u64 << score(c as i32);
    }

    return mask
}

fn parse_lines(lines: &[&str; 3]) -> u64 {
    let mut state: u64 = u64::MAX;

    for line in lines {
        state &= line_to_mask(line);
    }

    return (state.trailing_zeros() + 1).into()
}
