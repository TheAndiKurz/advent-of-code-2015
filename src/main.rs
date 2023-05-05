use std::io::{self, Write};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod utils;

fn main() {
    // read line from console which day you want to execute
    let mut input = String::new();
    print!("Enter day: ");
    io::stdout().flush().unwrap();

    std::io::stdin().read_line(&mut input).unwrap();
    let day = input.trim().parse::<i8>().unwrap();

    match day {
        1 => {
            day1::first_part();
            day1::second_part();
        }
        2 => {
            day2::first_part();
            day2::second_part();
        }
        3 => {
            day3::first_part();
            day3::second_part();
        }
        4 => {
            day4::first_part();
            day4::second_part();
        }
        5 => {
            day5::first_part();
            day5::second_part();
        }
        _ => println!("Day not implemented yet!"),
    }
}
