use core::panic;

use regex::Regex;

use crate::parser::parse_aoc_file;

type NumberSize = u64;
type UnsignedNumberSize = i64;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(OperationValue),
    Mul(OperationValue),
}

#[derive(Debug, Clone, Copy)]
enum OperationValue {
    Old,
    Number(NumberSize),
}

#[derive(Debug, Clone)]
struct Item {
    multiplication: Vec<NumberSize>,
    additions: Vec<NumberSize>,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<NumberSize>,
    operation: Operation,
    test_divisible_by: NumberSize,
    true_monkey: NumberSize,
    false_monkey: NumberSize,
    pub inspected: NumberSize,
}

impl Monkey {
    fn new(
        items: Vec<NumberSize>,
        operation: Operation,
        test_divisible_by: NumberSize,
        true_monkey: NumberSize,
        false_monkey: NumberSize,
    ) -> Monkey {
        Monkey {
            items,
            operation,
            test_divisible_by,
            true_monkey,
            false_monkey,
            inspected: 0,
        }
    }

    fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    fn push_item(&mut self, item: NumberSize) {
        self.items.push(item);
    }

    fn inspect(&mut self, worry_divider: Option<NumberSize>) -> (NumberSize, NumberSize) {
        self.inspected += 1;
        self.items.reverse();
        let Some(item) = self.items.pop() else {
            panic!("Can't inspect monkey without items");
        };
        self.items.reverse();
        let divider = worry_divider.unwrap_or(3);
        let worry = match self.operation {
            Operation::Add(v) => match v {
                OperationValue::Old => item * 2,
                OperationValue::Number(n) => item + n,
            },
            Operation::Mul(v) => match v {
                OperationValue::Old => item * item,
                OperationValue::Number(n) => item * n,
            },
        } / divider;
        if worry % self.test_divisible_by == 0 {
            (worry, self.true_monkey)
        } else {
            (worry, self.false_monkey)
        }
    }
}

fn parse_file(filename: &str) -> Vec<Monkey> {
    let raw = parse_aoc_file(filename, Some("\n\n"));
    let Ok(regex) = Regex::new(r"Monkey \d+:
  Starting items: ([0-9, ]+)
  Operation: new = old (\+|\*) (\d+|old)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)") else {
        panic!("Can't compile regex");
    };

    raw.iter()
        .map(|r| {
            let captures = regex
                .captures(r)
                .unwrap_or_else(|| panic!("Failed parsing {r}"));
            let items = captures
                .get(1)
                .unwrap_or_else(|| panic!("Can't read items list from {r}"))
                .as_str()
                .split(", ")
                .map(|s| {
                    s.parse::<NumberSize>()
                        .unwrap_or_else(|_| panic!("Can't parse number {s} in items list"))
                })
                .collect::<Vec<NumberSize>>();
            let operation_value = match captures
                .get(3)
                .unwrap_or_else(|| panic!("Can't read operation value from {r}"))
                .as_str()
            {
                "old" => OperationValue::Old,
                _ => OperationValue::Number(
                    captures
                        .get(3)
                        .unwrap_or_else(|| panic!("Can't read operation number from {r}"))
                        .as_str()
                        .parse::<NumberSize>()
                        .unwrap_or_else(|_| panic!("Can't parse operation number from {r}")),
                ),
            };
            let operation = match captures
                .get(2)
                .unwrap_or_else(|| panic!("Can't read operation from {r}"))
                .as_str()
            {
                "+" => Operation::Add(operation_value),
                "*" => Operation::Mul(operation_value),
                _ => panic!("Unknown operation"),
            };
            let test_divisible_by = captures
                .get(4)
                .unwrap_or_else(|| panic!("Can't read divisble_by from {r}"))
                .as_str()
                .parse::<NumberSize>()
                .unwrap();
            let true_monkey = captures
                .get(5)
                .unwrap_or_else(|| panic!("Can't read if true number from {r}"))
                .as_str()
                .parse::<NumberSize>()
                .unwrap();
            let false_monkey = captures
                .get(6)
                .unwrap_or_else(|| panic!("Can't read if false number from {r}"))
                .as_str()
                .parse::<NumberSize>()
                .unwrap();

            Monkey::new(
                items,
                operation,
                test_divisible_by,
                true_monkey,
                false_monkey,
            )
        })
        .collect()
}

fn compute_monkeys(monkeys: &mut Vec<Monkey>, rounds: NumberSize, worry_divider: Option<NumberSize>) -> NumberSize {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].has_items() {
                let (worry, index) = monkeys[i].inspect(worry_divider);
                monkeys[index as usize].push_item(worry);
            }
        }
    }

    monkeys.sort_by_key(|m| 0 - m.inspected as UnsignedNumberSize);
    monkeys[0].inspected * monkeys[1].inspected
}

pub fn day_11_1(filename: &str) -> NumberSize {
    let mut monkeys = parse_file(filename);
    compute_monkeys(&mut monkeys, 20, None)
}

pub fn day_11_2(filename: &str) -> NumberSize {
    let mut monkeys = parse_file(filename);
    compute_monkeys(&mut monkeys, 10000, Some(1))
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::exercises::day11::{parse_file, compute_monkeys, day_11_2};

    use super::day_11_1;

    #[test]
    fn test_parse_monkeys() {
        let monkeys = parse_file("src/files/day11_1.test");
        assert_debug_snapshot!(monkeys);
    }

    #[test]
    fn test_day_11_1() {
        let mut monkeys = day_11_1("src/files/day11_1.test");
        assert_eq!(monkeys, 10605);
    }

    #[test]
    fn test_day_11_2() {
        let mut monkeys = day_11_2("src/files/day11_1.test");
        assert_eq!(monkeys, 2713310158);
    }
}
