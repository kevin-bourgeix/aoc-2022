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

    // Day 6.1
    let day6_1 = exercises::day6::day_6_1("./src/files/day6");
    println!("Day 6.1: {}", day6_1.first().unwrap());

    // Day 6.2
    let day6_2 = exercises::day6::day_6_2("./src/files/day6");
    println!("Day 6.2: {}", day6_2.first().unwrap());

    // Day 7.1
    let day7_1 = exercises::day7::day_7_1("./src/files/day7");
    println!("Day 7.1: {day7_1}");

    // Day 7.2
    let day7_2 = exercises::day7::day_7_2("./src/files/day7");
    println!("Day 7.2: {day7_2}");

    // Day 8.1
    let day8_1 = exercises::day8::day_8_1("./src/files/day8", 99);
    println!("Day 8.1: {day8_1}");

    // Day 8.2
    let day8_2 = exercises::day8::day_8_2("./src/files/day8", 99);
    println!("Day 8.2: {day8_2}");

    // Day 9.1
    let day9_1 = exercises::day9::day_9_1("./src/files/day9");
    println!("Day 9.1: {day9_1}");

    // Day 9.2
    let day9_2 = exercises::day9::day_9_2("./src/files/day9", 10);
    println!("Day 9.2: {day9_2}");

    // Day 10.1
    let day10_1 = exercises::day10::day_10_1("./src/files/day10");
    println!("Day 10.1: {day10_1}");

    // Day 10.2
    let day10_2 = exercises::day10::day_10_2("./src/files/day10");
    println!("Day 10.2:");
    for i in day10_2 {
        for j in i {
            if j {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!();
}
