mod day1;
mod day2;

const INPUT_FOLDER: &str = "inputs/";

#[cfg(test)]
mod tests {
    use super::INPUT_FOLDER;
    use super::day1;
    use super::day2;

    #[test]
    fn test_day1() {
        let filename: String = string_path("day1.txt");
        let sum = day1::calibration_sum(filename)
            .expect("Invalid input file");
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_day1_2() {
        let filename: String = string_path("day1_2.txt");
        let sum = day1::calibration_sum(filename)
            .expect("Invalid input file");
        assert_eq!(sum, 281);
    }

    #[test]
    fn test_day2() {
        let filename: String = string_path("day2.txt");
        let sum = day2::id_sum(filename)
            .expect("Invalid input file");
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_day2_2() {
        let filename: String = string_path("day2.txt");
        let sum = day2::power_sum(filename)
            .expect("Invalid input file");
        assert_eq!(sum, 2286);
    }

    fn string_path(filename: &str) -> String {
        return INPUT_FOLDER.to_owned() + filename;
    }
}