use core::cmp::{min, max};
use std::str::FromStr;

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn manhattan_distance(&self, pos: &Pos) -> i64 {
        (self.x.abs_diff(pos.x) + self.y.abs_diff(pos.y)) as i64
    }
}

static MAX_SIZE: i64 = 4000000;

struct World {
    reports: Vec<SensorReport>,
    min: Pos,
    max: Pos,
}

impl World {
    fn new(reports: Vec<SensorReport>) -> Self {
        let (min, max) = reports.iter().fold((Pos{x: i64::MAX, y: i64::MAX}, Pos{x: i64::MIN, y: i64::MIN}), |acc, r| {
            let mut result = acc;

            result.0.x = min(min(result.0.x, r.sensor_pos.x - r.distance), r.beacon_pos.x);
            result.0.y = min(min(result.0.y, r.sensor_pos.y - r.distance), r.beacon_pos.y);

            result.1.x = max(max(result.1.x, r.sensor_pos.x + r.distance), r.beacon_pos.x);
            result.1.y = max(max(result.1.y, r.sensor_pos.y + r.distance), r.beacon_pos.y);

            result
        });

        World {
            reports, min, max
        }
    }

    fn find_the_fish(&self) -> Option<Pos> {
        let count = self.reports.len();

        for (i, r) in self.reports.iter().enumerate() {
            let mut iter = ReportPerimeterIter::new(r);

            println!("checking report perimeter {} of {}...", i, count);

            while let Some(p) = iter.next() {
                if p.x < 0 || p.y > 4000000 {
                    continue;
                }

                let mut found = true;

                for r2 in &self.reports {
                    if r2.in_range(&p) {
                        found = false;
                        break;
                    }
                }

                if found {
                    return Some(p);
                }
            }
        }

        None
    }
}

struct ReportPerimeterIter<'a> {
    report: &'a SensorReport,
    pos: Option<Pos>,
}

impl<'a> ReportPerimeterIter<'a> {
    fn new(report: &'a SensorReport) -> Self {
        ReportPerimeterIter {
            report,
            pos: None
        }
    }
}

fn constrain_pos(p: &mut Pos) {
    if p.x < 0 {
        p.y += p.x;
        p.x = 0;
    }

    if p.y < 0 {
        p.x -= p.y;
        p.y = 0;
    }

    if p.x > MAX_SIZE {
        p.y += MAX_SIZE - p.x;
        p.x = MAX_SIZE;
    }

    if p.y > MAX_SIZE {
        p.x -= MAX_SIZE - p.y;
        p.y = MAX_SIZE;
    }
}

impl<'a> Iterator for ReportPerimeterIter<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let org = self.report.sensor_pos;
        let r = self.report.distance + 1;

        if org.x - r > MAX_SIZE || org.x + r < 0 || org.y - r > MAX_SIZE || org.y + r < 0 {
            return None;
        }

        let mut initialpos = Pos { x: org.x - r, y: org.y };
        constrain_pos(&mut initialpos);

        let mut nextpos = {
            if let Some(p) = self.pos {

                // clockwise

                if p.x < org.x {
                    if p.y <= org.y {
                        // top left
                        Pos { x: p.x + 1, y: p.y - 1 }
                    } else {
                        // bottom left
                        Pos { x: p.x - 1, y: p.y - 1 }
                    }
                } else if p.x == org.x {
                    if p.y < org.y {
                        // top
                        Pos { x: p.x + 1, y: p.y + 1 }
                    } else {
                        // bottom
                        Pos { x: p.x - 1, y: p.y - 1 }
                    }
                } else {
                    if p.y < org.y {
                        // top right
                        Pos { x: p.x + 1, y: p.y + 1 }
                    } else {
                        // bottom right
                        Pos { x: p.x - 1, y: p.y + 1 }
                    }
                }
            } else {
                initialpos
            }
        };

        constrain_pos(&mut nextpos);

        if let Some(_) = self.pos {
            if nextpos == initialpos {
                println!("yaay");
                return None;
            }
        }

        //println!("pos = {},{}", nextpos.x, nextpos.y);

        self.pos = Some(nextpos);

        return Some(nextpos);
    }
}

#[derive(Debug)]
struct SensorReport {
    sensor_pos: Pos,
    beacon_pos: Pos,
    distance: i64,
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

    if let Some(p) = world.find_the_fish() {
        println!("hidden beacon pos = {},{}. freq={}", p.x, p.y, p.x * MAX_SIZE + p.y);
    }

}
