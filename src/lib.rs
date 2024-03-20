mod day1;

#[cfg(test)]
mod tests {
    use super::day1;

    #[test]
    fn test_day1() {
        let filename: String = "day1.txt".to_string();
        let sum = day1::calibration_sum(filename)
            .expect("Invalid input file");
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_day1_2() {
        let filename: String = "day1_2.txt".to_string();
        let sum = day1::calibration_sum(filename)
            .expect("Invalid input file");
        assert_eq!(sum, 281);
    }
}