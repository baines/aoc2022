use std::io::BufRead;

fn main() {
    let mut total = 0;
    let input = std::fs::read("input").unwrap();

    let mut counter = 0;
    let mut temp: [String; 3] = [
        "".to_owned(),
        "".to_owned(),
        "".to_owned()
    ];

    for line in input.lines() {
        let l = line.unwrap();

        counter = (counter + 1) % 3;

        if counter == 0 {
            temp[2] = l;
            let score = parse_lines(&temp).unwrap();
            total += score;
        } else {
            temp[counter-1] = l
        }
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

fn parse_lines(lines: &[String; 3]) -> Option<i32> {

    let dup_item_idx = lines[0].find(|c| {
        return match lines[1].find(c) {
            Some(_) => lines[2].contains(c),
            None => false,
        };
    })?;

    let dup_item = lines[0].as_bytes()[dup_item_idx];
    return Some(score(dup_item as i32));
}
