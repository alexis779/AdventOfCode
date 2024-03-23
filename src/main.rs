
mod day1;
use day1::calibration_sum;
mod day2;
use day2::power_sum;
mod day3;
use day3::gear_ratio_sum;
mod day4;
use day4::card_count;

fn main() {
    let filename = "inputs/day4_input.txt".to_string();

    let sum = card_count(filename)
        .expect("Invalid input file");

    println!("{}", sum);
}