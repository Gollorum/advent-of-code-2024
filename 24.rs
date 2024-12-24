extern crate core;

use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use itertools::Itertools;
use std::fs;

const INPUT_ACTUAL: &str = include_str!("inputs/24.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/24_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

#[derive(Debug, Clone, PartialEq, Eq)]
enum GateMode { AND, OR, XOR, }

impl Display for GateMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            GateMode::AND => "AND".to_string(),
            GateMode::OR => "OR".to_string(),
            GateMode::XOR => "XOR".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
struct Gate {
    input1: String,
    input2: String,
    mode: GateMode,
    output: String,
}

impl FromStr for Gate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let gate_regex = regex::Regex::new(r"([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z0-9]+)").unwrap();
        let captures = gate_regex.captures(s).unwrap();
        let mut input1 = captures.get(1).unwrap().as_str().to_string();
        let mode = match captures.get(2).unwrap().as_str() {
            "AND" => GateMode::AND,
            "OR" => GateMode::OR,
            "XOR" => GateMode::XOR,
            _ => panic!(),
        };
        let mut input2 = captures.get(3).unwrap().as_str().to_string();
        let output = rap(captures.get(4).unwrap().as_str());
        if input1.cmp(&input2) == std::cmp::Ordering::Greater {
            (input1, input2) = (input2, input1);
        }
        Ok(Gate { input1, input2, mode, output })
    }
}

fn rap(s: &str) -> String {
    match s {
        "z06" => "ksv".to_string(),
        "ksv" => "z06".to_string(),
        _ => s.to_string()
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} -> {}", self.input1, self.mode, self.input2, self.output)
    }
}

fn main() {
    let (initials_str, gates_str) = INPUT.trim().split_once("\n\n").unwrap();
    let initials = initials_str.lines().map(|line| line.split_once(": ").unwrap()).map(|(a, b)| (a.to_string(), b.parse::<u8>().unwrap() == 1)).collect::<HashMap<String, bool>>();
    let gates = gates_str.lines().map(|line| line.parse::<Gate>().unwrap()).collect::<Vec<Gate>>();
    let mut gates_to_eval = gates.clone();
    let mut known_values = initials.clone();
    let mut last_gates = gates_to_eval.len();
    while !gates_to_eval.is_empty() {
        let mut gates_to_remove = Vec::new();
        for gate in gates_to_eval.iter().cloned() {
            if let Some((i1, i2)) = known_values.get(gate.input1.as_str()).and_then(|i1| known_values.get(gate.input2.as_str()).map(|i2| (i1, i2))) {
                let result = match gate.mode {
                    GateMode::AND => i1 & i2,
                    GateMode::OR => i1 | i2,
                    GateMode::XOR => i1 ^ i2,
                };
                known_values.insert(gate.output.clone(), result);
                gates_to_remove.push(gate.output.clone());
            }
        }
        gates_to_eval.retain(|gate| !gates_to_remove.contains(&gate.output));
        if gates_to_eval.len() == last_gates {
            println!("Stuck");
            break;
        }
        last_gates = gates_to_eval.len();
    }
    let outputs = known_values.iter().filter(|(k, _)| k.starts_with('z')).sorted_by_key(|(k, _)| *k).map(|(_, v)| v).collect::<Vec<&bool>>();
    let result = outputs.iter().rfold(0u64, |acc, &v| acc << 1 | if *v { 1 } else { 0 });
    println!("Part 1: {:?}", result);

    let mut renaming_map = HashMap::new();

    for gate in gates.iter() {
        if gate.input1.starts_with('x') && !gate.output.starts_with('z') {
            let pref = if gate.mode == GateMode::AND { "ÜB_RAW_" } else { "NM_RAW_" };
            renaming_map.insert(gate.output.clone(), pref.to_owned() + &gate.input1[1..]);
        }
    }

    for gate in gates.iter() {
        if gate.mode == GateMode::AND {
            let pref = "ÜB_PRE_";
            let pref1 = "ÜB_COL_";
            if let Some(i1nm) = renaming_map.get(&gate.input1).map(|s| s.clone()) {
                let to_replace = &gate.input2;
                renaming_map.insert(to_replace.clone(), pref.to_owned() + &i1nm[7..]);
                renaming_map.insert(gate.output.clone(), pref1.to_owned() + &i1nm[7..]);
            } else if let Some(i2nm) = renaming_map.get(&gate.input2).map(|s| s.clone()) {
                let to_replace = &gate.input1;
                renaming_map.insert(to_replace.clone(), pref.to_owned() + &i2nm[7..]);
                renaming_map.insert(gate.output.clone(), pref1.to_owned() + &i2nm[7..]);
            }
        }
    }

    let gates = gates.iter().map(|gate| {
        let mut input1 = renaming_map.get(&gate.input1).unwrap_or(&gate.input1).to_string();
        let mut input2 = renaming_map.get(&gate.input2).unwrap_or(&gate.input2).to_string();
        if input1.cmp(&input2) == std::cmp::Ordering::Greater {
            (input1, input2) = (input2, input1);
        }
        let output = renaming_map.get(&gate.output).unwrap_or(&gate.output).to_string();
        Gate { input1, input2, mode: gate.mode.clone(), output }
    }).collect::<Vec<Gate>>();

    let data = gates.iter()
        .sorted_by_key(|g| g.output.clone())
        .sorted_by_key(|g| g.input2.clone())
        .map(|gate| gate.to_string())
        .join("\n");
    fs::write("tmp/24.txt", data).expect("Unable to write file");
    let result = vec!["ksv", "z06", "kbs", "nbd", "tqq", "z20","ckb", "z39"]
        .iter().sorted()
        .join(",");
    println!("Part 2: {:?}", result);
}