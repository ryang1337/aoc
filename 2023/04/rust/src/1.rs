use std::{
    collections::HashSet,
    io::{self, BufRead},
};
use text_io::scan;

fn main() {
    let stdin = io::stdin();
    let mut res = 0;
    for line in stdin.lock().lines() {
        let card_num: String;
        let winning_nums: String;
        let curr_nums: String;
        let line = line.unwrap();

        scan!(line.bytes() => "Card{}: {}| {}\n", card_num, winning_nums, curr_nums);
        let card_num: i32 = card_num.trim().parse::<i32>().unwrap();
        let winning_nums = winning_nums.trim();
        let curr_nums = curr_nums.trim();

        let winning_nums: HashSet<i32> = winning_nums
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut count = 0;
        for num in curr_nums.split_whitespace() {
            let num = num.parse::<i32>().unwrap();
            if winning_nums.contains(&num) {
                count += 1;
            }
        }

        res += if count == 0 { 0 } else { 1 << (count - 1) };
    }

    println!("{res}");
}
