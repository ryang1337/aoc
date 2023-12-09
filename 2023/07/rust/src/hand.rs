use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Hand {
    pub _hand_rep: HandRep,
    pub hand_str: String,
    pub hand_val: u8,
}

#[derive(Debug)]
pub enum HandRep {
    High,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl Hand {
    pub fn new(s: &str) -> Self {
        let mut map: HashMap<char, u32> = HashMap::new();

        for c in s.chars() {
            if !map.contains_key(&c) {
                map.insert(c, 1);
            } else {
                map.insert(c, map[&c] + 1);
            }
        }

        let mut a = 0;
        let mut b = 0;
        for (_, occur) in map {
            if occur > a {
                b = a;
                a = occur;
            } else if occur > b {
                b = occur;
            }
        }

        match (a, b) {
            (5, _) => Hand {
                _hand_rep: HandRep::FiveKind,
                hand_str: str::to_string(s),
                hand_val: 6,
            },
            (4, _) => Hand {
                _hand_rep: HandRep::FourKind,
                hand_str: str::to_string(s),
                hand_val: 5,
            },
            (3, 2) => Hand {
                _hand_rep: HandRep::FullHouse,
                hand_str: str::to_string(s),
                hand_val: 4,
            },
            (3, _) => Hand {
                _hand_rep: HandRep::ThreeKind,
                hand_str: str::to_string(s),
                hand_val: 3,
            },
            (2, 2) => Hand {
                _hand_rep: HandRep::TwoPair,
                hand_str: str::to_string(s),
                hand_val: 2,
            },
            (2, _) => Hand {
                _hand_rep: HandRep::Pair,
                hand_str: str::to_string(s),
                hand_val: 1,
            },
            _ => Hand {
                _hand_rep: HandRep::High,
                hand_str: str::to_string(s),
                hand_val: 0,
            },
        }
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_val.cmp(&other.hand_val) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let chars1: Vec<char> = self.hand_str.chars().collect();
                let chars2: Vec<char> = other.hand_str.chars().collect();

                for (c1, c2) in chars1.iter().zip(chars2.iter()) {
                    match Hand::get_card_value(&c1).cmp(&Hand::get_card_value(&c2)) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {
                            continue;
                        }
                    };
                }

                Ordering::Equal
            }
        }
    }

    fn get_card_value(c: &char) -> u32 {
        let mut map = HashMap::new();
        map.insert('T', 10);
        map.insert('J', 11);
        map.insert('Q', 12);
        map.insert('K', 13);
        map.insert('A', 14);

        match c.to_digit(10) {
            Some(val) => val,
            None => map[&c],
        }
    }
}
