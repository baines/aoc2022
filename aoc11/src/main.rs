#![feature(iter_array_chunks)]
use regex::{Regex, Captures};

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    op: MonkeyOp,
    divisor: usize,
    pass_to_true: usize,
    pass_to_false: usize,
    inspect_count: usize,
}

#[derive(Debug)]
enum MonkeyOp {
    Add(usize),
    Mul(usize),
    Square,
}

impl<'a> From<Captures<'a>> for MonkeyOp {
    fn from(caps: Captures) -> Self {
        let sym = caps.get(1).unwrap().as_str();
        let operand = caps.get(2).unwrap().as_str();

        if operand == "old" {
            return MonkeyOp::Square;
        } else if sym == "*" {
            let num: usize = operand.parse().unwrap();
            return MonkeyOp::Mul(num);
        } else if sym == "+" {
            let num: usize = operand.parse().unwrap();
            return MonkeyOp::Add(num);
        } else {
            panic!("unknown op: {}", sym);
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let oprx = Regex::new(r"([+*]) (\d+|old)$").unwrap();
    let numrx = Regex::new(r"(\d+)$").unwrap();

    let nonblanklines = input.lines().filter(|line| !line.is_empty());

    let mut monkeys: Vec<Monkey> = nonblanklines.array_chunks::<6>().filter_map(|lines| {
        let items: Vec<usize> = lines[1].split(": ")
            .last()?
            .split(", ")
            .map(|n| n.parse().unwrap())
            .collect();

        let op: MonkeyOp = oprx.captures(lines[2])?.into();
        let div: usize = numrx.captures(lines[3])?.get(1)?.as_str().parse().ok()?;
        let trueidx: usize = numrx.captures(lines[4])?.get(1)?.as_str().parse().ok()?;
        let falseidx: usize = numrx.captures(lines[5])?.get(1)?.as_str().parse().ok()?;

        return Some(Monkey {
            items, op, divisor: div, pass_to_true: trueidx, pass_to_false: falseidx, inspect_count: 0
        });
    }).collect();

    let nrounds = 10000;
    let modulo = monkeys.iter().map(|m| m.divisor).product();

    for _round in 0..nrounds {
        for idx in 0..monkeys.len() {
            do_turn_p2(&mut monkeys, idx, modulo);
        }

        /*
        println!("After round {}:", _round+1);
        for (i, m) in monkeys.iter().enumerate() {
            print!("Monkey {}: ", i);

            for item in &m.items {
                print!("{}, ", item);
            }

            println!("");
        }
        */
    }

    for (i, m) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected {} times.", i, m.inspect_count);
    }

    monkeys.sort_by(|a, b| {
        return b.inspect_count.cmp(&a.inspect_count);
    });

    println!("monkey business = {}", monkeys[0].inspect_count * monkeys[1].inspect_count);
}

#[allow(dead_code)]
fn do_turn_p1(monkeys: &mut Vec<Monkey>, index: usize) {
    let m = &monkeys[index];

    let updates: Vec<(usize, usize)> = m.items.iter().map(|item| {
        let newitem: usize = match m.op {
            MonkeyOp::Add(n) => { (item + n) / 3 },
            MonkeyOp::Mul(n) => { (item * n) / 3 },
            MonkeyOp::Square => { (item * item) / 3 },
        };

        let passidx = {
            if (newitem % monkeys[index].divisor) == 0 {
                monkeys[index].pass_to_true
            } else {
                monkeys[index].pass_to_false
            }
        };

        return (passidx, newitem);
    }).collect();

    drop(m);

    for (idx, value) in &updates {
        monkeys[*idx].items.push(*value);
    }

    monkeys[index].inspect_count += updates.len();
    monkeys[index].items.clear();
}

fn do_turn_p2(monkeys: &mut Vec<Monkey>, index: usize, modulo: usize) {
    let m = &monkeys[index];

    let updates: Vec<(usize, usize)> = m.items.iter().map(|item| {
        let newitem: usize = match m.op {
            MonkeyOp::Add(n) => { (item + n) % modulo },
            MonkeyOp::Mul(n) => { (item * n) % modulo },
            MonkeyOp::Square => { (item * item) % modulo },
        };

        let passidx = {
            if (newitem % m.divisor) == 0 {
                monkeys[index].pass_to_true
            } else {
                monkeys[index].pass_to_false
            }
        };

        return (passidx, newitem);
    }).collect();

    drop(m);

    for (idx, value) in &updates {
        monkeys[*idx].items.push(*value);
    }

    monkeys[index].inspect_count += updates.len();
    monkeys[index].items.clear();
}
