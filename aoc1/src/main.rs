use std::io::BufRead;

fn main() {
    let input = std::fs::read("input").unwrap();

    let mut calories: Vec<u32> = vec![];
    let mut total = 0;

    for line in input.lines() {
        let l = line.unwrap();

        if l == "" {
            calories.push(total);
            total = 0;
        } else {
            let num: u32 = l.parse().unwrap();
            total += num
        }
    }

    calories.push(total);

    calories.sort_by(|a, b| {
        b.cmp(a)
    });

    let mut total = 0;
    for i in 0..3 {
        total += calories[i];
    }

    println!("total = {}", total);
}
