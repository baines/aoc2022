use core::cmp::{min, max};
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Air,
    Rock,
    Sand
}

impl Tile {
    fn char(&self) -> char {
        match self {
            Tile::Air => { '.' },
            Tile::Sand => { 'o' },
            Tile::Rock => { '#' },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn down(&self) -> Self {
        Pos { x: self.x, y: self.y + 1 }
    }

    fn down_left(&self) -> Self {
        Pos { x: self.x - 1, y: self.y + 1 }
    }

    fn down_right(&self) -> Self {
        Pos { x: self.x + 1, y: self.y + 1 }
    }
}

struct Cave {
    map: Vec<Vec<Tile>>,
}

impl Cave {
    fn get(&self, p: &Pos) -> Option<Tile> {
        if p.y >= self.map.len() || p.x >= self.map[p.y].len() {
            None
        } else {
            Some(self.map[p.y][p.x])
        }
    }

    fn set(&mut self, p: &Pos, v: Tile) {
        self.map[p.y][p.x] = v;
    }
}

#[derive(Debug)]
struct Path {
    vertices: Vec<Pos>,
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s.split(" -> ");

        let vertices: Vec<Pos> = pairs.map(|s2| {
            let nums: Vec<usize> = s2.split(",").map(|n| n.parse().unwrap()).collect();
            return Pos {
                x: nums[0],
                y: nums[1]
            };
        }).collect();

        return Ok(Path {
            vertices
        });
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut cave = Cave {
        map: vec![]
    };

    let paths: Vec<Path> = input.lines().map(|l| l.parse().unwrap()).collect();

    //println!("paths = {:?}", paths);

    // this is real bad

    for p in paths {
        let mut iter = p.vertices.iter();
        let mut from = iter.next().unwrap();

        for to in iter {

            let maxh = max(from.y, to.y);
            if cave.map.len() <= maxh {
                cave.map.resize_with(maxh+1, Default::default);
            }

            let maxw = max(from.x, to.x);
            for i in 0..cave.map.len() {
                if cave.map[i].len() <= maxw {
                    cave.map[i].resize_with(maxw+1, || { Tile::Air });
                }
            }

            if from.x == to.x {
                let lo = min(from.y, to.y);
                let hi = max(from.y, to.y);

                for i in lo..hi+1 {
                    cave.map[i][from.x] = Tile::Rock;
                }
            } else if from.y == to.y {
                let lo = min(from.x, to.x);
                let hi = max(from.x, to.x);

                for i in lo..hi+1 {
                    cave.map[from.y][i] = Tile::Rock;
                }
            } else {
                panic!("not h/v line");
            }

            from = to;
        }
    }

    draw_map(&cave);

    let mut drop_count = 0;

    'outer: loop {
        let mut pos = Pos { x: 500, y: 0 };

        if cave.get(&pos) == Some(Tile::Sand) {
            break;
        }

        loop {
            let mut last_result = SandFallResult::Infinite;
            let mut next_pos: Option<Pos> = None;

            // loop while blocked
            for p in [pos.down(), pos.down_left(), pos.down_right()] {
                last_result = drop_sand(&cave, &p);

                if last_result == SandFallResult::Open {
                    next_pos = Some(p);
                    break;
                }

                if last_result == SandFallResult::Infinite {
                    break 'outer;
                }
            }

            // if all positions are blocked, set this pos to sand
            if last_result == SandFallResult::Blocked {
                break;
            }

            // else there is an open position, keep falling
            pos = next_pos.unwrap();
        }

        drop_count += 1;

        //println!("set sand at {},{}", pos.x, pos.y);

        cave.set(&pos, Tile::Sand);
    }

    println!("drop count = {}", drop_count);
}

#[derive(Eq, PartialEq)]
enum SandFallResult {
    Infinite,
    Blocked,
    Open,
}

fn drop_sand(cave: &Cave, pos: &Pos) -> SandFallResult {
    match cave.get(pos) {
        Some(Tile::Air) => { SandFallResult::Open },
        Some(Tile::Rock) => { SandFallResult::Blocked },
        Some(Tile::Sand) => { SandFallResult::Blocked },
        None => { SandFallResult::Infinite },
    }
}

#[allow(dead_code)]
fn draw_map(cave: &Cave) {

    let min_x: usize = cave.map.iter().fold(usize::MAX, |acc, row| {
        let i = row.iter().position(|t| t == &Tile::Rock).unwrap_or(usize::MAX);
        min(acc, i)
    });

    for y in 0..cave.map.len() {
        for x in min_x..cave.map[y].len() {
            print!("{}", cave.map[y][x].char())
        }
        println!("");
    }

}
