static INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let input = INPUT;
    println!("{:#?}", score(input));
}

fn score(input: &str) -> (u32, u32) {
    let mut total = 0;
    let mut depth = 0;
    let mut garbage = 0;
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
                    _ => garbage += 1,
                }
            }
            _ => {},
        }
    }

    (total, garbage)
}

#[test]
fn score_test_1() {
    assert_eq!(score("{}"), 1)
}

#[test]
fn score_test_2() {
    assert_eq!(score("{{{}}}"), 6)
}

#[test]
fn score_test_3() {
    assert_eq!(score("{{},{}}"), 5)
}

#[test]
fn score_test_4() {
    assert_eq!(score("{{{},{},{{}}}}"), 16)
}

#[test]
fn score_test_5() {
    assert_eq!(score("{<a>,<a>,<a>,<a>}"), 1)
}

#[test]
fn score_test_6() {
    assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9)
}

#[test]
fn score_test_7() {
    assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9)
}

#[test]
fn score_test_8() {
    assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3)
}
