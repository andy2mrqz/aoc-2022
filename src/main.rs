use crate::bin::*;
mod bin;

fn main() {
    println!();
    println!("🎄 Advent of Code 🎄");
    println!();

    for day in 1..=9 {
        let day_str = format!("{:02}", day);
        println!("🎄 Day {} 🎄", day_str);
        match day {
            1 => {
                day01::main();
            }
            2 => {
                day02::main();
            }
            3 => {
                day03::main();
            }
            4 => {
                day04::main();
            }
            5 => {
                day05::main();
            }
            6 => {
                day06::main();
            }
            7 => {
                day07::main();
            }
            8 => {
                day08::main();
            }
            9 => {
                day09::main();
            }
            _ => {}
        }
        println!();
    }
}
