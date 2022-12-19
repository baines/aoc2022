#![feature(iter_array_chunks)]

use std::{str::FromStr, collections::{HashMap, HashSet, LinkedList}, cmp::{min, max}};
use itertools::Itertools;
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

    let mut best_score = 0;
    let k = min(7, targets.len());

    for mut p in targets.iter().permutations(k) {
        let mut score = 0;
        let mut time = 30;
        let mut cur_room = rooms.iter().find(|r| r.id == "AA").unwrap();

        loop {
            if cur_room.rate != 0 {
                time -= 1;
                //println!("turn valve in {}, released {}x{} pressure", cur_room.id, cur_room.rate, time);
                //println!(" % time left = {}", time);
                score += cur_room.rate * time;
            }

            let dists = all_dists.get(&cur_room.id as &str).unwrap();

            if let Some(next_place) = p.pop() {
                let dist = dists.iter().find(|d| d.0 == next_place.id).unwrap().1;
                //println!("move to {}, time cost = {}", next_place.id, dist);

                time -= dist;
                if time <= 0 {
                    break;
                }

                //println!(" % time left = {}", time);

                cur_room = next_place
            } else {
                break
            }
        }

        best_score = max(score, best_score);
    }

    println!("final score = {}", best_score);
}
