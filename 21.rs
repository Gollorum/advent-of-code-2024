extern crate core;

use std::collections::HashMap;
use std::iter;
use itertools::Itertools;

const INPUT_ACTUAL: &str = include_str!("inputs/21.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/21_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

const NUM_PAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A']];
const DIR_PAD: [[char; 3]; 2] = [
    [' ', '^', 'A'],
    ['<', 'v', '>']];
const NUM_LEVEL: usize = 25;

fn pos_of<const H: usize>(button: char, target_pad: [[char; 3]; H]) -> (usize, usize) {
    target_pad.iter().enumerate().find_map(|(y, row)| row.iter().enumerate().find_map(|(x, &c)| if c == button { Some((x, y)) } else { None })).unwrap()
}

fn optimal_path_to_press(from: char, to: char, depth: usize, mem: &mut HashMap<(char, char, usize), usize>) -> usize {
    if from == to {
        return 1;
    }
    if let Some(dirs) = mem.get(&(from, to, depth)) {
        return dirs.clone();
    }
    let pos_a = if depth == NUM_LEVEL {pos_of(from, NUM_PAD)} else {pos_of(from, DIR_PAD)};
    let pos_b = if depth == NUM_LEVEL {pos_of(to, NUM_PAD)} else {pos_of(to, DIR_PAD)};
    let res = if pos_a.0 == pos_b.0 || pos_a.1 == pos_b.1 {
        let mut dir = if pos_a.0 == pos_b.0 {
            if pos_a.1 < pos_b.1 { vec!['v'; pos_b.1 - pos_a.1] }
            else { vec!['^'; pos_a.1 - pos_b.1] }
        } else {
            if pos_a.0 < pos_b.0 { vec!['>'; pos_b.0 - pos_a.0] }
            else { vec!['<'; pos_a.0 - pos_b.0] }
        };
        dir.push('A');
        cost_of_path(&dir, depth, mem)
    } else {
        let side = match pos_b.0 as isize - pos_a.0 as isize {
            x if x > 0 => iter::repeat('>').take(x as usize).collect::<Vec<char>>(),
            x if x < 0 => iter::repeat('<').take((-x) as usize).collect::<Vec<char>>(),
            _ => panic!()
        };
        let vert = match pos_b.1 as isize - pos_a.1 as isize {
            y if y > 0 => iter::repeat('v').take(y as usize).collect::<Vec<char>>(),
            y if y < 0 => iter::repeat('^').take((-y) as usize).collect::<Vec<char>>(),
            _ => panic!()
        };
        let combined = [side, vert].concat();
        let all_combs: Vec<Vec<char>> = combined.iter().permutations(combined.len()).unique().map(|dirs| dirs.iter().map(|d| **d).chain(std::iter::once('A')).collect()).collect();
        all_combs.iter()
            .filter(|p| validate_path(pos_a.0, pos_a.1, p, depth))
            .map(|dirs| cost_of_path(dirs, depth, mem))
            .min().unwrap()
    };
    mem.insert((from, to, depth), res);
    res
}

fn validate_path(mut x: usize, mut y: usize, path: &Vec<char>, depth: usize) -> bool {
    for &dir in path.iter() {
        if dir == '<' {
            x -= 1;
        } else if dir == '>' {
            x += 1;
        } else if dir == '^' {
            y -= 1;
        } else if dir == 'v' {
            y += 1;
        }
        let c = if depth == NUM_LEVEL { NUM_PAD[y][x] } else { DIR_PAD[y][x] };
        if c == ' ' {
            return false;
        }
    }
    true
}

fn cost_of_path(path: &Vec<char>, depth: usize, mem: &mut HashMap<(char, char, usize), usize>) -> usize {
    if depth == 0 { path.len() }
    else {
        let mut accum = 0;
        let mut last_pos = 'A';
        for &dir in path.iter() {
            let new_dirs = optimal_path_to_press(last_pos, dir, depth - 1, mem);
            accum += new_dirs;
            last_pos = dir;
        }
        accum
    }
}

fn main() {
    let sequences = INPUT.trim().lines().collect::<Vec<&str>>();
    let mut result = 0;
    let mut mem = HashMap::new();
    for line in sequences {
        let mut cur_pos = 'A';
        let mut accum = 0;
        for button in line.chars() {
            let new_dirs = optimal_path_to_press(cur_pos, button, NUM_LEVEL, &mut mem);
            accum += new_dirs;
            cur_pos = button;
        }
        let num_val = line[0..(line.len() - 1)].parse::<usize>().unwrap();
        println!("{:?} * {:?}", accum, num_val);
        result += num_val * accum;
    }
    println!("{:?}", result);
}