extern crate core;

use std::collections::BinaryHeap;

struct Input {
    bytes: &'static str,
    dimensions: u8,
    bytes_to_simulate: u16
}
const INPUT_ACTUAL: Input = Input { bytes: include_str!("inputs/18.txt"), dimensions: 70, bytes_to_simulate: 1024};
const INPUT_SAMPLE: Input = Input { bytes: include_str!("inputs/18_sample.txt"), dimensions: 6, bytes_to_simulate: 12};
const INPUT: Input = INPUT_ACTUAL;

#[derive(PartialEq, Eq, Copy, Clone)]
struct HeapEntry {
    cost: u16,
    approx_cost: u16,
    x: u8,
    y: u8
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        other.approx_cost.cmp(&self.approx_cost)
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let all_byte_coords = INPUT.bytes.trim().split('\n').map(|line| line.split_once(',').and_then(|(x, y)| Some((x.parse::<u8>().ok()?, y.parse::<u8>().ok()?))).unwrap()).collect::<Vec<(u8, u8)>>();
    let mut grid = (0..=INPUT.dimensions).map(|x| (0..=INPUT.dimensions).map(|y| all_byte_coords.iter().take(INPUT.bytes_to_simulate as usize).any(|t| *t == (x, y))).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>();
    let mut queue = BinaryHeap::new();
    queue.push(HeapEntry { cost: 0, approx_cost: INPUT.dimensions as u16 * 2, x: 0, y: 0 });
    let mut visited = vec![vec![false; INPUT.dimensions as usize + 1]; INPUT.dimensions as usize + 1];
    while let Some(HeapEntry { cost, x, y, .. }) = queue.pop() {
        if x == INPUT.dimensions && y == INPUT.dimensions {
            println!("Part 1: {}", cost);
            break;
        }
        visited[x as usize][y as usize] = true;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_x = x as i8 + dx;
            let new_y = y as i8 + dy;
            if new_x >= 0 && new_x as u8 <= INPUT.dimensions && new_y >= 0 && new_y as u8 <= INPUT.dimensions && !visited[new_x as usize][new_y as usize] && !grid[new_x as usize][new_y as usize] {
                let cost = cost + 1;
                let approx_cost = cost + (INPUT.dimensions as u16 - new_x as u16) + (INPUT.dimensions as u16 - new_y as u16);
                queue.push(HeapEntry { cost, approx_cost, x: new_x as u8, y: new_y as u8 });
            }
        }
    }
    for i in INPUT.bytes_to_simulate as usize..all_byte_coords.len() {
        for x in 0..=INPUT.dimensions as usize {
            for y in 0..=INPUT.dimensions as usize {
                visited[x][y] = false;
            }
        }
        queue.clear();
        let (x, y) = all_byte_coords[i];
        grid[x as usize][y as usize] = true;
        queue.push(HeapEntry { cost: 0, approx_cost: INPUT.dimensions as u16 * 2, x: 0, y: 0 });
        let mut found_exit = false;
        while let Some(HeapEntry { cost, x, y, .. }) = queue.pop() {
            if x == INPUT.dimensions && y == INPUT.dimensions {
                found_exit = true;
                break;
            }
            visited[x as usize][y as usize] = true;
            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
                let new_x = x as i8 + dx;
                let new_y = y as i8 + dy;
                if new_x >= 0 && new_x as u8 <= INPUT.dimensions && new_y >= 0 && new_y as u8 <= INPUT.dimensions && !visited[new_x as usize][new_y as usize] && !grid[new_x as usize][new_y as usize] {
                    let cost = cost + 1;
                    let approx_cost = cost + (INPUT.dimensions as u16 - new_x as u16) + (INPUT.dimensions as u16 - new_y as u16);
                    queue.push(HeapEntry { cost, approx_cost, x: new_x as u8, y: new_y as u8 });
                }
            }
        }
        if !found_exit {
            println!("Part 2: {}", INPUT.bytes.trim().split('\n').nth(i).unwrap());
            break;
        }
    }
}