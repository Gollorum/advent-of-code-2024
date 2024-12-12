extern crate core;

use std::collections::HashSet;
use std::cmp::Ordering;

const INPUT_ACTUAL: &str = include_str!("inputs/12.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/12_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
enum Direction {
    Vertical,
    Horizontal
}

fn try_remove<T: PartialEq>(vec: &mut Vec<T>, item: T) -> bool {
    match vec.iter().position(|i| *i == item) {
        Some(index) => {
            vec.remove(index);
            true
        },
        None => false
    }
}

fn main() {
    let grid = INPUT.trim().replace('\r', "").split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut to_visit = grid.iter().enumerate().flat_map(|(y, row)| (0..row.len()).map(move |x| (x, y))).collect::<HashSet<(usize, usize)>>();
    let mut regions: Vec<(char, Vec<(usize, usize)>, usize, usize)> = Vec::new();
    while !to_visit.is_empty() {
        let (x, y) = to_visit.iter().next().unwrap().clone();
        let mut region = Vec::new();
        let mut queue = vec![(x, y, (x, y, Direction::Horizontal))];
        let mut borders = Vec::new();
        let plant_type = grid[y][x];
        while !queue.is_empty() {
            let (x, y, border_coord) = queue.pop().unwrap();
            if region.contains(&(x, y)) {
                continue;
            }
            if y >= grid.len() || x >= grid[y].len() || grid[y][x] != plant_type {
                borders.push(border_coord);
                continue;
            }
            to_visit.remove(&(x, y));
            region.push((x, y));
            if x > 0 {
                queue.push((x - 1, y, (x, y, Direction::Vertical)));
            } else {
                borders.push((x, y, Direction::Vertical));
            }
            if y > 0 {
                queue.push((x, y - 1, (x, y, Direction::Horizontal)));
            } else {
                borders.push((x, y, Direction::Horizontal));
            }
            queue.push((x + 1, y, (x+1, y, Direction::Vertical)));
            queue.push((x, y + 1, (x, y+1, Direction::Horizontal)));
        }
        let mut sides = 0usize;
        let allBorders = borders.clone();
        borders.sort_by(|(x1, y1, dir1), (x2, y2, dir2)| {
            if dir1 != dir2 {
                return dir1.cmp(dir2);
            }
            if *dir1 == Direction::Horizontal {
                return x2.cmp(x1);
            } else {
                return y2.cmp(y1);
            }
        });
        while !borders.is_empty() {
            let (mut x, mut y, dir) = borders.pop().unwrap();
            sides += 1;
            if dir == Direction::Horizontal {
                loop {
                    x += 1;
                    if allBorders.contains(&(x, y, Direction::Vertical)) || y > 0 && allBorders.contains(&(x, y - 1, Direction::Vertical)) {
                        break;
                    }
                    if !try_remove(&mut borders, (x, y, dir)) {
                        break;
                    }
                }
            } else {
                loop {
                    y += 1;
                    if allBorders.contains(&(x, y, Direction::Horizontal)) || x > 0 && allBorders.contains(&(x - 1, y, Direction::Horizontal)) {
                        break;
                    }
                    if !try_remove(&mut borders, (x, y, dir)) {
                        break;
                    }
                }
            }
        }
        regions.push((plant_type, region, borders.len(), sides));
    }
    let price_per_region = regions.iter().map(|(plant_type, region, _, sides)| region.len() * sides).sum::<usize>();
    println!("{:?}", price_per_region);
}