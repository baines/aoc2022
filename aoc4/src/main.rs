use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let total: i32 = input.lines().map(|line| {
        return parse_line_p2(line).unwrap_or(0);
    }).sum();

    println!("score = {}\n", total);
}

fn parse_line_p1(line: &str) -> Option<i32> {
    let rx = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let bits = rx.captures(line).unwrap();

    let lo0: i32 = bits.get(1)?.as_str().parse().ok()?;
    let hi0: i32 = bits.get(2)?.as_str().parse().ok()?;

    let lo1: i32 = bits.get(3)?.as_str().parse().ok()?;
    let hi1: i32 = bits.get(4)?.as_str().parse().ok()?;

    if lo0 >= lo1 && hi0 <= hi1 {
        return Some(1);
    }

    if lo1 >= lo0 && hi1 <= hi0 {
        return Some(1);
    }

    return Some(0);
}

fn parse_line_p2(line: &str) -> Option<i32> {
    let rx = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    let bits = rx.captures(line).unwrap();

    let lo0: i32 = bits.get(1)?.as_str().parse().ok()?;
    let hi0: i32 = bits.get(2)?.as_str().parse().ok()?;

    let lo1: i32 = bits.get(3)?.as_str().parse().ok()?;
    let hi1: i32 = bits.get(4)?.as_str().parse().ok()?;

    if lo0 > hi1 || lo1 > hi0 {
        return Some(0);
    }

    return Some(1);
}
