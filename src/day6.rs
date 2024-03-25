use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Error};

const BASE: u64 = 10;

#[derive(Debug)]
struct Race {
    time: u64,
    duration: u64
}

pub fn ways_product(filename: String) -> Result<u64, Error>
{
    let races = parse_races(filename)?;

    let product = races.iter()
        .map(ways)
        .product();

    Ok(product)
}

pub fn ways_product2(filename: String) -> Result<u64, Error>
{
    let race = parse_race(filename)?;
    let ways = ways(&race);
    Ok(ways)
}

fn parse_races(filename: String) -> Result<Vec<Race>, Error>
{
    let path = Path::new(&filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let times: Vec<u64> = parse_values(&mut lines);
    let durations: Vec<u64> = parse_values(&mut lines);

    let races = (0..times.len()).map(|i| Race {
            time: times[i],
            duration: durations[i]
        })
        .collect();

    Ok(races)
}

fn parse_race(filename: String) -> Result<Race, Error>
{
    let path = Path::new(&filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let time = parse_value(&mut lines);
    let duration = parse_value(&mut lines);

    let race =  Race { time, duration } ;

    Ok(race)
}

fn parse_values(mut lines: impl Iterator<Item=Result<String, Error>>) -> Vec<u64> {
    let line = lines.next()
        .unwrap()
        .unwrap();

    let tokens: Vec<&str> = line.split(":")
        .collect();

    tokens[1].split_whitespace()
        .map(|token| token.parse())
        .map(|token| token.expect("Invalid number format"))
        .collect()
}

fn parse_value(mut lines: impl Iterator<Item=Result<String, Error>>) -> u64 {
    let line = lines.next()
        .unwrap()
        .unwrap();

    let tokens: Vec<&str> = line.split(":")
        .collect();

    let mut pow: u64 = 1;

    tokens[1].split_whitespace()
        .flat_map(|token| token.chars())
        .map(|c| c.to_digit(BASE as u32))
        .map(|c| c.expect("Invalid digit character") as u64)
        .rev()
        .fold(0, |acc, digit| {
                let sum = acc + digit * pow;
                pow *= BASE;
                sum
        })
}

fn ways(race: &Race) -> u64 {
    let t = race.time;
    let d = race.duration;
    let delta = t * t - 4 * d;

    if delta <= 0 {
        return 0;
    }

    let delta_sqrt = (delta as f64).sqrt();

    let x1 = (t as f64 - delta_sqrt) / 2.0;
    let x2 = (t as f64 + delta_sqrt) / 2.0;

    let mut upper = x1.ceil() as u64;
    let mut lower = x2.floor() as u64;

    let sqrt_int = delta_sqrt as u64;
    let is_square = delta == sqrt_int * sqrt_int;
    let is_even = (t + sqrt_int) % 2 == 0;
    let is_int = is_square && is_even;

    if is_int {
        upper += 1;
        lower -= 1;
    }

    if !(upper <= lower) {
        return 0;
    }

    lower - upper + 1
}