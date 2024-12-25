extern crate core;

use std::str::FromStr;
use crate::Type::Lock;

const INPUT_ACTUAL: &str = include_str!("inputs/25.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/25_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

#[derive(PartialEq, Eq, Debug)]
enum Type { Key, Lock }

struct Item {
    typ: Type,
    mask: u32
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let typ = if s.chars().next().unwrap() == '#' { Type::Lock } else { Type::Key };
        let mut vals: [u32; 5] = [0; 5];
        for line in s.split("\n"){
            for (i, val) in line.chars().enumerate() {
                if val == '#' {
                    vals[i] += 1;
                }
            }
        }
        let mut mask = 0u32;
        for i in 0..5 {
            let height = vals[i] - 1;
            let mut val = (1 << height) - 1;
            if typ == Lock {
                val = val << (5 - height);
            }
            mask = mask << 5 | val;
        }
        Ok(Item { typ, mask })
    }
}

fn main() {
    let items: Vec<Item> = INPUT.trim().split("\n\n").map(|line| line.parse().unwrap()).collect();
    let (keys, locks): (Vec<Item>, Vec<Item>) = items.into_iter().partition(|item| item.typ == Type::Key);
    let mut matching = 0u64;
    for key in keys {
        for lock in locks.iter() {
            if key.mask & lock.mask == 0 {
                matching += 1;
            }
        }
    }
    println!("{}", matching);
}