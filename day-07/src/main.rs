extern crate regex;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Program {
    name: String,
    weight: i32,
    children: Vec<String>,
    parent: Option<String>,
}

fn main() {
    let input = include_str!("../example.txt");
    root(input);
}

fn root(input: &str) -> &str {
    let re = Regex::new(r"(\w*) \((\d+)\)( -> ([\w, ]*)+)").unwrap();
    let mut programs = HashMap::new();

    for line in input.lines() {
        for capture in re.captures_iter(line) {
            let mut program = Program {
                name: capture[1].to_string(),
                weight: capture[2].parse().unwrap(),
                children: capture[4]
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
                parent: None,
            };
            programs.insert(program.name.clone(), program);
        }
    }

    "tknk"
}

#[test]
fn example() {
    let input = include_str!("../example.txt");
    assert_eq!(root(input), "tknk")
}
