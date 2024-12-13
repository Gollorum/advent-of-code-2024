extern crate core;
use regex::Regex;

const INPUT_ACTUAL: &str = include_str!("inputs/13.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/13_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

fn try_convert(value: f64) -> Option<u128> {
    let nearest = value.round();
    if value >= 0f64 && (value - nearest).abs() < 1e-7 {
        Some(nearest as u128)
    } else {
        None
    }
}

fn main() {
    let re: Regex = Regex::new(r"Button A\: X\+(\d+)\, Y\+(\d+)\nButton B\: X\+(\d+)\, Y\+(\d+)\nPrize\: X\=(\d+)\, Y\=(\d+)").unwrap();
    let input = INPUT.trim().replace('\r', "");
    let mut sum = 0u128;
    for (_, [a, b, c, d, e, f]) in re.captures_iter(input.as_str()).map(|c| c.extract()) {
        let ax = a.parse::<f64>().unwrap();
        let ay = b.parse::<f64>().unwrap();
        let bx = c.parse::<f64>().unwrap();
        let by = d.parse::<f64>().unwrap();
        let tx = 10000000000000f64 + e.parse::<f64>().unwrap();
        let ty = 10000000000000f64 + f.parse::<f64>().unwrap();
        let j = (ty * ax - ay * tx) / (by * ax - ay * bx);
        let i = (ty * bx - by * tx) / (ay * bx - by * ax);
        sum += match (try_convert(i), try_convert(j)) {
            (Some(i), Some(j)) => i * 3 + j,
            _ => 0
        };
    }
    println!("{}", sum);
}