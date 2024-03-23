mod day1;
mod day2;
mod day3;
mod day4;


#[cfg(test)]
mod tests {
    const INPUT_FOLDER: &str = "inputs/";
    const FILE_ERROR_MESSAGE: &str = "Invalid input file";

    use super::day1;
    use super::day2;
    use super::day3;
    use super::day4;

    #[test]
    fn test_day1() {
        let filename: String = string_path("day1.txt");
        let sum = day1::calibration_sum(filename)
            .expect(FILE_ERROR_MESSAGE);
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_day1_2() {
        let filename: String = string_path("day1_2.txt");
        let sum = day1::calibration_sum(filename)
            .expect(FILE_ERROR_MESSAGE);
        assert_eq!(sum, 281);
    }

    #[test]
    fn test_day2() {
        let filename: String = string_path("day2.txt");
        let sum = day2::id_sum(filename)
            .expect(FILE_ERROR_MESSAGE);
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_day2_2() {
        let filename: String = string_path("day2.txt");
        let sum = day2::power_sum(filename)
            .expect(FILE_ERROR_MESSAGE);
        assert_eq!(sum, 2286);
    }

    #[test]
    fn test_day3() {
        let filename: String = string_path("day3.txt");
        let sum = day3::number_sum(filename)
            .expect(FILE_ERROR_MESSAGE);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_day3_2() {
        let filename: String = string_path("day3.txt");
        let sum = day3::gear_ratio_sum(filename)
            .expect(FILE_ERROR_MESSAGE);
        assert_eq!(sum, 467835);
    }

    #[test]
    fn test_day4() {
        let filename: String = string_path("day4.txt");
        let sum = day4::card_point_sum(filename)
            .expect(FILE_ERROR_MESSAGE);
        assert_eq!(sum, 13);
    }

    #[test]
    fn test_day4_2() {
        let filename: String = string_path("day4.txt");
        let sum = day4::card_count(filename)
            .expect(FILE_ERROR_MESSAGE);
        assert_eq!(sum, 30);
    }


    fn string_path(filename: &str) -> String {
        return INPUT_FOLDER.to_owned() + filename;
    }
}