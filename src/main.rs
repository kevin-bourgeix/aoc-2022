mod exercises;
mod parser;

fn main() {
    // Day 1.1
    let day1_1 = exercises::day1::day_1_1("./src/files/day1.1");
    println!("Day 1.1 : {day1_1}");

    // Day 1.2
    let day1_2 = exercises::day1::day_1_2("./src/files/day1.1");
    println!("Day 1.2 : {day1_2}");
}
