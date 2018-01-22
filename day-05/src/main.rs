fn main() {
    let input = include_str!("../input.txt");
    // increase offset by 1 for part 1
    println!("{:?}", step_count_for(input, false));

    //decrease offset by 1 part 2
    println!("{:?}", step_count_for(input, true));
}

fn step_count_for(input: &str, decrease: bool) -> usize {
    let mut steps = 0;
    let mut instructions: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let mut current_offset: i32 = 0;

    while let Some(offset) = instructions.get_mut(current_offset as usize) {
        current_offset += *offset;
        if decrease && *offset >= 3 {
            *offset -= 1
        } else {
            *offset += 1
        }
        steps += 1;
    }

    steps
}

#[test]
fn example_1() {
    let input = include_str!("../example.txt");
    assert_eq!(step_count_for(input, false), 5)
}

#[test]
fn example_2() {
    let input = include_str!("../example.txt");
    assert_eq!(step_count_for(input, true), 10)
}
