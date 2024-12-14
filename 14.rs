extern crate core;
use regex::Regex;

const INPUT_ACTUAL: &str = include_str!("inputs/14.txt");
const SIZE_ACTUAL: (isize, isize) = (101isize, 103isize);
const INPUT_SAMPLE: &str = include_str!("inputs/14_sample.txt");
const SIZE_SAMPLE: (isize, isize) = (11isize, 7isize);
const INPUT: &str = INPUT_ACTUAL;
const SIZE: (isize, isize) = SIZE_ACTUAL;

fn main() {
    let re: Regex = Regex::new(r"p=(\d+),(\d+)\sv=([-\d]+),([-\d]+)").unwrap();
    let mut q1 = 0u32;
    let mut q2 = 0u32;
    let mut q3 = 0u32;
    let mut q4 = 0u32;
    const ITERATIONS: isize = 7286isize;
    const GRID_RESOLUTION: usize = 100usize;
    let mut grid = vec![vec![0u32; GRID_RESOLUTION+1]; GRID_RESOLUTION+1];
    for (_, [px, py, vx, vy]) in re.captures_iter(INPUT).map(|c| c.extract()) {
        let px = px.parse::<usize>().unwrap();
        let py = py.parse::<usize>().unwrap();
        let vx = vx.parse::<isize>().unwrap();
        let vy = vy.parse::<isize>().unwrap();

        let x = (px as isize + vx * ITERATIONS).rem_euclid(SIZE.0);
        let y = (py as isize + vy * ITERATIONS).rem_euclid(SIZE.1);
        if x < SIZE.0 / 2 {
            if y < SIZE.1 / 2 {
                q1 += 1;
            } else if y > SIZE.1 / 2 {
                q3 += 1;
            }
        } else if x > SIZE.0 / 2 {
            if y < SIZE.1 / 2 {
                q2 += 1;
            } else if y > SIZE.1 / 2 {
                q4 += 1;
            }
        }
        grid[y as usize * GRID_RESOLUTION / SIZE.0 as usize][x as usize * GRID_RESOLUTION / SIZE.1 as usize] += 1;
    }
    for row in grid {
        for cell in row {
            print!("{}", if cell > 10 { '#' } else  if cell > 5 { '+' } else if cell > 2 { ';' } else if cell > 0 { '.' } else { ' ' });
        }
        println!();
    }
}