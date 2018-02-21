static INPUT: &'static str = include_str!("../input.txt");
static EXAMPLE: &'static str = include_str!("../example.txt");

fn main() {
    let input = INPUT;
    println!("{:#?}", knot_hash1(input, 255));
    println!("{:#?}", knot_hash2(input, 255));
}

fn knot_hash1(input: &str, size: usize) -> usize {
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

fn knot_hash2(input: &str, size: usize) -> String {
    let mut lengths: Vec<u8> = input.trim().chars().map(|c| c as u8).collect();
    lengths.extend(vec![17, 31, 73, 47, 23]);

    let (mut current_pos, mut skip_size) = (0, 0);
    let mut list = (0..size + 1).map(|e| e as u8).collect::<Vec<_>>();

    for _ in 0..64 {
        for len in lengths.iter() {
            let mut sub_list = list.iter()
                .cloned()
                .cycle()
                .skip(current_pos)
                .take(*len as usize)
                .collect::<Vec<_>>();

            sub_list.reverse();

            for (i, value) in sub_list.iter().enumerate() {
                let index = (current_pos + i) % list.len();
                list[index] = *value;
            }

            current_pos = current_pos + *len as usize + skip_size;
            skip_size += 1;
        }
    }

    list.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
        .map(|val| format!("{:02x}", val))
        .collect()
}

#[test]
fn knot_hash1_test() {
    assert_eq!(knot_hash1(EXAMPLE, 4), 12)
}

#[test]
fn knot_hash2_test() {
    assert_eq!(knot_hash2("", 255), "a2582a3a0e66e6e86e3812dcb672a272");
    assert_eq!(knot_hash2("1,2,3", 255), "3efbe78a8d82f29979031a4aa0b16a9d");
    assert_eq!(knot_hash2("1,2,4", 255), "63960835bcdc130f0b66d7ff4f6a5a8e");
    assert_eq!(
        knot_hash2("AoC 2017", 255),
        "33efeb34ea91902bb2f59c9920caa6cd"
    );
}
