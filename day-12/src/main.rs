use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &'static str = include_str!("../input.txt");
static EXAMPLE: &'static str = include_str!("../example.txt");

fn main() {
    println!("{:?}", part1(INPUT));
}

fn part1(input: &str) -> (u32, u32) {
    let inputs = input
        .trim()
        .lines()
        .map(|line| line.split(" <-> ").collect::<Vec<&str>>())
        .collect::<Vec<_>>();

    let m = inputs
        .iter()
        .map(|data| {
            let program = data[0].parse::<u32>().unwrap();
            let connections = data[1]
                .split(", ")
                .map(|d| d.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            (program, connections)
        })
        .collect::<HashMap<u32, Vec<u32>>>();

    let mut result = HashSet::new();
    result.insert(0);

    connected_children_of(0, &m, &mut result);
    let count = result.len() as u32;

    let mut groups = 1;

    for key in m.keys() {
        if result.contains(key) {
            continue;
        }
        connected_children_of(*key, &m, &mut result);
        groups += 1;
    }

    (count, groups)
}

fn connected_children_of(
    program: u32,
    input: &HashMap<u32, Vec<u32>>,
    mut result: &mut HashSet<u32>,
) {
    for c in input[&program].iter() {
        if !result.contains(c) {
            result.insert(*c);
            connected_children_of(*c, input, &mut result);
        }
    }
}

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE), 6)
}
