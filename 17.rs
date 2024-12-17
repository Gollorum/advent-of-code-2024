extern crate core;
use regex::Regex;

const INPUT_ACTUAL: &str = include_str!("inputs/17.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/17_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

#[derive(Debug)]
struct State {
    a: u64,
    b: u64,
    c: u64,
    instruction_pointer: usize,
    out: Vec<u8>
}

fn resolve_operand(state: &State, operand: u8) -> u64 {
    match operand {
        0 | 1 | 2 | 3 => operand as u64,
        4 => state.a,
        5 => state.b,
        6 => state.c,
        _ => panic!("Invalid operand")
    }
}

fn execute_instruction(state: &mut State, instruction: u8, operand: u8) {
    match instruction {
        0 => state.a = state.a / (1 << resolve_operand(&state, operand)),
        1 => state.b = state.b ^ operand as u64,
        2 => state.b = resolve_operand(&state, operand) & 0b111,
        3 => if state.a > 0 { state.instruction_pointer = operand as usize }
             else { state.instruction_pointer += 2 },
        4 => state.b = state.b ^ state.c,
        5 => state.out.push(resolve_operand(&state, operand) as u8 & 0b111),
        6 => state.b = state.a / (1 << resolve_operand(&state, operand)),
        7 => state.c = state.a / (1 << resolve_operand(&state, operand)),
        _ => panic!("Invalid instruction")
    }
    if instruction != 3 {
        state.instruction_pointer += 2;
    }
}

fn main() {
    let register_regex: Regex = Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)").unwrap();
    let program_regex: Regex = Regex::new(r"Program: ([,\d]+)").unwrap();
    let input = INPUT.trim().replace('\r', "");
    let [a, b, c] = register_regex.captures(input.as_str()).unwrap().extract().1.map(|str| str.parse::<u64>().unwrap());
    let program = program_regex.captures(input.as_str()).unwrap().extract::<1>().1.first().unwrap().split(",").map(|str| str.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    let mut state = State { a, b, c, instruction_pointer: 0, out: Vec::new() };
    while state.instruction_pointer < program.len() {
        let ip = state.instruction_pointer;
        execute_instruction(&mut state, program[ip], program[ip + 1]);
    }
    println!("Part 1: {}", state.out.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","));
    let mut a_candidates = vec![0];
    for i in (0..program.len()).rev() {
        let truncated_program = program[i..].to_vec();
        let mut new_candidates = Vec::new();
        for head in a_candidates.iter() {
            for tail in 0..8u64 {
                let a_candidate = head << 3 | tail; // Assumes that the program is a loop that starts or ends with a 0,3 instruction (A = A / 8)
                let mut state = State { a: a_candidate, b, c, instruction_pointer: 0, out: Vec::new() };
                while state.instruction_pointer < program.len() && truncated_program.starts_with(&state.out) {
                    let ip = state.instruction_pointer;
                    execute_instruction(&mut state, program[ip], program[ip + 1]);
                }
                if state.out.eq(&truncated_program) {
                    new_candidates.push(a_candidate);
                }
            }
        }
        a_candidates = new_candidates;
    }
    println!("Part 2: {}", a_candidates.iter().min().unwrap());
}