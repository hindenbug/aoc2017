use std::collections::HashMap;
use std::collections::HashSet;

static INPUT: &'static str = include_str!("../input.txt");
static EXAMPLE: &'static str = include_str!("../example.txt");

fn main() {
    println!("{:?}", part1(INPUT));
}

fn part1(input: &str) -> u32 {
    let inputs = input.trim().split(" <-> ").collect::<Vec<&str>>();

    let mut m = for data in inputs.iter() {
        let program = data[0].parse::<u32>().unwrap();
        let connections = data[1]
            .split(", ")
            .map(|d| d.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        (program, connections)
    }.collect::<HashMap<u32, Vec<u32>>>();

    // WIP

    6
}

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE), 6)
}
