use pathfinding::prelude::dijkstra;

use crate::parser::parse_aoc_file;

type Array = Vec<Vec<char>>;
type Position = (usize, usize);

fn parse_file(filename: &str) -> Array {
    parse_aoc_file(filename, None)
        .iter()
        .map(|l| l.chars().collect())
        .collect()
}

fn is_next(current: char, next: char) -> bool {
    match current {
        'S' => next == 'a',
        'a' => (next == 'a' || next == 'b') && next != 'S' && next != 'E',
        'z' => next <= 'z' || next == 'E',
        c => (next <= c || next as u32 == c as u32 + 1) && next != 'S' && next != 'E',
    }
}

fn find_start(array: &Array) -> Option<Position> {
    for (x, row) in array.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

fn find_end(array: &Array) -> Option<Position> {
    for (x, row) in array.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == 'E' {
                return Some((x, y));
            }
        }
    }
    None
}

fn neighbours((x, y): &Position, array: &Array) -> Vec<Position> {
    let mut neighbours: Vec<Position> = Vec::new();
    if *x > 0 && is_next(array[*x][*y], array[*x - 1][*y]) {
        neighbours.push((*x - 1, *y));
    }
    if *x < array.len() - 1 && is_next(array[*x][*y], array[*x + 1][*y]) {
        neighbours.push((*x + 1, *y));
    }
    if *y > 0 && is_next(array[*x][*y], array[*x][*y - 1]) {
        neighbours.push((*x, *y - 1));
    }
    if *y < array[0].len() - 1 && is_next(array[*x][*y], array[*x][*y + 1]) {
        neighbours.push((*x, *y + 1));
    }
    neighbours
}

pub fn day_12_1(filename: &str) -> u32 {
    let array = parse_file(filename);
    let start = find_start(&array).unwrap_or_else(|| panic!("Unable to find start"));
    let end = find_end(&array).unwrap_or_else(|| panic!("Unable to find end"));

    let result = dijkstra(
        &start,
        |&p| neighbours(&p, &array).into_iter().map(|p| (p, 1)),
        |&p| p == end,
    );
    result.unwrap_or_else(|| panic!("Unable to find path")).1
}

// quick and dirty
pub fn day_12_2(filename: &str) -> u32 {
    // replace S by 'a'
    let array = parse_file(filename)
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| if *c == 'S' { 'a' } else { *c })
                .collect()
        })
        .collect();
    let end = find_end(&array).unwrap_or_else(|| panic!("Unable to find end"));
    let mut min = u32::MAX;

    for i in 0..array.len() {
        for j in 0..array[0].len() {
            if array[i][j] == 'a' {
                let start = (i, j);
                let result = dijkstra(
                    &start,
                    |&p| neighbours(&p, &array).into_iter().map(|p| (p, 1)),
                    |&p| p == end,
                );
                if let Some((_, steps)) = result {
                    if steps < min {
                        min = steps;
                    }
                }
            }
        }
    }

    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_12_1() {
        assert_eq!(day_12_1("src/files/day12_1.test"), 31);
    }

    #[test]
    fn test_day_12_2() {
        assert_eq!(day_12_2("src/files/day12_1.test"), 29);
    }
}
