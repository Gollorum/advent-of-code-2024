extern crate core;

use std::fmt::{Display, Formatter};

const INPUT_ACTUAL: &str = include_str!("inputs/15.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/15_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

#[derive(PartialEq, Eq, Copy, Clone)]
enum Tile {
    Wall,
    Empty,
    Box,
    BoxL,
    BoxR,
    Robot
}
impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "#"),
            Tile::Empty => write!(f, "."),
            Tile::Box => write!(f, "O"),
            Tile::BoxL => write!(f, "["),
            Tile::BoxR => write!(f, "]"),
            Tile::Robot => write!(f, "@")
        }
    }
}

struct Map<'a>(&'a Vec<Vec<Tile>>);
impl Display for Map<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn try_move_to(map: &Vec<Vec<Tile>>, x: usize, y: usize, dx: isize, dy: isize, tile: Tile) -> Option<Vec<Vec<Tile>>> {
    let (nx, ny) = (x as isize + dx, y as isize + dy);
    if nx >= 0 && ny >= 0 && ny < map.len() as isize && nx < map[ny as usize].len() as isize {
        let next_tile = map[ny as usize][nx as usize];
        match next_tile {
            Tile::Wall | Tile::Robot => None,
            Tile::Empty => {
                let mut map = map.clone();
                map[y][x] = Tile::Empty;
                map[ny as usize][nx as usize] = tile;
                Some(map)
            }
            Tile::Box | Tile::BoxL | Tile::BoxR =>
                try_move_to(map, nx as usize, ny as usize, dx, dy, next_tile).and_then(|mut map| {
                    map[y][x] = Tile::Empty;
                    map[ny as usize][nx as usize] = tile;
                    if dy == 0 || next_tile == Tile::Box {
                        Some(map)
                    } else {
                        let (ox, oy, other) = match next_tile {
                            Tile::BoxL => (nx + 1, ny, Tile::BoxR),
                            Tile::BoxR => (nx - 1, ny, Tile::BoxL),
                            _ => panic!()
                        };
                        try_move_to(&map, ox as usize, oy as usize, dx, dy, other)
                    }
                })
        }
    } else { None }
}

fn transform_for_part_2(map: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    map.iter().map(|row| row.iter().flat_map(|tile| match tile {
        Tile::Box => vec![Tile::BoxL, Tile::BoxR],
        Tile::Robot => vec![Tile::Robot, Tile::Empty],
        _ => vec![*tile, *tile]
    }).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>()
}

fn main() {
    let clean_input = INPUT.trim().replace('\r', "");
    let (map_str, moves_str) = clean_input.split_once("\n\n").unwrap();
    let mut map = map_str.split("\n").map(|s| s.chars().map(|c| match c {
        '#' => Tile::Wall,
        '.' => Tile::Empty,
        'O' => Tile::Box,
        '@' => Tile::Robot,
        _ => panic!()
    }).collect::<Vec<Tile>>()).collect::<Vec<Vec<Tile>>>();
    map = transform_for_part_2(&map);
    println!("{}", Map(&map));
    let mut robot_pos = map.iter().enumerate().flat_map(|(y, row)| row.iter().enumerate().position(|(x, tile)| *tile == Tile::Robot).map(|x| (x, y))).next().unwrap();
    for mv in moves_str.chars() {
        let (dx, dy) = match mv {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
            _ => continue
        };
        let (x, y) = robot_pos;
        match try_move_to(&mut map, x, y, dx, dy, Tile::Robot) {
            Some(new_map) => {
                map = new_map;
                let (nx, ny) = (x as isize + dx, y as isize + dy);
                robot_pos = (nx as usize, ny as usize);
            },
            None => ()
        }
        // println!("Move {}", mv);
        // println!("{}", Map(&map));
    }
    let mut score = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if *tile == Tile::Box || *tile == Tile::BoxL {
                score += x + 100 * y;
            }
        }
    }
    println!("{}", score);
}