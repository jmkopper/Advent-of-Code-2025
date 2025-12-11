use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::str::FromStr;

pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
    let machines: Vec<_> = input.lines().map(|x| parse_line(x).unwrap()).collect();

    println!("Part 1: {}", 123);
    println!("Part 2: {}", 456);

    Ok(())
}

struct Machine {
    lights: String,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn parse_numbers(s: &str) -> Vec<usize> {
    s.split(',')
        .filter(|t| !t.is_empty())
        .filter_map(|t| usize::from_str(t.trim()).ok())
        .collect()
}

fn parse_line(input: &str) -> Option<Machine> {
    let start = input.find('[')?;
    let end = input.find(']')?;
    let lights = input[start + 1..end].to_string();

    let rest = &input[end + 1..];
    let mut buttons = Vec::new();
    let mut joltage = Vec::new();

    let mut s = rest.trim();

    while !s.is_empty() {
        if let Some(i) = s.find('(') {
            let j = s[i + 1..].find(')')? + i + 1;
            let inside = &s[i + 1..j];
            buttons.push(parse_numbers(inside));
            s = &s[j + 1..];
        } else {
            break;
        }
    }

    if let Some(i) = s.find('{') {
        let j = s[i + 1..].find('}')? + i + 1;
        let inside = &s[i + 1..j];
        joltage = parse_numbers(inside);
    }

    Some(Machine {
        lights,
        buttons,
        joltage,
    })
}

fn shortest_path(machine: Machine) -> usize {
    let mut visited: HashSet<String> = HashSet::new();

    1
}
