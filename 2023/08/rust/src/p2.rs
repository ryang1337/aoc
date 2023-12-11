// i didn't come up with this solution
// had to look at the aoc reddit for hints
// basically finding lcm of the steps required
// for each --A to reach its corresponding --Z

use std::collections::HashMap;
use std::io::{self, BufRead};
use text_io::scan;

// this functions is actually too slow so i just
// used an online lcm calculator
fn lcm(mut nums: Vec<i64>) -> i64 {
    let original_nums = nums.clone();

    let mut largest = nums.iter().max().unwrap().to_owned();
    let mut count = 1;

    loop {
        let index = nums
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        nums[index] += original_nums[index];

        if nums[index] == largest {
            count += 1;
        } else if nums[index] > largest {
            largest = nums[index];
            count = 1;
        }

        if count == nums.len() {
            break;
        }
    }

    return nums[0];
}

pub fn solve() {
    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let mut lines = io::stdin().lock().lines();
    let mut curr: Vec<String> = Vec::new();

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

        if source.chars().nth(source.len() - 1).unwrap() == 'A' {
            curr.push(source.clone());
        }

        let mut dest = dest.split(",");
        let l = &dest.next().unwrap()[1..];
        let r = &dest.next().unwrap().trim();
        let r = &r[..r.len() - 1];

        map.insert(source.clone(), (l.to_owned(), r.to_owned()));
    }

    let mut steps: Vec<i64> = Vec::new();
    for starting_node in curr {
        let mut curr = starting_node;
        let mut cycle_len = 0;
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
                cycle_len += 1;
                if curr.chars().nth(curr.len() - 1).unwrap() == 'Z' {
                    break 'repeat_seq;
                }
            }
        }
        steps.push(cycle_len);
    }

    println!("{:?}", steps);
}
