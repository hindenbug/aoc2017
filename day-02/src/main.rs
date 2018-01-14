use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    println!("{}", checksum1("input.txt"));
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

    checksums.iter()
        .fold(0, |acc, &x| acc + x)

}

#[test]

fn example1() {
    assert_eq!(checksum1("example1.txt"), 18)
}
