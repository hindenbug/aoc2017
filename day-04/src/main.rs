use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut f = File::open("input.txt").expect("Unable to open");

    f.read_to_string(&mut input).expect("Something went wrong");

    println!("{}", input.lines().filter(|line| is_valid(line)).count())
}

fn is_valid(passphrase: &str) -> bool {

    let mut freq = HashMap::new();
    let phrase = passphrase.split_whitespace();

    for word in phrase {
        let counter = freq.entry(word).or_insert(0);
        *counter += 1;
    }

    !freq.values().any(|&x| x > 1 )

}


#[test]
fn example_1() {
    assert_eq!(is_valid("aa bb cc dd ee"), true)
}

#[test]
fn example_2() {
    assert_eq!(is_valid("aa bb cc dd aa"), false)
}

#[test]
fn example_3() {
    assert_eq!(is_valid("aa bb cc dd aaa"), true)
}
