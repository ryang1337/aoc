use std::{
    collections::HashSet,
    io::{self, BufRead},
};
use text_io::scan;

fn main() {
    let stdin = io::stdin();
    let mut res = 0;
    let num_lines = 198;
    let mut num_cards = vec![1; num_lines];

    for line in stdin.lock().lines() {
        let card_num: String;
        let winning_nums: String;
        let curr_nums: String;
        let line = line.unwrap();

        scan!(line.bytes() => "Card{}: {}| {}\n", card_num, winning_nums, curr_nums);
        let card_num: usize = card_num.trim().parse::<usize>().unwrap();
        let winning_nums = winning_nums.trim();
        let curr_nums = curr_nums.trim();

        let winning_nums: HashSet<i32> = winning_nums
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut matching = 0;
        for num in curr_nums.split_whitespace() {
            let num = num.parse::<i32>().unwrap();
            if winning_nums.contains(&num) {
                matching += 1;
            }
        }

        for i in card_num + 1..card_num + matching + 1 {
            num_cards[i] += num_cards[card_num];
        }
        res += num_cards[card_num];
    }

    for n in num_cards {
        println!("{n}");
    }

    println!("{res}");
}
