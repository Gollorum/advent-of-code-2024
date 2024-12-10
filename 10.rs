extern crate core;

const INPUT_ACTUAL: &str = include_str!("inputs/10.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/10_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

fn can_reach(map: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> bool {
    let mut queue = vec![start];
    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        if (x, y) == end {
            return true;
        }
        let next_height = map[y][x] + 1;
        if x > 0 && map[y][x - 1] == next_height {
            queue.push((x - 1, y));
        }
        if x < map[0].len() - 1 && map[y][x + 1] == next_height  {
            queue.push((x + 1, y));
        }
        if y > 0 && map[y - 1][x] == next_height {
            queue.push((x, y - 1));
        }
        if y < map.len() - 1 && map[y + 1][x] == next_height {
            queue.push((x, y + 1));
        }
    }
    false
}

fn rating_of(map: &Vec<Vec<u8>>, start: (usize, usize)) -> usize {
    let mut sum = 0;
    let next_height = map[start.1][start.0] + 1;
    if next_height == 10 {
        return 1;
    }
    if start.0 > 0 && map[start.1][start.0 - 1] == next_height {
        sum += rating_of(map, (start.0 - 1, start.1));
    }
    if start.0 < map[0].len() - 1 && map[start.1][start.0 + 1] == next_height {
        sum += rating_of(map, (start.0 + 1, start.1));
    }
    if start.1 > 0 && map[start.1 - 1][start.0] == next_height {
        sum += rating_of(map, (start.0, start.1 - 1));
    }
    if start.1 < map.len() - 1 && map[start.1 + 1][start.0] == next_height {
        sum += rating_of(map, (start.0, start.1 + 1));
    }
    sum
}

fn main() {
    let map = INPUT.trim().split("\n").map(|line| line.chars().filter(|c| c.is_digit(10)).map(|c| (c as u8) - ('0' as u8)).collect()).collect::<Vec<Vec<u8>>>();
    let starts = map.iter().enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().filter(|(_, height)| **height == 0u8)
            .map(move |(x, _)| (x, y)))
        .collect::<Vec<(usize, usize)>>();
    let ends = map.iter().enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().filter(|(_, height)| **height == 9u8)
            .map(move |(x, _)| (x, y)))
        .collect::<Vec<(usize, usize)>>();
    let score_part_1 = starts.iter().map(|&start| ends.iter().filter(|&&end| can_reach(&map, start, end)).count()).sum::<usize>();
    println!("{}", score_part_1);
    let rating_part_2 = starts.iter().map(|&start| rating_of(&map, start)).sum::<usize>();
    println!("{}", rating_part_2);
}
