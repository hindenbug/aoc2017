extern crate regex;

use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Program {
    name: String,
    weight: i32,
    children: Vec<String>,
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{:#?}", root(input));
    println!("{:#?}", balanced_weight(input));
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

fn balanced_weight(input: &str) {
    let mut programs = HashMap::new();
    let root = root(input);
    let re1 = Regex::new(r"(\w*) \((\d+)\)( -> ([\w, ]*)+)").unwrap();
    let re2 = Regex::new(r"(\w*) \((\d+)\)").unwrap();

    for line in input.lines() {
        if re1.is_match(line) {
            for capture in re1.captures_iter(line) {
                let mut children = capture[4]
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();

                let mut program = Program {
                    name: capture[1].to_string(),
                    weight: capture[2].parse().unwrap(),
                    children: children,
                };

                programs.insert(program.name.to_owned(), program);
            }
        } else if re2.is_match(line) {
            for capture in re2.captures_iter(line) {
                let mut program = Program {
                    name: capture[1].to_string(),
                    weight: capture[2].parse().unwrap(),
                    children: Vec::new(),
                };

                programs.insert(program.name.to_owned(), program);
            }
        }
    }

    evaluate(&root, &programs);
}

fn evaluate(name: &str, programs: &HashMap<String, Program>) {
    let program = programs.get(name).unwrap();
    let weights: Vec<i32> = program
        .children
        .iter()
        .map(|x| get_weight(x, programs))
        .collect();

    for w in weights.iter() {
        if *w != weights[0] {
            println!("Mismatch {} != {}", w, weights[0]);
            println!("Diff {}", (w - weights[0]).abs());
            break;
        }
    }

    if !program.children.is_empty() {
        println!(
            "{} ({}) -> {:?}",
            name,
            program.weight,
            program.children.iter().zip(weights).collect::<Vec<_>>()
        );
    } else {
        println!("{} ({})", name, program.weight);
    }

    for c in program.children.iter() {
        evaluate(c, programs);
    }
}

fn get_weight(name: &str, programs: &HashMap<String, Program>) -> i32 {
    let mut p = programs.get(name).unwrap();
    let mut weight = p.weight;

    for c in p.children.iter() {
        weight += get_weight(c, programs);
    }
    weight
}

#[test]
fn example() {
    let input = include_str!("../example.txt");
    assert_eq!(root(input), "tknk")
}
