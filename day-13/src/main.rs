use std::collections::HashMap;

static INPUT: &'static str = include_str!("../input.txt");
static EXAMPLE: &'static str = include_str!("../example.txt");

fn main() {
    println!("{:?}", part1(INPUT));
    println!("{:?}", part2(INPUT));
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

fn part2(input: &str) -> u32 {
    let mut delay = 0;
    let mut caught = true;

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

    while caught {
        caught = false;
        delay += 1;

        for (index, depth) in details.iter() {
            if *depth == 1 || ((index + delay) % (depth * 2 - 2)) == 0 {
                caught = true;
                break;
            }
        }
    }

    delay
}

#[test]
fn test_part1() {
    assert_eq!(part1(EXAMPLE), 24)
}

#[test]
fn test_part2() {
    assert_eq!(part2(EXAMPLE), 10)
}
