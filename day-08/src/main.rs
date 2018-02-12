use std::collections::HashMap;

static INPUT: &'static str = include_str!("../input.txt");
static EXAMPLE: &'static str = include_str!("../example.txt");

fn main() {
    let input = INPUT;
    println!("{:#?}", part1(input));
    println!("{:#?}", part2(input));
}

#[derive(Debug)]
enum Operation {
    Increment,
    Decrement,
}

impl Operation {
    fn from_str(input: &str) -> Operation {
        match input {
            "inc" => Operation::Increment,
            "dec" => Operation::Decrement,
            _ => panic!("Error"),
        }
    }
}

#[derive(Debug)]
enum Operator {
    GreaterThan,
    EqualTo,
    GreaterThanEqualTo,
    LessThan,
    LessThanEqualTo,
    NotEqualTo,
}

impl Operator {
    fn from_str(input: &str) -> Operator {
        match input {
            ">" => Operator::GreaterThan,
            ">=" => Operator::GreaterThanEqualTo,
            "==" => Operator::EqualTo,
            "<" => Operator::LessThan,
            "<=" => Operator::LessThanEqualTo,
            "!=" => Operator::NotEqualTo,
            _ => panic!("Error"),
        }
    }
}

#[derive(Debug)]
struct Condition {
    register: String,
    operator: Operator,
    value: i32,
}

#[derive(Debug)]
struct Instruction {
    register: String,
    operation: Operation,
    delta: i32,
    condition: Condition,
}

impl Instruction {
    fn from(instruction: &str) -> Instruction {
        let input = instruction.split_whitespace().collect::<Vec<&str>>();

        Instruction {
            register: input[0].to_owned(),
            operation: Operation::from_str(input[1]),
            delta: input[2].parse::<i32>().unwrap(),
            condition: Condition {
                register: input[4].to_owned(),
                operator: Operator::from_str(input[5]),
                value: input[6].parse::<i32>().unwrap(),
            },
        }
    }
}

fn part1(input: &str) -> i32 {
    let instructions = input
        .lines()
        .map(|i| Instruction::from(i))
        .collect::<Vec<Instruction>>();

    let mut registers = HashMap::new();

    instructions.iter().for_each(|i| {
        let old = *registers
            .entry(i.condition.register.to_owned())
            .or_insert(0);

        let cond = match i.condition.operator {
            Operator::GreaterThan => old > i.condition.value,
            Operator::EqualTo => old == i.condition.value,
            Operator::GreaterThanEqualTo => old >= i.condition.value,
            Operator::LessThan => old < i.condition.value,
            Operator::LessThanEqualTo => old <= i.condition.value,
            Operator::NotEqualTo => old != i.condition.value,
        };

        let reg = *registers.entry(i.register.to_owned()).or_insert(0);

        if cond {
            let new = match i.operation {
                Operation::Increment => reg + i.delta,
                Operation::Decrement => reg - i.delta,
            };

            registers.insert(i.register.to_owned(), new);
        }
    });

    let max = registers.iter().max_by_key(|x| x.1).unwrap();
    *max.1
}

fn part2(input: &str) -> i32 {
    let mut highest = 0;

    let instructions = input
        .lines()
        .map(|i| Instruction::from(i))
        .collect::<Vec<Instruction>>();

    let mut registers = HashMap::new();

    instructions.iter().for_each(|i| {
        let old = *registers
            .entry(i.condition.register.to_owned())
            .or_insert(0);

        let cond = match i.condition.operator {
            Operator::GreaterThan => old > i.condition.value,
            Operator::EqualTo => old == i.condition.value,
            Operator::GreaterThanEqualTo => old >= i.condition.value,
            Operator::LessThan => old < i.condition.value,
            Operator::LessThanEqualTo => old <= i.condition.value,
            Operator::NotEqualTo => old != i.condition.value,
        };

        let reg = *registers.entry(i.register.to_owned()).or_insert(0);

        if cond {
            let new = match i.operation {
                Operation::Increment => reg + i.delta,
                Operation::Decrement => reg - i.delta,
            };

            registers.insert(i.register.to_owned(), new);
            if new > highest {
                highest = new;
            }
        }
    });

    highest
}

#[test]
fn example() {
    assert_eq!(part1(EXAMPLE), 1)
}
