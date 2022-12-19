#![feature(iter_array_chunks)]

use std::{str::FromStr, collections::{HashMap, HashSet, LinkedList}};
use regex::Regex;

#[derive(Debug)]
struct ValveRoom {
    id: String,
    rate: i32,
    edges: Vec<String>,
}

impl FromStr for ValveRoom {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rx = Regex::new(r"^Valve (..) has flow rate=(\d+); tunnels? leads? to valves? (.*)$").unwrap();
        let caps = rx.captures(s).unwrap();

        let id = caps.get(1).unwrap().as_str().to_string();
        let rate = caps.get(2).unwrap().as_str().parse().unwrap();
        let edgestr = caps.get(3).unwrap().as_str();

        let edges: Vec<String> = edgestr.chars().array_chunks().filter_map(|id: [char; 2]| {
            if id[0] == ',' {
                None
            } else {
                Some(format!("{}{}", id[0], id[1]))
            }
        }).collect();

        Ok(ValveRoom {
            id, rate, edges
        })
    }
}

fn calc_distances<'a>(rooms: &'a Vec<ValveRoom>) -> HashMap<&'a str, Vec<(&'a str, i32)>> {
    let mut result = HashMap::new();

    for i in 0..rooms.len() {
        let mut next: LinkedList<(&ValveRoom, i32)> = LinkedList::new();
        next.push_back((&rooms[i], 0));

        let mut dists: Vec<(&'a str, i32)> = vec![];
        let mut checked: HashSet<&'a str> = HashSet::new();
        checked.insert(&rooms[i].id);

        while !next.is_empty() {
            let (cur, dist) = next.pop_front().unwrap();

            for edge in &cur.edges {
                if !checked.contains(&edge as &str) {
                    dists.push((&edge, dist + 1));
                    checked.insert(&edge);

                    if let Some(v) = rooms.iter().find(|vr| vr.id == *edge) {
                        next.push_back((&v, dist + 1));
                    }
                }
            }
        }

        result.insert(&rooms[i].id as &str, dists);
    }

    return result;
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let rooms: Vec<ValveRoom> = input.lines().map(|l| l.parse().unwrap()).collect();
    let all_dists = calc_distances(&rooms);

    let mut targets: Vec<&ValveRoom> = rooms.iter().filter(|r| r.rate > 0).collect();

    targets.sort_by(|a, b| {
        b.rate.cmp(&a.rate)
    });

    for t in &targets {
        println!("target {}: {}", t.id, t.rate);
    }

    let aa = rooms.iter().find(|r| r.id == "AA").unwrap();

    let mut time = 26;
    let mut ele_time = 26;

    let mut score = 0;
    let mut cur_room = aa;
    let mut ele_room = aa;

    loop {
        let mut done_count = 0;

        for (room, time) in [(&mut cur_room, &mut time), (&mut ele_room, &mut ele_time)] {
            if room.rate > 0 {
                *time -= 1;
                //println!("turn valve in {}, released {}x{} pressure", cur_room.id, cur_room.rate, time);
                //println!(" % time left = {}", time);
                score += room.rate * *time;
            }

            let dists = all_dists.get(&room.id as &str).unwrap();

            let Some(next_place) = find_next_place(room, dists, &targets) else {
                done_count += 1;
                continue;
            };

            let idx = targets.iter().enumerate().find(|t| { t.1.id == next_place.0.id }).unwrap().0;
            targets.swap_remove(idx);

            *time -= next_place.1;
            if *time <= 0 {
                done_count += 1;
                continue;
            }

            *room = next_place.0;
        }

        if done_count == 2 {
            break;
        }
    }

    println!("final score = {}, time_left = {}", score, time);
}

fn find_next_place<'a>(cur_room: &'a ValveRoom, dists: &'a Vec<(&'a str, i32)>, targets: &Vec<&'a ValveRoom>) -> Option<(&'a ValveRoom, &'a i32, f32)> {
    return targets.iter().fold(None, |acc, r| {

        let Some((_, dist)) = dists.iter().find(|r2| { r2.0 == &r.id }) else {
            return acc
        };

        let value = r.rate as f32 / *dist as f32;
        println!("  {} -> {}, dist = {}, rate = {}, value = {}", cur_room.id, r.id, dist, r.rate, value);

        match acc {
            None => {
                Some((r, dist, value))
            },
            Some((_, _, cur_value)) => {
                if value >= cur_value {
                    Some((r, dist, value))
                } else {
                    acc
                }
            }
        }
    });
}

