static INPUT: &'static str = include_str!("../input.txt");
static EXAMPLE: &'static str = include_str!("../example.txt");

fn main() {
    let input = INPUT;
    println!("{:#?}", knot_hash(input));
}

fn knot_hash(input: &str) -> u32 {
    let lengths = input.trim().split(",").map(|x| x.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    let (mut current_pos, mut skip_size) = (0, 0);
    let mut list = (0..256).map(|x| x as u8).collect::<Vec<u8>>();

    for length in lengths {
        let mut new_list = Vec::new();
        let (mut head, mut tail) = list.split_at_mut(current_pos);

        //tail[current_pos as usize..length as usize];
        current_pos += length as usize + skip_size;
        skip_size += 1
    }

    12
}

#[test]
fn knot_hash_test() {
    assert_eq!(knot_hash(EXAMPLE), 12)
}
