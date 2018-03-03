use std::collections::HashMap;

static INPUT: &'static str = include_str!("../input.txt");
static EXAMPLE: &'static str = include_str!("../example.txt");

fn main() {
    println!("{:?}", part1(INPUT));
}

fn part1(input: &str) -> u32 {
    let mut severity = 0;

    let details: HashMap<u32, u32> = input
        .lines()
        .map(|line| {
            let pts: Vec<_> = line.split(": ").collect();
            (
                pts[0].parse::<u32>().unwrap(),
                pts[1].parse::<u32>().unwrap(),
            )
        })
        .collect();

    for (index, depth) in details.iter() {
        if *depth == 1 || (index % (depth * 2 - 2)) == 0 {
            severity += *index * *depth
        }
    }

    severity
}

#[test]

fn test_part1() {
    assert_eq!(part1(EXAMPLE), 24)
}
