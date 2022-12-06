use crate::parser::parse_aoc_file;

// There should be something easier
fn has_multiple_char_occ(input: &str) -> bool {
    let mut chars = input.chars();
    loop {
        let Some(c) = chars.next() else {
            break;
        };

        if input.matches(c).count() != 1 {
            return true;
        }
    }
    false
}

fn make_substrings(input: &String, size: Option<usize>) -> Vec<String> {
    let n = size.unwrap_or(4);
    let mut result: Vec<String> = Vec::new();

    if input.len() <= n {
        return vec![input.to_string()];
    }

    let mut i = n;
    while i <= input.len() {
        result.push(input[i - n..i].to_string());
        i += 1;
    }

    result
}

pub fn day_6_1(filename: &str) -> Vec<u32> {
    let file = parse_aoc_file(filename, None);
    file.iter()
        .map(|line| {
            let substrings = make_substrings(line, None);
            let mut i: u32 = 0;
            loop {
                let Some(substring) = substrings.get(i as usize) else {
                break;
            };
                if !has_multiple_char_occ(substring) {
                    break;
                }
                i += 1;
            }
            i + 4
        })
        .collect()
}

pub fn day_6_2(filename: &str) -> Vec<u32> {
    let file = parse_aoc_file(filename, None);
    file.iter()
        .map(|line| {
            let substrings = make_substrings(line, Some(14));
            let mut i: u32 = 0;
            loop {
                let Some(substring) = substrings.get(i as usize) else {
                break;
            };
                if !has_multiple_char_occ(substring) {
                    i += 0;
                    break;
                }
                i += 1;
            }
            i + 14
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::exercises::day6::{day_6_2, has_multiple_char_occ};

    use super::day_6_1;

    #[test]
    fn test_make_substrings() {
        let input = "abcdef".to_string();
        let result = super::make_substrings(&input, None);
        assert_eq!(result, vec!["abcd", "bcde", "cdef"]);
    }

    #[test]
    fn test_multiple_char_occ() {
        let input = ["abcd".to_string(), "abca".to_string()];
        let result: Vec<bool> = input.iter().map(|s| has_multiple_char_occ(s)).collect();
        assert_eq!(result, vec![false, true]);
    }

    #[test]
    fn test_day_6_1() {
        let result: Vec<u32> = day_6_1("src/files/day6_1.test");
        assert_eq!(result, vec![5, 6, 10, 12]);
    }

    #[test]
    fn test_day_6_2() {
        let result: Vec<u32> = day_6_2("src/files/day6_2.test");
        assert_eq!(result, vec![19, 23, 23, 29, 27]);
    }
}
