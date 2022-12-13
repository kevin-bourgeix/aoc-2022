use std::cmp::Ordering;

use crate::parser::parse_aoc_file;

#[derive(Debug, Clone)]
enum Packet {
    Int(i32),
    Array(Vec<Packet>),
}

// it would be a good exercise to implement the Iterator trait for Packet and would simplify the code

impl Packet {
    fn from_vec(v: Vec<Packet>) -> Self {
        Self::Array(v)
    }
    fn slice(&self, i: usize, j: usize) -> Packet {
        match self {
            Self::Int(_) => panic!("Cannot slice an integer"),
            Self::Array(a) => Self::Array(a[i..j].to_vec()),
        }
    }
    fn first(&self) -> Packet {
        match self {
            Self::Int(_) => panic!("Cannot flatten an integer"),
            Self::Array(a) => a.first().unwrap_or(&Self::Array(vec![])).clone(),
        }
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Int(l0), Self::Int(r0)) => l0 == r0,
            (Self::Array(l0), Self::Array(r0)) => {
                l0.len() == r0.len() && l0.iter().enumerate().all(|(i, l)| *l == r0[i])
            }
            (Self::Int(l0), Self::Array(r0)) => r0 == &[Self::Int(*l0)],
            (Self::Array(l0), Self::Int(r0)) => l0 == &[Self::Int(*r0)],
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Int(i), Packet::Int(j)) => i.partial_cmp(j),
            (Packet::Array(a), Packet::Array(b)) => {
                if a.is_empty() && b.is_empty() {
                    return Some(Ordering::Equal);
                }
                let Some(a_item) = a.first() else {
                    return Some(Ordering::Less);
                };
                let Some(b_item) = b.first() else {
                    return Some(Ordering::Greater);
                };
                match a_item.partial_cmp(b_item) {
                    Some(Ordering::Equal) => {
                        self.slice(1, a.len()).partial_cmp(&other.slice(1, b.len()))
                    }
                    Some(Ordering::Less) => Some(Ordering::Less),
                    Some(Ordering::Greater) => Some(Ordering::Greater),
                    None => None,
                }
            }
            (Packet::Array(a), Packet::Int(i)) => a.partial_cmp(&vec![Packet::Int(*i)]),
            (Packet::Int(i), Packet::Array(a)) => vec![Packet::Int(*i)].partial_cmp(a),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        // safe unwrap because we theoretically never return None (as i32.partial_cmp(i32) never does)
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Packet {}

fn parse_file(filename: &str) -> Vec<String> {
    parse_aoc_file(filename, Some("\n\n"))
}

fn parse_line(line: String) -> Vec<Packet> {
    let mut curr: Vec<Packet> = Vec::new();
    let mut stack: Vec<Vec<Packet>> = Vec::new();
    let mut current_number: String = String::new();
    let mut col = 1;

    for c in line.chars() {
        match c {
            '[' => {
                stack.push(curr);
                curr = Vec::new();
            }
            ']' => {
                let Some(mut last) = stack.pop() else {
                    panic!("Unbalanced brackets at col {col}");
                };
                if !current_number.is_empty() {
                    let Ok(number) = current_number.parse() else {
                        panic!("Invalid number {current_number} read after ']' at col {col}");
                    };
                    curr.push(Packet::Int(number));
                    current_number = String::new();
                }
                last.push(Packet::Array(curr));
                curr = last;
            }
            ',' => {
                if !current_number.is_empty() {
                    let Ok(number) = current_number.parse() else {
                        panic!("Invalid number {current_number} read after ',' at col {col}");
                    };
                    curr.push(Packet::Int(number));
                    current_number = String::new();
                }
            }
            _ => {
                current_number.push(c);
            }
        }
        col += 1;
    }

    curr
}

fn is_right_order(pair: &str) -> bool {
    let lines = pair.split('\n');
    lines.map(|line| parse_line(line.to_string())).is_sorted()
}

