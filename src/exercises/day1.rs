use crate::parser::{parse_aoc_file, parse_into_struct, parse_until_pattern};

pub fn day_1_1(filename: &str) -> i32 {
    let mut remaining = parse_aoc_file(filename, None);
    let mut max = 0;
    loop {
        let (matched, rest) = parse_until_pattern(remaining, "^$");

        let total: i32 = parse_into_struct(matched, |s| {
            s.parse::<i32>()
                .unwrap_or_else(|_| panic!("Unable to parse number, found {s} instead."))
        })
        .iter()
        .sum();

        if total > max {
            max = total;
        }

        if rest.is_empty() {
            break;
        } else {
            // Ignore first element of rest, it's an empty string, otherwise it will loop infinitly
            remaining = rest[1..].to_vec();
        }
    }

    max
}

pub fn day_1_2(filename: &str) -> i32 {
    let mut remaining = parse_aoc_file(filename, None);
    let mut ordered: Vec<i32> = vec![];

    loop {
        let (matched, rest) = parse_until_pattern(remaining, "^$");

        let total: i32 = parse_into_struct(matched, |s| {
            s.parse::<i32>()
                .unwrap_or_else(|_| panic!("Unable to parse number, found {s} instead."))
        })
        .iter()
        .sum();

        let mut i = 0;
        loop {
            if i == ordered.len() {
                break;
            }

            if ordered[i] > total {
                i += 1;
            } else {
                break;
            }
        }
        ordered.insert(i, total);

        if rest.is_empty() {
            break;
        } else {
            // Ignore first element of rest, it's an empty string, otherwise it will loop infinitly
            remaining = rest[1..].to_vec();
        }
    }

    ordered[0..3].iter().sum()
}

mod tests {
    #[test]
    fn test_day_1_1() {
        let res = super::day_1_1("./src/files/day1.1.test");

        assert_eq!(res, 21);
    }

    #[test]
    fn test_day_1_2() {
        let res = super::day_1_2("./src/files/day1.1.test");

        assert_eq!(res, 39);
    }
}
