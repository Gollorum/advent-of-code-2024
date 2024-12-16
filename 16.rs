extern crate core;

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;

const INPUT_ACTUAL: &str = include_str!("inputs/16.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/16_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord, Hash)]
enum Direction {
    Up, Right, Down, Left
}
impl Direction {
    const ALL: [Direction; 4] = [Direction::Up, Direction::Right, Direction::Down, Direction::Left];
}

#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug)]
struct State {
    x: usize,
    y: usize,
    direction: Direction
}
#[derive(PartialEq, Eq)]
struct HeapEntry {
    state: State,
    cost: usize
}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn rotate_clockwise(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up
    }
}
fn rotate_counterclockwise(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Left,
        Direction::Right => Direction::Up,
        Direction::Down => Direction::Right,
        Direction::Left => Direction::Down
    }
}

fn enqueue(queue: &mut BinaryHeap<HeapEntry>, origins: &mut HashMap<State, (usize, Vec<State>)>, state: State, origin: State, cost: usize) {
    if let Some(prev_entry) = origins.get_mut(&state) {
        if prev_entry.0 < cost {
            return;
        } else if prev_entry.0 == cost {
            prev_entry.1.push(origin);
            return;
        } else {
            prev_entry.0 = cost;
            prev_entry.1 = vec![origin];
        }
    } else {
        origins.insert(state, (cost, vec![origin]));
    }
    queue.push(HeapEntry { state, cost });
}

fn main() {
    let mut input = INPUT.trim().split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let start = input.iter().enumerate().find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y))).unwrap();
    let end = input.iter().enumerate().find_map(|(y, row)| row.iter().position(|&c| c == 'E').map(|x| (x, y))).unwrap();

    let mut origins: HashMap<State, (usize, Vec<State>)> = HashMap::new();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(HeapEntry { state: State { x: start.0, y: start.1, direction: Direction::Right }, cost: 0 });
    while let Some(HeapEntry { state, cost }) = queue.pop() {
        if !visited.insert(state) {
            continue
        }
        let (dx, dy) = match state.direction {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0)
        };
        let (nx, ny) = (state.x as isize + dx, state.y as isize + dy);
        if nx >= 0 && ny >= 0 && ny < input.len() as isize && nx < input[ny as usize].len() as isize {
            if input[ny as usize][nx as usize] != '#' {
                enqueue(&mut queue, &mut origins, State { x: nx as usize, y: ny as usize, direction: state.direction }, state, cost + 1);
            }
        }
        enqueue(&mut queue, &mut origins, State { x: state.x, y: state.y, direction: rotate_clockwise(state.direction) }, state, cost + 1000);
        enqueue(&mut queue, &mut origins, State { x: state.x, y: state.y, direction: rotate_counterclockwise(state.direction) }, state, cost + 1000);
    }
    let all_ends = Direction::ALL.iter().map(|&dir| State { x: end.0, y: end.1, direction: dir }).collect::<Vec<State>>();
    let score = all_ends.iter().map(|&e| origins.get(&e).map(|(cost, _)| cost).unwrap_or(&usize::MAX).clone()).min().unwrap();
    let mut winning_paths_tiles = HashSet::new();
    let mut to_visit = all_ends.iter().map(|&e| (e, score)).collect::<Vec<(State, usize)>>();
    while let Some((current, cost_so_far)) = to_visit.pop(){
        winning_paths_tiles.insert((current.x, current.y));
        if let Some((cost, prev)) = origins.get(&current) {
            if *cost <= cost_so_far {
                to_visit.extend(prev.iter().map(|&p| (p, *cost)));
            }
        }
    }
    println!("{}", score);
    println!("Winning path tiles: {:?}", winning_paths_tiles.len());
}