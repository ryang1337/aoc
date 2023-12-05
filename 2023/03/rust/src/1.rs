use std::io::{self, BufRead};

fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && !(c == '.');
}

fn is_part(v: &Vec<Vec<char>>, i: usize, begin: usize, end: usize) -> bool {
    if i != 0 {
        for j in begin..end {
            if is_symbol(v[i - 1][j]) {
                return true;
            }
        }
    }

    if i != v.len() - 1 {
        for j in begin..end {
            if is_symbol(v[i + 1][j]) {
                return true;
            }
        }
    }

    if begin != 0 {
        let l = if i == 0 { 0 } else { i - 1 };
        let r = if i == v.len() - 1 { v.len() } else { i + 2 };
        for i in l..r {
            if is_symbol(v[i][begin - 1]) {
                return true;
            }
        }
    }

    if end != v[0].len() {
        let l = if i == 0 { 0 } else { i - 1 };
        let r = if i == v.len() - 1 { v.len() } else { i + 2 };
        for i in l..r {
            if is_symbol(v[i][end]) {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let mut res = 0;

    let stdin = io::stdin();
    let mut v: Vec<Vec<char>> = Vec::new();
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

                if is_part(&v, i, begin, end) {
                    res += num;
                }
            } else {
                j += 1;
            }
        }
    }

    println!("{res}");
}
