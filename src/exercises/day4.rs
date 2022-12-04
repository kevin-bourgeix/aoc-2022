use crate::parser::{parse_aoc_file, parse_into_struct};

#[derive(Debug, Clone, Copy)]
struct Interval {
    min: u32,
    max: u32,
}

impl Interval {
    fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }

    fn contains(&self, interval: &Interval) -> bool {
        self.min <= interval.min && self.max >= interval.max
    }

    fn contains_value(&self, value: u32) -> bool {
        self.min <= value && self.max >= value
    }

    fn overlaps(&self, interval: &Interval) -> bool {
        interval.contains_value(self.min) || interval.contains_value(self.max)
    }
}

fn read_intervals(line: String) -> (Interval, Interval) {
    let mut parts = line.split(',');
    let part1 = parts
        .next()
        .unwrap_or_else(|| panic!("Unable to get part1 from {line}"));
    let part2 = parts
        .next()
        .unwrap_or_else(|| panic!("Unable to get part2 from {line}"));

    let mut int1 = part1.split('-');
    let interval1 = Interval::new(
        int1.next()
            .unwrap_or_else(|| panic!("Unable to get min from {part1}"))
            .parse()
            .unwrap_or_else(|_| panic!("Unable to parse min from {part1} (not a number)")),
        int1.next()
            .unwrap_or_else(|| panic!("Unable to get max from {part1}"))
            .parse()
            .unwrap_or_else(|_| panic!("Unable to parse max from {part1} (not a number)")),
    );
    let mut int2 = part2.split('-');
    let interval2 = Interval::new(
        int2.next()
            .unwrap_or_else(|| panic!("Unable to get min from {part2}"))
            .parse()
            .unwrap_or_else(|_| panic!("Unable to parse min from {part2} (not a number)")),
        int2.next()
            .unwrap_or_else(|| panic!("Unable to get max from {part2}"))
            .parse()
            .unwrap_or_else(|_| panic!("Unable to parse max from {part2} (not a number)")),
    );

    (interval1, interval2)
}

pub fn day_4_1(filename: &str) -> u32 {
    let lines = parse_aoc_file(filename, None);
    let intervals: Vec<(Interval, Interval)> = parse_into_struct(lines, read_intervals);
    intervals.iter().fold(0, |acc, cur| {
        if cur.0.contains(&cur.1) || cur.1.contains(&cur.0) {
            acc + 1
        } else {
            acc
        }
    })
}

pub fn day_4_2(filename: &str) -> u32 {
    let lines = parse_aoc_file(filename, None);
    let intervals: Vec<(Interval, Interval)> = parse_into_struct(lines, read_intervals);
    intervals.iter().fold(0, |acc, cur| {
        if cur.0.overlaps(&cur.1) || cur.1.overlaps(&cur.0) {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day_4_1() {
        assert_eq!(super::day_4_1("src/files/day4_1.test"), 2);
    }

    #[test]
    fn test_day_4_2() {
        assert_eq!(super::day_4_2("src/files/day4_1.test"), 4);
    }
}
