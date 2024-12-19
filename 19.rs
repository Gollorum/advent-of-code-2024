extern crate core;

use std::collections::HashMap;

const INPUT_ACTUAL: &str = include_str!("inputs/19.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/19_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

fn possible_arrangements(towels: &Vec<&str>, pattern: &str, mem: &mut HashMap<String, u64>) -> u64 {
    if pattern == "" {
        return 1
    }
    if let Some(&result) = mem.get(pattern) {
        return result;
    }
    let mut possibilities = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            let result = possible_arrangements(towels, &pattern[towel.len()..], mem);
            possibilities += result;
        }
    }
    mem.insert(pattern.to_owned(), possibilities);
    possibilities
}

fn main() {
    let lines = INPUT.trim().lines().collect::<Vec<&str>>();
    let towels = lines[0].split(", ").collect::<Vec<&str>>();
    let patterns = &lines[2..];
    let mut mem = HashMap::new();
    let pattern_arrangements = patterns.iter().map(|pattern| possible_arrangements(&towels, pattern, &mut mem)).collect::<Vec<u64>>();
    println!("Part 1: {}", pattern_arrangements.iter().filter(|c| **c > 0).count());
    println!("Part 2: {}", pattern_arrangements.iter().sum::<u64>());
}