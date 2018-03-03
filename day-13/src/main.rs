use std::collections::HashMap;

static INPUT: &'static str = include_str!("../input.txt");
static EXAMPLE: &'static str = include_str!("../example.txt");

fn main() {
    println!("#{:?}", part1(EXAMPLE));
}

fn part1(input: &str) -> usize {
    let details = input.lines()
        .map(|line| {
            line.split(": ")
                .map(|x| x.parse::<usize>().unwrap());

        })
        .collect::<Vec<_>>();

    for detail in details.into_iter() {
        detail
    }

    println!("{:?}", details);
    24
}


#[test]

fn test_part1() {
    assert_eq!(part1(EXAMPLE), 24)
}
