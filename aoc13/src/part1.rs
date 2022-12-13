use std::str::FromStr;
use std::cmp::Ordering;

#[derive(Debug, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>)
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: Vec<char> = s.chars().collect();
        let mut index = 0usize;
        return Ok(Packet::parse(&bytes, &mut index));
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Number(a_num), Packet::Number(b_num)) => {
                 a_num.cmp(b_num)
            },
            (Packet::List(a_list), Packet::List(b_list)) => {
                let mut a_iter = a_list.iter();
                let mut b_iter = b_list.iter();

                loop {
                    if let Some(a_item) = a_iter.next() {
                        if let Some(b_item) = b_iter.next() {
                            let ord = a_item.cmp(b_item);
                            if ord != Ordering::Equal {
                                return ord;
                            }
                        } else {
                            return Ordering::Greater;
                        }
                    } else if let Some(_) = b_iter.next() {
                        return Ordering::Less;
                    } else {
                        return Ordering::Equal;
                    }
                }
            },
            (Packet::Number(a_num), b_list) => {
                Packet::List(vec![Packet::Number(*a_num)]).cmp(b_list)
            },
            (a_list, Packet::Number(b_num)) => {
                a_list.cmp(&Packet::List(vec![Packet::Number(*b_num)]))
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Packet {
    fn parse(str: &[char], i: &mut usize) -> Self {
        let mut c = str[*i];

        if c == '[' {
            *i += 1;

            let mut children: Vec<Packet> = vec![];

            loop {
                c = str[*i];

                if c == ',' {
                    *i += 1;
                    c = str[*i];
                }

                if c == ']' {
                    *i += 1;
                    break;
                }

                let next = Packet::parse(&str, i);
                children.push(next);
            }

            return Packet::List(children)

        } else {
            let mut num: u32 = 0;

            while let Some(d) = c.to_digit(10) {
                num = (num * 10) + d;
                *i += 1;
                c = str[*i];
            };

            return Packet::Number(num)
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let packets: Vec<Packet> = input.lines().filter_map(|line| {
        if line.is_empty() {
            None
        } else {
            Some(line.parse().unwrap())
        }
    }).collect();

    let mut index = 1;
    let mut result = 0;

    for pairs in packets.chunks(2) {
        match pairs {
            [a, b] => {
                if a <= b {
                    result += index
                }
            },
            _ => panic!("unexpected packet")
        }

        index += 1
    }

    println!("result = {}", result);
}
