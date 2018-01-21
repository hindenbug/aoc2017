use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    let mut f = File::open("input.txt").expect("Unable to open");

    f.read_to_string(&mut input).expect("Something went wrong");

    println!("{}", input.lines().filter(|line| is_valid_1(line)).count());
    println!("{}", input.lines().filter(|line| is_valid_2(line)).count())
}

fn is_valid_1(passphrase: &str) -> bool {

    let mut freq = HashMap::new();
    let words = passphrase.split_whitespace();

    for word in words {
        let counter = freq.entry(word).or_insert(0);
        *counter += 1;
    }

    !freq.values().any(|&x| x > 1 )

}

fn is_valid_2(passphrase: &str) -> bool {

    let words = passphrase.split_whitespace();

    words.clone().enumerate().all(|(i1, word)| {
        for (i2, w) in words.clone().enumerate() {
            if i1 == i2 { continue; }
            if is_anagram(word, w) { return false }
        }
        true
    })

}


fn is_anagram(word1: &str, word2: &str) -> bool {
    if word1.len() != word2.len() {
        return false
    }

    let mut freq = HashMap::new();

    // could have used `sort` to make it easier, wanted to try another approach.

    for word in word1.chars() {
        let counter = freq.entry(word).or_insert(0);
        *counter +=1;
    }

    for word in word2.chars() {
        let counter = freq.entry(word).or_insert(0);
        *counter -= 1;
    }

    freq.values().all(|&x| x == 0 )
}


#[test]
fn test_is_anagram1() {
    assert_eq!(is_anagram("abcde", "ecdab"), true)
}

#[test]
fn test_is_anagram2() {
    assert_eq!(is_anagram("abcde", "fghij"), false)
}

// Tests for Part 1
#[test]
fn example_1() {
    assert_eq!(is_valid_1("aa bb cc dd ee"), true)
}

#[test]
fn example_2() {
    assert_eq!(is_valid_1("aa bb cc dd aa"), false)
}

#[test]
fn example_3() {
    assert_eq!(is_valid_1("aa bb cc dd aaa"), true)
}

// Tests for Part 2

#[test]
fn example_4() {
    assert_eq!(is_valid_2("abcde fghij"), true)
}

#[test]
fn example_5() {
    assert_eq!(is_valid_2("abcde xyz ecdab"), false)
}

#[test]
fn example_6() {
    assert_eq!(is_valid_2("a ab abc abd abf abj"), true)
}

#[test]
fn example_7() {
    assert_eq!(is_valid_2("iiii oiii ooii oooi oooo"), true)
}
