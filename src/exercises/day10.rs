use core::panic;

use crate::parser::parse_aoc_file;

#[derive(Debug)]
enum Operation {
    Addx(i32),
    Noop,
}

fn parse_file(filename: &str) -> Vec<Operation> {
    let lines = parse_aoc_file(filename, None);
    lines
        .iter()
        .enumerate()
        .flat_map(|(i, l)| {
            let splits = l.split(' ').collect::<Vec<&str>>();
            let Some(op) = splits.first() else {
            panic!("Can't read operation {l} at line {i}");
        };
            let value = splits.get(1);
            if *op != "addx" && value.is_some() {
                panic!("Unknown operation {op} at line {i}");
            }

            match *op {
                "addx" => vec![
                    Operation::Noop,
                    Operation::Addx(value.unwrap().parse::<i32>().unwrap_or_else(|_| {
                        panic!(
                            "Can't parse value {} at line {}",
                            value.unwrap_or(&"NONE"),
                            i
                        )
                    })),
                ],
                "noop" => vec![Operation::Noop],
                _ => panic!("Unknown operation {op} at line {i}"),
            }
        })
        .collect()
}

fn run_operations(operations: Vec<Operation>, signal_start: u32, signal_step: u32) -> i32 {
    let mut iter = 0;
    let mut register: i32 = 1;

    operations.iter().enumerate().fold(0, |acc, (step, op)| {
        let mut current = acc;
        if (step + 1) as u32 == signal_start + iter * signal_step {
            current += register * (step + 1) as i32;
            iter += 1;
        }

        if let Operation::Addx(value) = op {
            register += value;
        }

        current
    })
}

// works only with parse_file_v2
fn run_operations_screen(
    operations: Vec<Operation>,
    signal_size: i32,
    screen_height: u32,
) -> Vec<Vec<bool>> {
    let mut register: i32 = 1;
    let mut screen: Vec<Vec<bool>> = vec![];
    let mut ops = operations.iter().cycle();

    for _ in 0..screen_height {
        let line: Vec<bool> = (0..signal_size)
            .map(|i| {
                let mut value = false;

                // safe unwrap as ops is a cycle
                let op = ops.next().unwrap();
                if i == register - 1 || i == register || i == register + 1 {
                    value = true;
                }
                if let Operation::Addx(v) = op {
                    register += v;
                }
                value
            })
            .collect();
        screen.push(line);
    }

    screen
}

pub fn day_10_1(filename: &str) -> i32 {
    let operations = parse_file(filename);
    run_operations(operations, 20, 40)
}

pub fn day_10_2(filename: &str) -> Vec<Vec<bool>> {
    let operations = parse_file(filename);
    run_operations_screen(operations, 40, 6)
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn test_exec_op() {
        let operations = parse_file("src/files/day10_1.test");
        let total = run_operations(operations, 20, 40);
        assert_eq!(total, 13140);
    }

    #[test]
    fn test_exec_op_screen() {
        let operations = parse_file("src/files/day10_1.test");
        let screen = run_operations_screen(operations, 40, 6);
        assert_debug_snapshot!(screen);
    }
}
