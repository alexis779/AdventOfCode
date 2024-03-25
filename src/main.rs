
mod day1;
use day1::calibration_sum;
mod day2;
use day2::power_sum;
mod day3;
use day3::gear_ratio_sum;
mod day4;
use day4::card_count;
mod day5;
use day5::lowest_location;

fn main() {
    let filename = "inputs/day5_input.txt".to_string();

    let min = lowest_location(filename)
        .expect("Invalid input file");

    println!("{}", min);
}