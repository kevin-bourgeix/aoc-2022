use crate::parser::{parse_aoc_file, parse_into_struct};

enum Shifumi {
  Rock,
  Paper,
  Scissor,
}

// Rock = 1, Paper = 2, Scissor = 3 
// Lose = 0, Draw = 3, Win = 6
fn resolve_shifumi(you: Shifumi, opp: Shifumi) -> i32 {
 match (you, opp) {
    (Shifumi::Rock, Shifumi::Rock) => 4,
    (Shifumi::Rock, Shifumi::Paper) => 1,
    (Shifumi::Rock, Shifumi::Scissor) => 7,
    (Shifumi::Paper, Shifumi::Rock) => 8,
    (Shifumi::Paper, Shifumi::Paper) => 5,
    (Shifumi::Paper, Shifumi::Scissor) => 2,
    (Shifumi::Scissor, Shifumi::Rock) => 3,
    (Shifumi::Scissor, Shifumi::Paper) => 9,
    (Shifumi::Scissor, Shifumi::Scissor) => 6,
  }
}

fn convert_to_shifumi(letter: char) -> Shifumi {
  match letter {
    'A'|'X' => Shifumi::Rock,
    'B'|'Y' => Shifumi::Paper,
    'C'|'Z' => Shifumi::Scissor,
    _ => panic!("Invalid letter {letter}"),
  }
}

// X = should lose, Y = should draw, Z = should win
fn get_shifumi_strategy(opp: &Shifumi, letter: char) -> Shifumi {
  match (opp, letter) {
    (Shifumi::Rock, 'X') => Shifumi::Scissor,
    (Shifumi::Rock, 'Y') => Shifumi::Rock,
    (Shifumi::Rock, 'Z') => Shifumi::Paper,
    (Shifumi::Paper, 'X') => Shifumi::Rock,
    (Shifumi::Paper, 'Y') => Shifumi::Paper,
    (Shifumi::Paper, 'Z') => Shifumi::Scissor,
    (Shifumi::Scissor, 'X') => Shifumi::Paper,
    (Shifumi::Scissor, 'Y') => Shifumi::Scissor,
    (Shifumi::Scissor, 'Z') => Shifumi::Rock,
    _ => panic!("Invalid letter {letter}"),
  }
}

pub fn day_2_1(filename: &str) -> i32 {
  let games = parse_aoc_file(filename, None);
  let scores = parse_into_struct(games, |l| {
    let opp = l.chars().next().unwrap_or_else(|| panic!("Unable to get 1st char at {l}"));
    let you = l.chars().nth(2).unwrap_or_else(|| panic!("Unable to get 2nd char at {l}"));

    let opp_shifumi = convert_to_shifumi(opp);
    let you_shifumi = convert_to_shifumi(you);

    resolve_shifumi(you_shifumi, opp_shifumi)
  });

  scores.iter().sum()
}

pub fn day_2_2(filename: &str) -> i32 {
  let games = parse_aoc_file(filename, None);
  let scores = parse_into_struct(games, |l| {
    let opp = l.chars().next().unwrap_or_else(|| panic!("Unable to get 1st char at {l}"));
    let you = l.chars().nth(2).unwrap_or_else(|| panic!("Unable to get 2nd char at {l}"));

    let opp_shifumi = convert_to_shifumi(opp);
    let you_shifumi = get_shifumi_strategy(&opp_shifumi, you);

    resolve_shifumi(you_shifumi, opp_shifumi)
  });

  scores.iter().sum()
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_day_2_1() {
    let result = day_2_1("./src/files/day2_1.test");
    assert_eq!(result, 15);
  }

  #[test]
  fn test_day_2_2() {
    let result = day_2_2("./src/files/day2_2.test");
    assert_eq!(result, 12);
  }
}