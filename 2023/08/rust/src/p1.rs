use std::collections::HashMap;
use std::io::{self, BufRead};
use text_io::scan;

pub fn solve() {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut lines = io::stdin().lock().lines();

    let seq = lines.next().unwrap().unwrap();
    lines.next();

    loop {
        let line = match lines.next() {
            Some(line) => line.unwrap(),
            None => {
                break;
            }
        };

        let source: String;
        let dest: String;
        scan!(line.bytes() => "{} = {}\n", source, dest);
        let mut dest = dest.split(",");
        let l = &dest.next().unwrap()[1..];
        let r = &dest.next().unwrap().trim();
        let r = &r[..r.len() - 1];

        map.insert(source, (l.to_owned(), r.to_owned()));
    }

    let mut steps = 0;
    let mut curr = String::from("AAA");
    'repeat_seq: loop {
        for c in seq.chars() {
            match c {
                'L' => {
                    curr = (&map)[&curr].0.to_owned();
                }
                'R' => {
                    curr = (&map)[&curr].1.to_owned();
                }
                _ => {}
            }
            steps += 1;
            if curr == "ZZZ" {
                break 'repeat_seq;
            }
        }
    }

    println!("{steps}");
}
