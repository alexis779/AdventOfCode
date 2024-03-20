
mod day1;
use day1::calibration_sum;
mod day2;
use day2::power_sum;

fn main() {
    let filename = "inputs/day2_input.txt".to_string();

    let sum = power_sum(filename)
        .expect("Invalid input file");

    println!("{}", sum);
}