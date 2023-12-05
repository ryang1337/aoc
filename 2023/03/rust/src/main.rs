use std::collections::HashMap;
use std::io::{self, BufRead};

fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && !(c == '.');
}

fn is_part(v: &Vec<Vec<char>>, i: usize, begin: usize, end: usize) -> Option<(usize, usize)> {
    if i != 0 {
        for j in begin..end {
            if is_symbol(v[i - 1][j]) {
                return Some((i - 1, j));
            }
        }
    }

    if i != v.len() - 1 {
        for j in begin..end {
            if is_symbol(v[i + 1][j]) {
                return Some((i + 1, j));
            }
        }
    }

    if begin != 0 {
        let l = if i == 0 { 0 } else { i - 1 };
        let r = if i == v.len() - 1 { v.len() } else { i + 2 };
        for i in l..r {
            if is_symbol(v[i][begin - 1]) {
                return Some((i, begin - 1));
            }
        }
    }

    if end != v[0].len() {
        let l = if i == 0 { 0 } else { i - 1 };
        let r = if i == v.len() - 1 { v.len() } else { i + 2 };
        for i in l..r {
            if is_symbol(v[i][end]) {
                return Some((i, end));
            }
        }
    }

    return None;
}

fn main() {
    let mut res = 0;

    let stdin = io::stdin();
    let mut v: Vec<Vec<char>> = Vec::new();
    let mut map: HashMap<(usize, usize), (i32, i32)> = HashMap::new();
    for line in stdin.lock().lines() {
        let line = line.unwrap().chars().collect();
        v.push(line);
    }

    for (i, vec) in v.iter().enumerate() {
        let mut begin: usize;
        let mut end: usize;
        let mut j: usize = 0;
        while j < vec.len() {
            if vec[j].is_digit(10) {
                let mut s = String::new();
                begin = j;
                while j < vec.len() && vec[j].is_digit(10) {
                    s.push(vec[j]);
                    j += 1;
                }
                end = j;

                let num: i32 = s.parse().unwrap();

                match is_part(&v, i, begin, end) {
                    Some(x) => {
                        if map.contains_key(&x) {
                            map.insert(x, (map[&x].0 * num, map[&x].1 + 1));
                        } else {
                            map.insert(x, (num, 1));
                        }
                    }
                    None => {}
                }
            } else {
                j += 1;
            }
        }
    }

    for (_, part) in map {
        if part.1 == 2 {
            res += part.0;
        }
    }

    println!("{res}");
}
