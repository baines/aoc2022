use std::ops::{Index, IndexMut};

#[derive(Clone)]
struct Cell {
    height: i32,
    min_path: Option<u32>,
    is_start: bool,
    is_end: bool,
}

impl Cell {
    fn new(letter: char) -> Cell {
        let height = {
            if letter == 'S' {
                0
            } else if letter == 'E' {
                25
            } else {
                ((letter as u8) - ('a' as u8)) as i32
            }
        };

        return Cell {
            height,
            min_path: if letter == 'S' { Some(0) } else { None },
            is_start: letter == 'S',
            is_end: letter == 'E',
        };
    }
}

#[derive(Copy, Clone)]
struct Pos(usize, usize);

impl Pos {
    fn up(&self) -> Option<Self> {
        if self.1 == 0 {
            None
        } else {
            Some(Pos(self.0, self.1-1))
        }
    }

    fn down(&self, height: usize) -> Option<Self> {
        if self.1 >= (height - 1) {
            None
        } else {
            Some(Pos(self.0, self.1+1))
        }
    }

    fn left(&self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(Pos(self.0-1, self.1))
        }
    }

    fn right(&self, width: usize) -> Option<Self> {
        if self.0 >= (width - 1) {
            None
        } else {
            Some(Pos(self.0+1, self.1))
        }
    }
}

struct Map {
    rows: Vec<Vec<Cell>>,
}

impl Index<Pos> for Map {
    type Output = Cell;

    fn index(&self, index: Pos) -> &Self::Output {
        return &self.rows[index.1][index.0];
    }
}

impl IndexMut<Pos> for Map {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        return &mut self.rows[index.1][index.0];
    }
}

impl Map {
    fn width(&self) -> usize {
        return self.rows[0].len();
    }

    fn height(&self) -> usize {
        return self.rows.len();
    }

    fn new(cells: Vec<Vec<Cell>>) -> Map {
        return Map {
            rows: cells
        }
    }

    fn get_start(&self) -> Option<Pos> {
        for (j, row) in self.rows.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                if cell.is_start {
                    return Some(Pos(i, j));
                }
            }
        }
        return None;
    }

    fn get_end(&self) -> Option<Pos> {
        for (j, row) in self.rows.iter().enumerate() {
            for (i, cell) in row.iter().enumerate() {
                if cell.is_end {
                    return Some(Pos(i, j));
                }
            }
        }
        return None;
    }

    fn climbable(&self, from: Pos, to: Pos) -> bool {
        return self[to].height - self[from].height <= 1;
    }

    fn better_path(&self, from: Pos, to: Pos) -> bool {
        if !self.climbable(from, to) {
            return false;
        }

        let len = self[from].min_path.unwrap() + 1;

        if let Some(current) = self[to].min_path {
            len < current
        } else {
            true
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut map = Map::new(
        input.lines().map(|line| {
            line.chars().map(|c| Cell::new(c)).collect()
        }).collect()
    );

    let width = map.width();
    let height = map.height();

    let mut to_explore: Vec<Pos> = vec![];
    let start = map.get_start().unwrap();
    to_explore.push(start);

    while !to_explore.is_empty() {
        let pos = to_explore.pop().unwrap();
        let path_len = map[pos].min_path.unwrap();

        if let Some(to) = pos.up() {
            if map.better_path(pos, to) {
                map[to].min_path = Some(path_len + 1);
                to_explore.push(to);
            }
        }

        if let Some(to) = pos.down(height) {
            if map.better_path(pos, to) {
                map[to].min_path = Some(path_len + 1);
                to_explore.push(to);
            }
        }

        if let Some(to) = pos.left() {
            if map.better_path(pos, to) {
                map[to].min_path = Some(path_len + 1);
                to_explore.push(to);
            }
        }

        if let Some(to) = pos.right(width) {
            if map.better_path(pos, to) {
                map[to].min_path = Some(path_len + 1);
                to_explore.push(to);
            }
        }
    }

    let end = map.get_end().unwrap();

    println!("path len = {}", map[end].min_path.unwrap());
}
