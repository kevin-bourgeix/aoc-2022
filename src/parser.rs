use std::fs;

use regex::Regex;

// Parses an aoc file into a vector of strings
// By defaults it splits on newlines, but it can be changed easily
pub fn parse_aoc_file(filename: &str, delimiter: Option<&str>) -> Vec<String> {
    let Ok(file) = fs::read_to_string(filename) else {
        panic!("Could not read file {filename}");
    };
    let split: Vec<&str> = file.split(delimiter.unwrap_or("\n")).collect();

    return split.iter().map(|s| s.to_string()).collect();
}

// Parses strings with the given parser function
pub fn parse_into_struct<T>(data: Vec<String>, parser: fn(String) -> T) -> Vec<T> {
    let mut result = Vec::new();
    for line in data {
        result.push(parser(line));
    }
    result
}

// Reads through a vector of lines while not matching a specific regex
pub fn parse_until_pattern(data: Vec<String>, pattern: &str) -> (Vec<String>, Vec<String>) {
    let Ok(re) = Regex::new(pattern) else {
        panic!("Could not parse pattern {pattern} at parse_until_pattern");
    };
    let mut index = 0;
    for line in &data {
        if re.is_match(line) {
            break;
        }
        index += 1;
    }
    (data[0..index].to_vec(), data[index..].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[derive(Debug)]
    struct TestStruct {
        a: String,
        b: String,
    }

    #[test]
    fn test_parse_aoc_file_default() {
        let data = parse_aoc_file("./src/tests/numbers1.txt", None);
        insta::assert_debug_snapshot!(data);
    }

    #[test]
    fn test_parse_aoc_file_comma() {
        let data = parse_aoc_file("./src/tests/numbers2.txt", Some(","));
        insta::assert_debug_snapshot!(data);
    }

    #[test]
    fn test_parse_aoc_file_with_blank() {
        let data = parse_aoc_file("./src/tests/numbers3.txt", None);
        insta::assert_debug_snapshot!(data);
    }

    #[test]
    fn test_parse_into_struct() {
        let data = vec!["1,2".to_string(), "3,4".to_string()];
        let result = parse_into_struct(data, |s| {
            let split: Vec<&str> = s.split(',').collect();
            TestStruct {
                a: split[0].to_string(),
                b: split[1].to_string(),
            }
        });

        insta::assert_debug_snapshot!(result);
    }

    #[test]
    fn test_parse_into_simple() {
        let data = vec!["1".to_string(), "2".to_string()];
        let result = parse_into_struct(data, |s| s.parse::<i32>().unwrap());

        insta::assert_debug_snapshot!(result);
    }

    #[test]
    fn test_parse_until() {
        let data = vec!["aaaa", "aaaa", "aaaa", "bbbb", "bbbb", "cccc"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let (first, second) = parse_until_pattern(data, "bbbb");

        insta::assert_debug_snapshot!(vec![first, second]);
    }

    #[test]
    fn test_parse_empty() {
        let data = vec!["1", "", "2", "3"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let (first, second) = parse_until_pattern(data, r"^$");

        insta::assert_debug_snapshot!(vec![first, second]);
    }
}
