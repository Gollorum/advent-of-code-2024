extern crate core;

use cached::proc_macro::cached;

const INPUT_ACTUAL: &str = include_str!("inputs/11.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/11_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

#[cached]
fn children_after(iterations: u8, stone: u64) -> u64 {
    if iterations == 0 {
        return 1;
    }
    if stone == 0 {
        return children_after(iterations - 1, 1);
    }
    let digits = (stone as f32).log10().floor() as u32 + 1;
    if digits % 2 == 0 {
        let left_half = stone / 10u64.pow(digits / 2);
        let right_half = stone % 10u64.pow(digits / 2);
        return children_after(iterations - 1, left_half) + children_after(iterations - 1, right_half);    
    }
    children_after(iterations - 1, stone * 2024)
}

fn main() {
    let stones = INPUT.trim().split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    let new_stones = stones.iter().map(|s| children_after(75, *s)).sum::<u64>();
    println!("{:?}", new_stones);
}