use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut f = File::open("input.txt").expect("Unable to open");

    f.read_to_string(&mut input).expect("Something went wrong");
    println!("{}", sum(&input));
}

fn sum(input: &str) -> u32 {
    let mut total = 0;

    for pair in input.chars().zip(input.chars().cycle().skip(1)) {
        let (x, y) = pair;
        if x == y {
            total += x.to_digit(10).unwrap();
        }
    }

    total
}

#[test]
fn example_one() {
    // 1122 => 3
    assert_eq!(sum("1122"), 3);
}

#[test]
fn example_two() {
    assert_eq!(sum("1111"), 4);
}

#[test]
fn example_three() {
    assert_eq!(sum("1234"), 0);
}

#[test]
fn example_four() {
    assert_eq!(sum("91212129"), 9);
}
