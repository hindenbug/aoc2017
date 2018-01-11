use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut f = File::open("input.txt").expect("Unable to open");

    f.read_to_string(&mut input).expect("Something went wrong");
    println!("{}", sum1(&input));
    println!("{}", sum2(&input));
}

fn sum1(input: &str) -> u32 {
    let mut total = 0;

    for pair in input.chars().zip(input.chars().cycle().skip(1)) {
        let (x, y) = pair;
        if x == y {
            total += x.to_digit(10).unwrap();
        }
    }

    total
}


fn sum2(input: &str) -> u32 {
    let mut total = 0;

    for pair in input.chars().zip(input.chars().cycle().skip(input.len()/2)) {
        let (x, y) = pair;
        if x == y {
            total += x.to_digit(10).unwrap();
        }
    }

    total
}



// Part 1

#[test]
fn example_one() {
    assert_eq!(sum1("1122"), 3);
}

#[test]
fn example_two() {
    assert_eq!(sum1("1111"), 4);
}

#[test]
fn example_three() {
    assert_eq!(sum1("1234"), 0);
}

#[test]
fn example_four() {
    assert_eq!(sum1("91212129"), 9);
}

// Part 2

#[test]
fn example1() {
    assert_eq!(sum2("1212"), 6);
}

#[test]
fn example2() {
    assert_eq!(sum2("1221"), 0);
}

#[test]
fn example3() {
    assert_eq!(sum2("123425"), 4);
}

#[test]
fn example4() {
    assert_eq!(sum2("123123"), 12);
}
