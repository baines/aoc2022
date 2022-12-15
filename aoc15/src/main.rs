use core::cmp::{min, max};
use std::str::FromStr;

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn manhattan_distance(&self, pos: &Pos) -> u64 {
        self.x.abs_diff(pos.x) + self.y.abs_diff(pos.y)
    }
}

struct World {
    reports: Vec<SensorReport>,
    min: Pos,
    max: Pos,
}

impl World {
    fn new(reports: Vec<SensorReport>) -> Self {
        let (min, max) = reports.iter().fold((Pos{x: i64::MAX, y: i64::MAX}, Pos{x: i64::MIN, y: i64::MIN}), |acc, r| {
            let mut result = acc;

            result.0.x = min(min(result.0.x, r.sensor_pos.x - r.distance as i64), r.beacon_pos.x);
            result.0.y = min(min(result.0.y, r.sensor_pos.y - r.distance as i64), r.beacon_pos.y);

            result.1.x = max(max(result.1.x, r.sensor_pos.x + r.distance as i64), r.beacon_pos.x);
            result.1.y = max(max(result.1.y, r.sensor_pos.y + r.distance as i64), r.beacon_pos.y);

            result
        });

        World {
            reports, min, max
        }
    }

    fn check_row(&self, y: i64) -> usize {
        let mut count = 0;

        for x in self.min.x..(self.max.x+1) {
            let mut found_beacon = false;
            let pos = Pos { x, y };

            for r in &self.reports {
                if r.beacon_pos == pos {
                    found_beacon = true;
                    break;
                }
            }

            if found_beacon {
                continue;
            }

            for r in &self.reports {
                if r.in_range(&pos) {
                    count += 1;
                    break;
                }
            }
        }

        count
    }
}

#[derive(Debug)]
struct SensorReport {
    sensor_pos: Pos,
    beacon_pos: Pos,
    distance: u64,
}

impl SensorReport {

    fn new(sensor_pos: &Pos, beacon_pos: &Pos) -> Self {
        SensorReport {
            sensor_pos: *sensor_pos,
            beacon_pos: *beacon_pos,
            distance: sensor_pos.manhattan_distance(beacon_pos)
        }
    }

    fn in_range(&self, pos: &Pos) -> bool {
        self.sensor_pos.manhattan_distance(&pos) <= self.distance
    }
}

impl FromStr for SensorReport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rx = Regex::new(r"x=(-?\d+), y=(-?\d+).*x=(-?\d+), y=(-?\d+)").unwrap();
        let caps = rx.captures(s).unwrap();

        // yeah I should use ? or something, meh...

        return Ok(SensorReport::new(&Pos {
            x: caps.get(1).unwrap().as_str().parse().unwrap(),
            y: caps.get(2).unwrap().as_str().parse().unwrap(),
        }, &Pos {
            x: caps.get(3).unwrap().as_str().parse().unwrap(),
            y: caps.get(4).unwrap().as_str().parse().unwrap(),
        }));
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let reports: Vec<SensorReport> = input.lines().map(|l| l.parse().unwrap()).collect();
    let world = World::new(reports);

    let y = 2000000;
    println!("count = {}", world.check_row(y));
}
