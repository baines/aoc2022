use std::collections::HashSet;


fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let result = get_index_p2(&input).unwrap();

    println!("{}", result);
}

/*
fn get_index_p1(data: &String) -> Option<i32> {
    let chars = data.lines().last()?.chars();
    let mut i = 1;
    let mut window: [char; 4] = ['\0', '\0', '\0', '\0'];

    for c in chars {
        window.copy_within(1.., 0);
        window[3] = c;

        if window.contains(&'\0') {
            i += 1;
            continue
        }

        let mut seen = HashSet::new();
        let mut uniq_count = 0;

        for w in window {
            if !seen.contains(&w) {
                uniq_count += 1;
            }
            seen.insert(w);
        }

        if uniq_count == 4 {
            return Some(i);
        }

        i += 1;
    }

    return None
}*/

fn get_index_p2(data: &String) -> Option<i32> {
    let chars = data.lines().last()?.chars();
    let mut i = 1;
    let mut window: [char; 14] = ['\0'; 14];

    for c in chars {
        window.copy_within(1.., 0);
        window[13] = c;

        if window.contains(&'\0') {
            i += 1;
            continue
        }

        let mut seen = HashSet::new();
        let mut found_dup = false;

        for w in window {
            if seen.contains(&w) {
                found_dup = true;
            }
            seen.insert(w);
        }

        if !found_dup {
            return Some(i);
        }

        i += 1;
    }

    return None
}
