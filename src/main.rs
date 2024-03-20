
mod day1;
use day1::calibration_sum;

fn main() {
    let filename = "day1_input.txt".to_string();

    let sum = calibration_sum(filename)
        .expect("Invalid input file");

    println!("{}", sum);
}