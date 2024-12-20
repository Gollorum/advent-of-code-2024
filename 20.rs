extern crate core;

const INPUT_ACTUAL: &str = include_str!("inputs/20.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/20_sample.txt");
const INPUT: &str = INPUT_ACTUAL;
const CHEAT_SUCCESS_THRESHOLD: u16 = 100;
const CHEAT_LIMIT: usize = 20;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Tile {
    Wall,
    Empty,
    Start,
    End
}

fn main() {
    let map = INPUT.trim().lines().map(|line| line.chars().map(|c| match c {
        '#' => Tile::Wall,
        '.' => Tile::Empty,
        'S' => Tile::Start,
        'E' => Tile::End,
        _ => unreachable!()
    }).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>();
    let start_coords = map.iter().enumerate().find_map(|(y, row)| row.iter().enumerate().find_map(|(x, tile)| if *tile == Tile::Start { Some((x, y)) } else { None })).unwrap();
    let mut path = vec![start_coords];
    loop {
        let (x, y) = path[path.len() - 1];
        if map[y][x] == Tile::End {
            break;
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0 && new_x < map[0].len() as isize
                && new_y >= 0 && new_y < map.len() as isize
                && (path.len() <= 1 || (new_x as usize, new_y as usize) != path[path.len() - 2])
                && (map[new_y as usize][new_x as usize] == Tile::Empty
                    || map[new_y as usize][new_x as usize] == Tile::End) {
                path.push((new_x as usize, new_y as usize));
                break;
            }
        }
    }
    let mut cheat_counts = vec![0; path.len()];
    let mut successful_cheats = 0;
    for (i, pos) in path.iter().enumerate() {
        for j in (i+2)..=(path.len()-1) {
            let x_diff: usize = (path[j].0 as isize - pos.0 as isize).abs() as usize;
            let y_diff: usize = (path[j].1 as isize - pos.1 as isize).abs() as usize;
            if x_diff + y_diff <= CHEAT_LIMIT {
                let score = j - i - x_diff - y_diff;
                cheat_counts[score] += 1;
                if score >= CHEAT_SUCCESS_THRESHOLD as usize {
                    successful_cheats += 1;
                }
            }
        }
    }
    for (i, count) in cheat_counts.iter().enumerate() {
        if *count > 0 {
            println!("{} cheats save {}", count, i);
        }
    }
    println!();
    println!("Successful cheats: {}", successful_cheats);
}