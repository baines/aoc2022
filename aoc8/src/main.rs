use std::cmp::max;

fn main() {
    main_p2();
}

/*
fn main_p1() {
    let mut map: Vec<Vec<u32>> = vec![];
    let input = std::fs::read_to_string("input").unwrap();

    for line in input.lines() {
        let row: Vec<u32> = line.chars().filter_map(|c| {
            c.to_digit(10)
        }).collect();

        map.push(row);
    }

    let mut vis_count = 0;

    for (ri, row) in map.iter().enumerate() {
        for (ci, cell) in row.iter().enumerate() {
            if ri == 0 || ci == 0 || ri == map.len()-1 || ci == row.len()-1 {
                vis_count += 1;
                continue;
            }

            let mut vis_l = true;
            let mut vis_r = true;

            for (x, v) in row.iter().enumerate() {
                let blocked = v >= cell;

                if x == ci {
                    continue;
                } else if x < ci && blocked {
                    vis_l = false;
                } else if blocked {
                    vis_r = false;
                }
            }

            let mut vis_u = true;
            let mut vis_d = true;

            for x in 0..map.len() {
                let v = &map[x][ci];
                let blocked = v >= cell;

                if x == ri {
                    continue;
                } else if x < ri && blocked {
                    vis_u = false;
                } else if blocked {
                    vis_d = false;
                }
            }

            let vis = vis_l || vis_r || vis_u || vis_d;

            if vis {
                vis_count += 1;
            }
        }
    }

    println!("vis count = {}", vis_count);
}*/

fn main_p2() {
    let mut map: Vec<Vec<u32>> = vec![];
    let input = std::fs::read_to_string("input").unwrap();

    for line in input.lines() {
        let row: Vec<u32> = line.chars().filter_map(|c| {
            c.to_digit(10)
        }).collect();

        map.push(row);
    }

    let mut max_score: u32 = 0;

    for (ri, row) in map.iter().enumerate() {
        for (ci, cell) in row.iter().enumerate() {

            let vis_l = {
                let mut res = 0;
                for i in (0..ci).rev() {
                    res += 1;
                    let v = &row[i];
                    if v >= cell {
                        break;
                    }
                }
                res
            };

            let vis_r = {
                let mut res = 0;
                for i in (ci+1)..row.len() {
                    res += 1;
                    let v = &row[i];
                    if v >= cell {
                        break;
                    }
                }
                res
            };

            let vis_u = {
                let mut res = 0;
                for i in (0..ri).rev() {
                    res += 1;
                    let v = &map[i][ci];
                    if v >= cell {
                        break;
                    }
                }
                res
            };

            let vis_d = {
                let mut res = 0;
                for i in (ri+1)..map.len() {
                    res += 1;
                    let v = &map[i][ci];
                    if v >= cell {
                        break;
                    }
                }
                res
            };

            let score = vis_l * vis_r * vis_u * vis_d;
            max_score = max(max_score, score);
        }
    }

    println!("max scenic score = {}", max_score);
}
