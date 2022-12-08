use crate::parser::parse_aoc_file;

fn parse_array(filename: &str) -> Vec<Vec<u8>> {
    let lines = parse_aoc_file(filename, None);
    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| {
                    c.to_string()
                        .parse::<u8>()
                        .unwrap_or_else(|_| panic!("Unable to parse char {c}"))
                })
                .collect()
        })
        .collect()
}

fn check_left(trees: &[Vec<u8>], i: &u32, j: &u32) -> bool {
    trees[*i as usize][0..(*j as usize)]
        .iter()
        .all(|x| *x < trees[*i as usize][*j as usize])
}

fn check_right(trees: &[Vec<u8>], i: &u32, j: &u32, size: &u32) -> bool {
    trees[*i as usize][((*j as usize) + 1)..*size as usize]
        .iter()
        .all(|x| *x < trees[*i as usize][*j as usize])
}

fn check_top(trees: &[Vec<u8>], i: &u32, j: &u32) -> bool {
    trees.iter().map(|x| x[*j as usize]).collect::<Vec<u8>>()[0..(*i as usize)]
        .to_vec()
        .iter()
        .all(|x| *x < trees[*i as usize][*j as usize])
}

fn check_bottom(trees: &[Vec<u8>], i: &u32, j: &u32, size: &u32) -> bool {
    trees.iter().map(|x| x[*j as usize]).collect::<Vec<u8>>()[((*i as usize) + 1)..*size as usize]
        .to_vec()
        .iter()
        .all(|x| *x < trees[*i as usize][*j as usize])
}

fn get_line_max_visible_tree(line: &mut [u8], value: u8) -> u32 {
    let mut count = 0;
    let mut remaining = Vec::from(line);
    remaining.reverse();
    loop {
        if remaining.is_empty() {
            break;
        }

        count += 1;
        let curr = remaining.pop().unwrap();
        if curr >= value {
            break;
        }
    }
    count
}

fn max_tree_left(trees: &[Vec<u8>], i: &u32, j: &u32) -> u32 {
    let mut line = trees[*i as usize][0..(*j as usize)].to_vec();
    // reverse the line so the first index is the closest from the inner tree
    line.reverse();
    get_line_max_visible_tree(&mut line, trees[*i as usize][*j as usize])
}

fn max_tree_right(trees: &[Vec<u8>], i: &u32, j: &u32, size: &u32) -> u32 {
    let mut line = trees[*i as usize][((*j as usize) + 1)..*size as usize].to_vec();
    get_line_max_visible_tree(&mut line, trees[*i as usize][*j as usize])
}

fn max_tree_top(trees: &[Vec<u8>], i: &u32, j: &u32) -> u32 {
    let mut line =
        trees.iter().map(|x| x[*j as usize]).collect::<Vec<u8>>()[0..(*i as usize)].to_vec();
    // reverse the line so the first index is the closest from the inner tree
    line.reverse();
    get_line_max_visible_tree(&mut line, trees[*i as usize][*j as usize])
}

fn max_tree_bottom(trees: &[Vec<u8>], i: &u32, j: &u32, size: &u32) -> u32 {
    let mut line = trees.iter().map(|x| x[*j as usize]).collect::<Vec<u8>>()
        [((*i as usize) + 1)..*size as usize]
        .to_vec();
    get_line_max_visible_tree(&mut line, trees[*i as usize][*j as usize])
}

// Size is predefined before parsing for convenience
pub fn day_8_1(filename: &str, size: u32) -> u32 {
    let mut count = 0;
    let trees = parse_array(filename);

    for i in 0..size {
        for j in 0..size {
            if i == 0 || i == size - 1 || j == 0 || j == size - 1 {
                count += 1;
                continue;
            }

            if check_left(&trees, &i, &j)
                || check_right(&trees, &i, &j, &size)
                || check_top(&trees, &i, &j)
                || check_bottom(&trees, &i, &j, &size)
            {
                count += 1;
            }
        }
    }
    count
}

pub fn day_8_2(filename: &str, size: u32) -> u32 {
    let trees = parse_array(filename);
    trees
        .iter()
        .enumerate()
        .map(|(i, v_i)| {
            v_i.iter()
                .enumerate()
                .map(|(j, _)| {
                    let k = i as u32;
                    let l = j as u32;
                    max_tree_left(&trees, &k, &l)
                        * max_tree_right(&trees, &k, &l, &size)
                        * max_tree_top(&trees, &k, &l)
                        * max_tree_bottom(&trees, &k, &l, &size)
                })
                .max()
                .unwrap_or(1)
        })
        .max()
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_8_1() {
        assert_eq!(day_8_1("src/files/day8_1.test", 5), 21);
    }

    #[test]
    fn test_day_8_2() {
        assert_eq!(day_8_2("src/files/day8_1.test", 5), 8);
    }
}
