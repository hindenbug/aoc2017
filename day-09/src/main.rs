static INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let input = INPUT;
    println!("{:#?}", part1(input));
}

fn part1(input: &str) -> u32 {
    let mut total = 0;
    let mut depth = 0;
    let mut chrs = input.chars();

    while let Some(c) = chrs.next() {
        match c {
            '{' => depth += 1,
            '}' => {
                total += depth;
                depth -= 1;
            }
            '!' => {
                chrs.next();
            }
            '<' => while let Some(c) = chrs.next() {
                match c {
                    '!' => {
                        chrs.next();
                    }
                    '>' => break,
                    _ => {}
                }
            },
            _ => {}
        }
    }

    total
}

#[test]
fn part1_test_1() {
    assert_eq!(part1("{}"), 1)
}

#[test]
fn part1_test_2() {
    assert_eq!(part1("{{{}}}"), 6)
}

#[test]
fn part1_test_3() {
    assert_eq!(part1("{{},{}}"), 5)
}

#[test]
fn part1_test_4() {
    assert_eq!(part1("{{{},{},{{}}}}"), 16)
}

#[test]
fn part1_test_5() {
    assert_eq!(part1("{<a>,<a>,<a>,<a>}"), 1)
}

#[test]
fn part1_test_6() {
    assert_eq!(part1("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9)
}

#[test]
fn part1_test_7() {
    assert_eq!(part1("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9)
}

#[test]
fn part1_test_8() {
    assert_eq!(part1("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3)
}
