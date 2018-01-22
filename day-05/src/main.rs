fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}", step_count_for(input));
}

fn step_count_for(input: &str) -> usize {
    let mut steps = 0;
    let mut instructions: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut current_offset: i32 = 0;

    while let Some(offset) = instructions.get_mut(current_offset as usize) {
        current_offset += *offset;
        *offset += 1;
        steps += 1;
    }

    steps
}

#[test]
fn example_1() {
    let input = include_str!("../example.txt");
    assert_eq!(step_count_for(input), 5)
}
