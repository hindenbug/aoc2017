static INPUT: &'static str = include_str!("../input.txt");
static EXAMPLE: &'static str = include_str!("../example.txt");

fn main() {
    let input = INPUT;
    println!("{:#?}", knot_hash(input, 255));
}

fn knot_hash(input: &str, size: usize) -> usize {
    let lengths = input
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (mut current_pos, mut skip_size) = (0, 0);
    let mut list = (0..size + 1).collect::<Vec<_>>();

    for len in lengths {
        let mut sub_list = list.iter()
            .cloned()
            .cycle()
            .skip(current_pos)
            .take(len)
            .collect::<Vec<_>>();

        sub_list.reverse();

        for (i, value) in sub_list.iter().enumerate() {
            let index = (current_pos + i) % list.len();
            list[index] = *value;
        }

        current_pos = current_pos + len + skip_size;
        skip_size += 1;
    }

    list[0] * list[1]
}

#[test]
fn knot_hash_test() {
    assert_eq!(knot_hash(EXAMPLE, 4), 12)
}
