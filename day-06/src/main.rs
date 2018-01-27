use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", redistribute_1(input));
    println!("{:?}", redistribute_2(input));

}

fn redistribute_1(input: &str) -> usize {
    let mut bank = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut previous: HashSet<Vec<usize>> = HashSet::new();
    let mut cycles = 0;

    previous.insert(bank.clone());

    loop {
        let (mut index, max) = find_max_with_index(bank.clone());

        let mut memory = max;
        bank[index] -= max;

        for idx in (0..bank.len()).cycle().skip(index+1) {
            bank[idx] += 1;
            memory -= 1;
            if memory == 0 { break;}
        }

        cycles += 1;

        if !previous.insert(bank.clone()) { break; }
    }

    cycles
}

fn redistribute_2(input: &str) -> usize {
    let mut bank = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut previous: HashMap<Vec<usize>, usize> = HashMap::new();
    let mut cycles = 0;

    previous.insert(bank.clone(), cycles);

    loop {
        let (mut index, max) = find_max_with_index(bank.clone());

        let mut memory = max;
        bank[index] -= max;

        for idx in (0..bank.len()).cycle().skip(index+1) {
            bank[idx] += 1;
            memory -= 1;
            if memory == 0 { break;}
        }

        cycles += 1;

        if previous.contains_key(&bank.clone()) {
            break;
        }

        previous.insert(bank.clone(), cycles);
    }
    cycles - previous.get(&bank).unwrap()
}


fn find_max_with_index(bank: Vec<usize>) -> (usize, usize) {
    let (i, max) = bank.iter().enumerate().max_by(|x, y| x.1.cmp(y.1).then_with(|| y.0.cmp(&x.0))).unwrap();
    (i, *max)

}

#[test]
fn example_1() {
    let input = include_str!("../example.txt");
    assert_eq!(redistribute_1(input), 5);
}

#[test]
fn example_2() {
    let input = include_str!("../example.txt");
    assert_eq!(redistribute_2(input), 4);
}

#[test]
fn test_find_max_with_index() {
    let input = vec![0, 2, 7, 0];
    assert_eq!(find_max_with_index(input), (2, 7 as usize))
}
