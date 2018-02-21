use std::cmp::max;

static INPUT: &'static str = include_str!("../input.txt");

#[derive(Default)]
struct Location {
    x: i32,
    y: i32,
    z: i32,
}

impl Location {
    fn walk(&mut self, steps: &str) {
        match steps {
            "n" => {
                self.y += 1;
                self.z -= 1;
            }
            "s" => {
                self.y -= 1;
                self.z += 1;
            }
            "ne" => {
                self.x += 1;
                self.z -= 1;
            }
            "sw" => {
                self.x -= 1;
                self.z += 1;
            }
            "nw" => {
                self.y += 1;
                self.x -= 1;
            }
            "se" => {
                self.x += 1;
                self.y -= 1;
            }
            _ => panic!("Error"),
        }
    }

    fn distance(&mut self) -> i32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) / 2
    }
}

fn main() {
    println!("{:?}", steps(INPUT));
    println!("{:?}", max_distance(INPUT));
}

fn steps(input: &str) -> i32 {
    let mut location = Location::default();

    for step in input.trim().split(',') {
        location.walk(step);
    }

    location.distance()
}

fn max_distance(input: &str) -> i32 {
    let mut location = Location::default();

    input.trim().split(',').fold(0, |distance, step| {
        location.walk(step);
        max(distance, location.distance())
    })
}

#[test]
fn test_steps() {
    assert_eq!(steps("ne,ne,ne"), 3);
    assert_eq!(steps("ne,ne,sw,sw"), 0);
    assert_eq!(steps("ne,ne,s,s"), 2);
    assert_eq!(steps("se,sw,se,sw,sw"), 3);
}
