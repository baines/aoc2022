use std::io::BufRead;

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

fn score(item: i32) -> i32 {
    let a_lo = 'a' as i32;
    let a_up = 'A' as i32;

    if item < a_lo {
        return ((item as i32) - a_up) + 27;
    } else {
        return (item as i32) - a_lo + 1;
    }
}

fn parse_line(line: &str) -> Option<i32> {

    let item_count = line.len();
    let half_count = item_count / 2;

    let (part1, part2) = line.split_at(half_count);

    let dup_item_idx = part1.find(|c| {
        return match part2.find(c) {
            Some(_) => true,
            None => false,
        };
    })?;

    let dup_item = part1.as_bytes()[dup_item_idx];
    return Some(score(dup_item as i32));
}
