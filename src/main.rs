#![feature(slice_as_chunks)]

mod exercises;
mod parser;

fn main() {
    // Day 1.1
    let day1_1 = exercises::day1::day_1_1("./src/files/day1.1");
    println!("Day 1.1 : {day1_1}");

    // Day 1.2
    let day1_2 = exercises::day1::day_1_2("./src/files/day1.1");
    println!("Day 1.2 : {day1_2}");

    // Day 2.1
    let day2_1 = exercises::day2::day_2_1("./src/files/day2");
    println!("Day 2.1 : {day2_1}");

    // Day 2.2
    let day2_2 = exercises::day2::day_2_2("./src/files/day2");
    println!("Day 2.2 : {day2_2}");

    // Day 3.1
    let day3_1 = exercises::day3::day_3_1("./src/files/day3");
    println!("Day 3.1: {day3_1}");

    // Day 3.2
    let day3_2 = exercises::day3::day_3_2("./src/files/day3");
    println!("Day 3.2: {day3_2}");

    // Day 4.1
    let day4_1 = exercises::day4::day_4_1("./src/files/day4");
    println!("Day 4.1: {day4_1}");

    // Day 4.2
    let day4_2 = exercises::day4::day_4_2("./src/files/day4");
    println!("Day 4.2: {day4_2}");

    // Day 5.1
    let day5_1 = exercises::day5::day_5_1("./src/files/day5");
    print!("Day 5.1: ");
    for c in day5_1 {
        print!("{c}");
    }
    println!();

    // Day 5.2
    let day5_2 = exercises::day5::day_5_2("./src/files/day5");
    print!("Day 5.2: ");
    for c in day5_2 {
        print!("{c}");
    }
    println!();
}