pub fn day_13_1(filename: &str) -> usize {
    let pairs = parse_file(filename);
    pairs.iter().enumerate().fold(0, |acc, (i, pair)| {
        if is_right_order(pair) {
            acc + i + 1
        } else {
            acc
        }
    })
}

pub fn day_13_2(filename: &str) -> i32 {
    let mut packets: Vec<Packet> = parse_aoc_file(filename, None)
        .iter()
        .filter(|l| !l.is_empty())
        // not ideal as we put everything in an array to take it out after, but i don't want to debug the parser
        .map(|l| Packet::from_vec(parse_line(l.clone())).first())
        .collect();
    packets.sort();

    let (div_x, div_y) = packets
        .iter()
        .enumerate()
        .fold((-1, -1), |(x, y), (i, packet)| {
            if let Packet::Array(a) = packet && a.len() == 1 && let Packet::Array(b) = a.first().unwrap() && b.len() == 1 {
                if let Packet::Int(value) = b.first().unwrap() {
                    if *value == 2 {
                        return (i as i32 + 1, y)
                    } else if *value == 6  {
                        return (x, i as i32 + 1)
                    }
                }
            }
            (x, y)
        });
        
    div_x * div_y
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn test_day_13_1() {
        let result = day_13_1("./src/files/day13_1.test");
        assert_eq!(result, 13);
    }

    #[test]
    fn test_day_13_2() {
        let result = day_13_2("./src/files/day13_1.test");
        assert_eq!(result, 140);
    }

    #[test]
    fn test_packet_comparisons() {
        let a = Packet::Int(1);
        let b = Packet::Int(1);
        let c = Packet::Int(2);
        let d = Packet::Array(vec![Packet::Int(1), Packet::Int(2)]);
        let e = Packet::Array(vec![Packet::Int(1), Packet::Int(2)]);
        let f = Packet::Array(vec![Packet::Int(1), Packet::Int(3)]);
        let g = Packet::Array(vec![Packet::Int(1), Packet::Int(2), Packet::Int(3)]);
        let h = Packet::Array(vec![Packet::Int(1), Packet::Int(2), Packet::Int(3)]);
        let i = Packet::Array(vec![Packet::Int(1)]);
        let j = Packet::Array(vec![
            Packet::Array(vec![]),
            Packet::Int(2),
            Packet::Int(3),
            Packet::Int(4),
        ]);

        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_ne!(a, d);
        assert_eq!(d, e);
        assert_ne!(d, f);
        assert_ne!(d, g);
        assert_eq!(g, h);
        assert_eq!(a, i);
        assert_ne!(i, j);

        assert_eq!(a.partial_cmp(&b), Some(Ordering::Equal));
        assert_eq!(a.partial_cmp(&c), Some(Ordering::Less));
        assert_eq!(c.partial_cmp(&a), Some(Ordering::Greater));
        assert_eq!(d.partial_cmp(&e), Some(Ordering::Equal));
        assert_eq!(d.partial_cmp(&f), Some(Ordering::Less));
        assert_eq!(f.partial_cmp(&d), Some(Ordering::Greater));
        assert_eq!(a.partial_cmp(&h), Some(Ordering::Less));
        assert_eq!(i.partial_cmp(&a), Some(Ordering::Equal));
        assert_eq!(i.partial_cmp(&j), Some(Ordering::Greater));
    }

    #[test]
    fn test_parse_line() {
        let lines = vec![
            "[1,2,3]".to_string(),
            "[1,2,3,[4,5,[]]]".to_string(),
            "[9]".to_string(),
            "[[1],[2,3,4]]".to_string(),
            "[[1],[2,3,14]]".to_string(),
        ];
        assert_debug_snapshot!(lines
            .iter()
            .map(|line| parse_line(line.to_string()))
            .collect::<Vec<Vec<Packet>>>());
    }
}
