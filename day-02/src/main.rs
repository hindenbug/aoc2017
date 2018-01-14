use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("{}", checksum1("input1.txt"));
    println!("{}", checksum2("input2.txt"));
}

fn checksum1(filename: &str) -> u32 {
    let f = BufReader::new(File::open(filename).expect("Cannot open file"));
    let mut checksums: Vec<u32> = Vec::new();

    for line in f.lines() {
        let v: Vec<u32> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        checksums.push(v.iter().max().unwrap() - v.iter().min().unwrap());
    }

    checksums.iter().fold(0, |acc, &x| acc + x)
}

fn checksum2(filename: &str) -> u32 {
    let f = BufReader::new(File::open(filename).expect("Cannot open file"));
    let mut checksums: Vec<u32> = Vec::new();

    for line in f.lines() {
        let mut v: Vec<u32> = line.unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        'outer: loop {
            let x = v.pop().expect("Some Error");

            for i in 0..(v.len()) {
                let y = v[i];

                if x > y && x % y == 0 {
                    checksums.push(x / y);
                    break 'outer;
                } else if y > x && y % x == 0 {
                    checksums.push(y / x);
                    break 'outer;
                }
            }
        }
    }

    checksums.iter().fold(0, |acc, &x| acc + x)
}

#[test]
fn example1() {
    assert_eq!(checksum1("example1.txt"), 18)
}

#[test]
fn example2() {
    assert_eq!(checksum2("example2.txt"), 9)
}
