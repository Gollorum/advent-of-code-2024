extern crate core;

use std::collections::{HashMap, HashSet};

const INPUT_ACTUAL: &str = include_str!("inputs/22.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/22_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

const PRUNE_ROOT: u32 = 0b1000000000000000000000000u32;

fn advance_secret(mut num: u32) -> u32 {
    num = (num ^ (num << 6)) & (PRUNE_ROOT - 1);
    num = num ^ (num >> 5);
    num = (num ^ (num << 11)) & (PRUNE_ROOT - 1);
    num
}

fn main() {
    let nums = INPUT.trim().split('\n').map(|line| line.parse::<u32>().unwrap()).collect::<Vec<u32>>();
    let mut part1_sum = 0u64;
    let mut sequence_score = HashMap::new();
    for mut num in nums {
        let mut local_seen_seqs = HashSet::new();
        let mut last_price = (num % 10) as i8;
        let mut m3 = 0;
        let mut m2 = 0;
        let mut m1 = 0;
        for i in 0..2000 {
            num = advance_secret(num);
            let price = (num % 10) as i8;
            let m = price - last_price;
            if i >= 3 {
                let seq = (m3, m2, m1, m);
                if !local_seen_seqs.contains(&seq) {
                    local_seen_seqs.insert(seq);
                    let score = sequence_score.entry(seq).or_insert(0);
                    *score += price as u64;
                }
            }
            m3 = m2;
            m2 = m1;
            m1 = m;
            last_price = price;
        }
        part1_sum += num as u64;
    }
    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", sequence_score.values().max().unwrap());
}