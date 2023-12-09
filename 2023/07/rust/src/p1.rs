use crate::hand::Hand;
use std::io::{self, BufRead};
use text_io::scan;

pub fn solve() {
    let mut hands: Vec<(Hand, u32)> = Vec::new();
    let mut res: usize = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let hand: String;
        let bid: u32;

        scan!(line.bytes() => "{} {}\n", hand, bid);
        let hand = Hand::new(&hand);
        hands.push((hand, bid));
    }

    hands.sort_by(|h1, h2| {
        let h1 = &h1.0;
        let h2 = &h2.0;

        h1.cmp(&h2)
    });

    for (i, hand) in hands.iter().enumerate() {
        res += (i + 1) * (hand.1 as usize);
    }

    println!("{res}");
}
