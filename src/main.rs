
mod day1;
use day1::calibration_sum;
mod day2;
use day2::power_sum;
mod day3;
use day3::gear_ratio_sum;
mod day4;
use day4::card_count;
mod day5;
use day5::lowest_location2;
mod day6;
use day6::ways_product2;


fn main() {
    let filename = "inputs/day6_input.txt".to_string();

    let product = ways_product2(filename)
        .expect("Invalid input file");

    println!("{}", product);
}