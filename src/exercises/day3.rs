use crate::parser::parse_aoc_file;

fn char_to_value(c: &char) -> u32 {
    if ('A'..='Z').contains(c) {
        return *c as u32 - 65 + 27;
    }
    if ('a'..='z').contains(c) {
        return *c as u32 - 97 + 1;
    }
    0
}

pub fn day_3_1(filename: &str) -> u32 {
    let rustsacks = parse_aoc_file(filename, None);
    rustsacks
        .iter()
        .map(|line| {
            let part1 = &line[0..line.len() / 2];
            let part2 = &line[line.len() / 2..line.len()];

            for c in part1.chars() {
                if part2.contains(c) {
                    return char_to_value(&c);
                }
            }

            0
        })
        .sum()
}

pub fn day_3_2(filename: &str) -> u32 {
    let rustsacks = parse_aoc_file(filename, None);
    let (chunks, remain) = rustsacks.as_chunks::<3>();
    if !remain.is_empty() {
        panic!("Unable to split all the lines into chunks of 3");
    }

    chunks
        .iter()
        .map(|chunk| {
            for c in chunk[0].chars() {
                if chunk[1].contains(c) && chunk[2].contains(c) {
                    return char_to_value(&c);
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day_3_1() {
        assert_eq!(super::day_3_1("src/files/day3_1.test"), 157);
    }

    #[test]
    fn test_day_3_2() {
        assert_eq!(super::day_3_2("src/files/day3_1.test"), 70);
    }
}
