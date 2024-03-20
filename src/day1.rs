use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Error};

struct Digit<'a> {
    i: usize,
    c: char,
    word: &'a [char]
}

const DIGITS: [Digit; 10] = [
    Digit { i: 0, c: '0', word: &['z', 'e', 'r', 'o'] },
    Digit { i: 1, c: '1', word: &['o', 'n', 'e'] },
    Digit { i: 2, c: '2', word: &['t', 'w', 'o'] },
    Digit { i: 3, c: '3', word: &['t', 'h', 'r', 'e', 'e'] },
    Digit { i: 4, c: '4', word: &['f', 'o', 'u', 'r'] },
    Digit { i: 5, c: '5', word: &['f', 'i', 'v', 'e'] },
    Digit { i: 6, c: '6', word: &['s', 'i', 'x'] },
    Digit { i: 7, c: '7', word: &['s', 'e', 'v', 'e', 'n'] },
    Digit { i: 8, c: '8', word: &['e', 'i', 'g', 'h', 't'] },
    Digit { i: 9, c: '9', word: &['n', 'i', 'n', 'e'] },
];

pub fn calibration_sum(filename: String) -> Result<u32, Error>
{
    let path = Path::new(&filename);
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let sum = buf_reader.lines()
        .flatten()
        .map(calibration_value)
        .sum();
    Ok(sum)
}

fn calibration_value(line: String) -> u32 {
    let chars = line.chars().collect();
    let first = first_digit(&chars, 0..chars.len()) as u32;
    let last = first_digit(&chars, (0..chars.len()).rev()) as u32;
    10 * first + last
}

fn first_digit<I>(chars: &Vec<char>, range: I) -> usize
where
    I: Iterator<Item=usize> {
    range
        .filter_map(|i| digit(chars, i))
        .next()
        .expect("No digit find in line")
}

fn digit(chars: &[char], i: usize) -> Option<usize> {
    DIGITS.iter()
        .find(|digit| digit.c == chars[i] || start_with(chars, i, digit.word))
        .map(|digit| digit.i)
}


fn start_with(chars: &[char], i: usize, word: &[char]) -> bool {
    if !(i + word.len() <= chars.len()) {
        return false;
    }

    chars[i..(i+word.len())].eq(word)
}