use std::collections::HashSet;


fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", count_cycles_for(input));

}

fn count_cycles_for(input: &str) -> usize {
    let mut bank = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut previous: HashSet<Vec<usize>> = HashSet::new();
    let mut cycles = 0;

    previous.insert(bank.clone());

    loop {
        let (mut index, max) = find_max_with_index(bank.clone());
        println!("{:?}", bank.clone());

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

fn find_max_with_index(bank: Vec<usize>) -> (usize, usize) {
    let (i, max) = bank.iter().enumerate().max_by(|x, y| x.1.cmp(y.1).then_with(|| y.0.cmp(&x.0))).unwrap();
    (i, *max)

}

#[test]
fn example() {
    let input = include_str!("../input.txt");
    assert_eq!(count_cycles_for(input), 5);
}

#[test]
fn test_find_max_with_index() {
    let input = vec![0, 2, 7, 0];
    assert_eq!(find_max_with_index(input), (2, 7 as usize))
}
