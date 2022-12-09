use std::cmp::{max, min};

use crate::parser::parse_aoc_file;

type Position = (i32, i32);

enum Move {
    L(u32),
    R(u32),
    U(u32),
    D(u32),
}

fn get_moves(filename: &str) -> Vec<Move> {
    let lines = parse_aoc_file(filename, None);
    lines
        .iter()
        .map(|l| {
            let splits = l.split(' ').collect::<Vec<&str>>();
            let first = splits
                .first()
                .unwrap_or_else(|| panic!("Unable to get direction from {l}"));
            let second = splits
                .last()
                .unwrap_or_else(|| panic!("Unable to get value from {l}"));
            let val = second
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Unable to parse {second}"));
            match *first {
                "L" => Move::L(val),
                "R" => Move::R(val),
                "U" => Move::U(val),
                "D" => Move::D(val),
                _ => panic!("Unknown direction"),
            }
        })
        .collect()
}

fn distance(p1: &Position, p2: &Position) -> f32 {
    let dx = (p1.0 - p2.0).abs();
    let dy = (p1.1 - p2.1).abs();
    let min = min(dx, dy);
    let max = max(dx, dy);
    let straight_path = max - min;

    f32::sqrt(2f32) * min as f32 + straight_path as f32
}

fn recompute_rope_tail(head: &Position, tail: &mut Position) {
    if distance(head, tail) <= 1f32 {
        return;
    }

    let dx = (tail.0 - head.0).abs();
    let dy = (tail.1 - head.1).abs();

    if dx > dy {
        if tail.0 < head.0 {
            tail.0 = head.0 - 1;
        }
        if tail.0 > head.0 {
            tail.0 = head.0 + 1;
        }
        tail.1 = head.1;
    }
    if dx < dy {
        if tail.1 < head.1 {
            tail.1 = head.1 - 1;
        }
        if tail.1 > head.1 {
            tail.1 = head.1 + 1;
        }
        tail.0 = head.0;
    }
}

fn recompute_rope(
    head: &mut Position,
    tail: &mut Position,
    all_positions: &mut Vec<Position>,
    movement: Move,
) {
    match movement {
        Move::L(v) => {
            for _ in 0..v {
                head.0 -= 1;
                recompute_rope_tail(head, tail);
                all_positions.push(*tail);
            }
        }
        Move::R(v) => {
            for _ in 0..v {
                head.0 += 1;
                recompute_rope_tail(head, tail);
                all_positions.push(*tail);
            }
        }
        Move::U(v) => {
            for _ in 0..v {
                head.1 -= 1;
                recompute_rope_tail(head, tail);
                all_positions.push(*tail);
            }
        }
        Move::D(v) => {
            for _ in 0..v {
                head.1 += 1;
                recompute_rope_tail(head, tail);
                all_positions.push(*tail);
            }
        }
    }
}

fn recompute_rope_tail_multiple(mut rope: Vec<Position>, index: usize) -> Vec<Position> {
    let mut positions = Vec::new();
    let mut dx = (rope[index + 1].0 - rope[index].0).abs();
    let mut dy = (rope[index + 1].1 - rope[index].1).abs();

    while distance(&rope[index], &rope[index + 1]) > f32::sqrt(2f32) {
        if dx > 0 {
            if rope[index + 1].0 < rope[index].0 {
                rope[index + 1].0 += 1;
            }
            if rope[index + 1].0 > rope[index].0 {
                rope[index + 1].0 -= 1;
            }
        }

        if dy > 0 {
            if rope[index + 1].1 < rope[index].1 {
                rope[index + 1].1 += 1;
            }
            if rope[index + 1].1 > rope[index].1 {
                rope[index + 1].1 -= 1;
            }
        }

        dx = (rope[index + 1].0 - rope[index].0).abs();
        dy = (rope[index + 1].1 - rope[index].1).abs();

        positions.push(rope[index + 1]);
    }

    positions
}

fn recompute_rope_multiple(
    rope: &mut Vec<Position>,
    all_positions: &mut Vec<Position>,
    movement: &Move,
) {
    match movement {
        Move::L(v) => {
            for _ in 0..*v {
                rope[0].0 -= 1;
                for i in 0..rope.len() - 1 {
                    let positions = recompute_rope_tail_multiple(rope.clone(), i);
                    if let Some(p) = positions.last() {
                        rope[i + 1] = *p;
                    }
                    if i == rope.len() - 2 {
                        for v in positions {
                            all_positions.push(v);
                        }
                    }
                }
            }
        }
        Move::R(v) => {
            for _ in 0..*v {
                rope[0].0 += 1;
                for i in 0..rope.len() - 1 {
                    let positions = recompute_rope_tail_multiple(rope.clone(), i);
                    if let Some(p) = positions.last() {
                        rope[i + 1] = *p;
                    }
                    if i == rope.len() - 2 {
                        for v in positions {
                            all_positions.push(v);
                        }
                    }
                }
            }
        }
        Move::U(v) => {
            for _ in 0..*v {
                rope[0].1 -= 1;
                for i in 0..rope.len() - 1 {
                    let positions = recompute_rope_tail_multiple(rope.clone(), i);
                    if let Some(p) = positions.last() {
                        rope[i + 1] = *p;
                    }
                    if i == rope.len() - 2 {
                        for v in positions {
                            all_positions.push(v);
                        }
                    }
                }
            }
        }
        Move::D(v) => {
            for _ in 0..*v {
                rope[0].1 += 1;
                for i in 0..rope.len() - 1 {
                    let positions = recompute_rope_tail_multiple(rope.clone(), i);
                    if let Some(p) = positions.last() {
                        rope[i + 1] = *p;
                    }
                    if i == rope.len() - 2 {
                        for v in positions {
                            all_positions.push(v);
                        }
                    }
                }
            }
        }
    }
}

pub fn day_9_1(filename: &str) -> u32 {
    let moves = get_moves(filename);
    let mut positions: Vec<Position> = Vec::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    positions.push(tail);

    for movement in moves {
        recompute_rope(&mut head, &mut tail, &mut positions, movement);
    }

    positions.sort();
    positions.dedup();

    positions.len() as u32
}

pub fn day_9_2(filename: &str, rope_size: u32) -> u32 {
    let moves = get_moves(filename);
    let mut positions: Vec<Position> = Vec::new();
    let mut rope: Vec<Position> = Vec::new();

    for _ in 0..rope_size {
        rope.push((0, 0));
    }

    positions.push((0, 0));

    for movement in moves {
        recompute_rope_multiple(&mut rope, &mut positions, &movement);
    }

    positions.sort();
    positions.dedup();

    positions.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_9_1() {
        assert_eq!(day_9_1("src/files/day9_1.test"), 13);
    }

    #[test]
    fn test_day_9_2() {
        assert_eq!(day_9_2("src/files/day9_1.test", 2), 13);
        assert_eq!(day_9_2("src/files/day9_2.test", 10), 36);
    }
}
