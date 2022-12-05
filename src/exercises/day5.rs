use regex::Regex;

use crate::parser::{parse_aoc_file, parse_until_pattern};

#[derive(Debug)]
struct Instruction {
    number: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from(instruction: String) -> Instruction {
        let Ok(re) = Regex::new(r"move (\d*) from (\d*) to (\d*)") else {
            panic!("Could not parse regex pattern at Instruction::from");
        };
        let matches = re
            .captures(&instruction)
            .unwrap_or_else(|| panic!("Could not parse instruction {instruction}"));

        Instruction {
            number: matches
                .get(1)
                .unwrap_or_else(|| panic!("Unable to read number from instruction {instruction}"))
                .as_str()
                .parse()
                .unwrap_or_else(|e| {
                    panic!("Unable to parse number from instruction {instruction} with error : {e}")
                }),
            from: matches
                .get(2)
                .unwrap_or_else(|| panic!("Unable to read 'from' from instruction {instruction}"))
                .as_str()
                .parse::<usize>()
                .unwrap_or_else(|e| {
                    panic!("Unable to parse 'from' from instruction {instruction} with error : {e}")
                })
                - 1,
            to: matches
                .get(3)
                .unwrap_or_else(|| panic!("Unable to read 'to' from instruction {instruction}"))
                .as_str()
                .parse::<usize>()
                .unwrap_or_else(|e| {
                    panic!("Unable to parse 'to' from instruction {instruction} with error : {e}")
                })
                - 1,
        }
    }

    fn execute(&self, stacks: &mut [Vec<char>]) {
        let mut i = 0;
        loop {
            let from = stacks
                .get_mut(self.from)
                .unwrap_or_else(|| panic!("Unable to get stack {}", self.from));
            if i == self.number || from.is_empty() {
                break;
            }

            if let Some(item) = from.pop() {
                stacks
                    .get_mut(self.to)
                    .unwrap_or_else(|| panic!("Unable to get stack {}", self.to))
                    .push(item);
            }

            i += 1;
        }
    }

    fn execute_same_order(&self, stacks: &mut [Vec<char>]) {
        // Moving one stack is the same as above function
        if self.number <= 1 {
            self.execute(stacks);
            return;
        }

        let from = stacks
            .get_mut(self.from)
            .unwrap_or_else(|| panic!("Unable to get stack {}", self.from));
        let temp: &mut Vec<char> = &mut vec![];

        for _ in 0..self.number {
            if let Some(item) = from.pop() {
                temp.push(item);
            }
        }

        for _ in 0..self.number {
            if let Some(item) = temp.pop() {
                stacks
                    .get_mut(self.to)
                    .unwrap_or_else(|| panic!("Unable to get stack {}", self.to))
                    .push(item);
            }
        }
    }
}

fn parse_stacks(filename: &str) -> (Vec<Vec<char>>, Vec<String>) {
    let lines = parse_aoc_file(filename, None);
    let (mut stack_lines, remaining) = parse_until_pattern(lines, "^$");
    stack_lines.reverse();

    // Get first line
    let (first_line, stack_lines) = stack_lines
        .split_first()
        .unwrap_or_else(|| panic!("Unable to get first line from {filename}"));

    // Get the total number of stacks to compute
    let num_stacks = first_line
        .split_whitespace()
        .map(|s| {
            s.parse::<u32>()
                .unwrap_or_else(|_| panic!("Unable to parse {s} on the first line (not a number)"))
        })
        .max()
        .unwrap_or(0);

    // Init the stacks
    let mut stacks = Vec::<Vec<char>>::new();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }

    // Fill the stacks
    for line in stack_lines {
        let mut chars = line.chars();
        let i = 1;
        for j in 0..num_stacks {
            let char_index = (i + 4u32 * j) as usize;
            let char = chars.nth(if j == 0 { 1 } else { 3 }).unwrap_or_else(|| {
                panic!("Unable to get char at index {char_index} on line {line}")
            });
            if char != ' ' {
                stacks
                    .get_mut(j as usize)
                    .unwrap_or_else(|| panic!("Stack {j} does not exists"))
                    .push(char);
            }
        }
    }

    (stacks, remaining[1..].to_vec())
}

pub fn day_5_1(filename: &str) -> Vec<char> {
    let (mut stacks, instructions) = parse_stacks(filename);

    for instruction in instructions {
        Instruction::from(instruction).execute(&mut stacks);
    }

    stacks
        .iter()
        .map(|s| s.last().unwrap_or(&' '))
        .filter(|c| **c != ' ')
        .cloned()
        .collect()
}

pub fn day_5_2(filename: &str) -> Vec<char> {
    let (mut stacks, instructions) = parse_stacks(filename);

    for instruction in instructions {
        Instruction::from(instruction).execute_same_order(&mut stacks);
    }

    stacks
        .iter()
        .map(|s| s.last().unwrap_or(&' '))
        .filter(|c| **c != ' ')
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::{parse_stacks, Instruction};

    #[test]
    fn test_parse_stacks() {
        let parsed = parse_stacks("src/files/day5_1.test");
        assert_debug_snapshot!(parsed);
    }

    #[test]
    fn test_instruction_parsing() {
        let raw = "move 1 from 1 to 2".to_string();
        let instruction = super::Instruction::from(raw);
        assert_debug_snapshot!(instruction);
    }

    #[test]
    fn test_instruction_execute() {
        let mut stacks = vec![vec!['a'], vec![]];
        let instruction = Instruction::from("move 1 from 1 to 2".to_string());
        instruction.execute(&mut stacks);

        assert_debug_snapshot!(stacks);
    }

    #[test]
    fn test_day_5_1() {
        let result = super::day_5_1("src/files/day5_1.test");
        assert_debug_snapshot!(result);
    }

    #[test]
    fn test_day_5_2() {
        let result = super::day_5_2("src/files/day5_1.test");
        assert_debug_snapshot!(result);
    }
}
