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
    let input = include_str!("../input.txt");
    println!("{:#?}", root(input));
}

fn root(input: &str) -> String {
    let re = Regex::new(r"(\w*) \((\d+)\)( -> ([\w, ]*)+)").unwrap();
    let mut programs = HashMap::new();
    let mut nodes = Vec::new();
    let mut root = String::new();

    for line in input.lines() {
        for capture in re.captures_iter(line) {
            let mut children = capture[4]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            let mut program = Program {
                name: capture[1].to_string(),
                weight: capture[2].parse().unwrap(),
                children: children,
                parent: None,
            };

            programs.insert(program.name.to_owned(), program.children);
        }
    }


    // This is ugly, must be a better way

    for (_k, v) in programs.iter() {
        nodes.push(v)
    }

    let nodes = nodes.iter().flat_map(|&x| x).collect::<Vec<_>>();

    for (k, _v) in programs.iter() {
        if !nodes.contains(&k) {
            root = k.to_string();
        }
    }

    root
}

#[test]
fn example() {
    let input = include_str!("../example.txt");
    assert_eq!(root(input), "tknk")
}
