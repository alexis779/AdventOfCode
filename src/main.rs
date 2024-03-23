
mod day1;
use day1::calibration_sum;
mod day2;
use day2::power_sum;
mod day3;
use day3::gear_ratio_sum;

fn main() {
    let filename = "inputs/day3_input.txt".to_string();

    let sum = gear_ratio_sum(filename)
        .expect("Invalid input file");

    println!("{}", sum);
}