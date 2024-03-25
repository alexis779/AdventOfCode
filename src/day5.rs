use std::collections::VecDeque;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    transforms: Vec<Transform>,
}

#[derive(Debug)]
struct Transform {
    maps: Vec<CategoryMap>,
}

#[derive(Debug)]
struct CategoryMap {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

#[derive(Debug)]
struct Interval {
    start: u64,
    length: u64,
}

pub fn lowest_location(filename: String) -> Result<u64, Error>
{
    let almanac = parse_almanac(filename)?;

    let min = almanac.seeds
        .iter()
        .map(|seed| location(seed, &almanac.transforms))
        .min()
        .expect("List of seeds is empty");

    Ok(min)
}

pub fn lowest_location2(filename: String) -> Result<u64, Error>
{
    let almanac = parse_almanac(filename)?;
    let mut seed_intervals: Vec<Interval> = (0..almanac.seeds.len()/2).map(|i| Interval {
            start: almanac.seeds[2*i],
            length: almanac.seeds[2*i+1],
        }).collect();

    sort_intervals(&mut seed_intervals);

    let location_intervals: Vec<Interval> = almanac.transforms.iter()
        .fold(seed_intervals, transpose);

    let min = location_intervals[0].start;
    Ok(min)
}

fn parse_almanac(filename: String) -> Result<Almanac, Error>
{
    let path = Path::new(&filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let seeds = parse_seeds(&mut lines);

    let mut transforms: Vec<Transform> = Vec::new();

    loop {
        let transform = parse_transform(&mut lines);
        if transform.is_none() {
            break;
        }

        transforms.push(transform.unwrap());
    }

    let almanac = Almanac { seeds, transforms };
    Ok(almanac)
}

fn parse_seeds(mut lines: impl Iterator<Item=Result<String, Error>>) -> Vec<u64>
{
    let line = lines.next().unwrap().unwrap();
    lines.next();

    let tokens: Vec<&str> = line.split(": ")
        .collect();

    parse_sequence(tokens[1])
}

fn parse_sequence(token: &str) -> Vec<u64>
{
    token.split(" ")
        .map(parse_token)
        .collect()
}

fn parse_transform(mut lines: impl Iterator<Item=Result<String, Error>>) -> Option<Transform>
{
    // parse the 1st line containing the map name
    let mut line = lines.next();
    if line.is_none() {
        return None;
    }

    let mut maps = Vec::new();

    loop {
        line = lines.next();
        if line.is_none() {
            break;
        }

        let line_string = line.unwrap().unwrap();
        if line_string.is_empty() {
            break;
        }

        let category_map = parse_category_map(&line_string);
        maps.push(category_map);
    }

    // sort vector by source_start in place
    maps.sort_by_key(|category_map| category_map.source_start);

    let transform = Transform { maps };
    Some(transform)
}

fn parse_category_map(line: &str) -> CategoryMap {
    // parse the 3 tokens in the line
    let tokens: Vec<&str> = line.split(" ")
        .collect();
    let destination_start = parse_token(tokens[0]);
    let source_start = parse_token(tokens[1]);
    let range_length = parse_token(tokens[2]);

    CategoryMap {
        destination_start,
        source_start,
        range_length,
    }
}

fn parse_token(token: &str) -> u64 {
    token.parse()
        .expect("Invalid number format")
}

fn location(seed: &u64, transforms: &Vec<Transform>) -> u64 {
    transforms.iter()
        .fold(*seed, apply_transform)
}

fn apply_transform(value: u64, transform: &Transform) -> u64 {
    transform.maps.iter()
        .find(|category_map| within_range(value, category_map))
        .map(|category_map| apply_map(value, category_map))
        .or(Some(value))
        .unwrap()
}

fn transpose(interval_vec: Vec<Interval>, transform: &Transform) -> Vec<Interval> {
    let mut transposed : Vec<Interval> = Vec::new();

    let interval_id = 0;
    let mut category_map_id = 0;

    let category_maps = &transform.maps;

    let mut intervals = VecDeque::from(interval_vec);

    while interval_id < intervals.len() && category_map_id < category_maps.len() {

        let interval_start = intervals[interval_id].start;
        let interval_length = intervals[interval_id].length;
        let interval_end = interval_start + interval_length;

        let category_start = category_maps[category_map_id].source_start;
        let category_end = category_start + category_maps[category_map_id].range_length;

        // assign the min between interval_end and category_end to left_end
        let left_end = interval_end.min(category_end);

        if interval_start < category_start {
            let left_length = left_end - interval_start;

            transposed.push(Interval {
                start: interval_start,
                length: left_length,
            });

            intervals.pop_front();

            if interval_end <= category_start {
                continue;
            }

            let right_length = interval_end - left_end;
            if right_length == 0 {
                continue;
            }

            intervals.push_front(Interval {
                start: left_end,
                length: right_length,
            });

            continue;
        }

        if interval_start < category_end {
            // assign the max between interval_start and category_start to left_start
            let middle_start = interval_start.max(category_start);
            let middle_length = left_end - middle_start;

            let transpose_start = category_maps[category_map_id].destination_start + interval_start - category_start;

            transposed.push(Interval {
                start: transpose_start,
                length: middle_length,
            });

            intervals.pop_front();

            if interval_end <= category_end {
                continue;
            }

            let right_length = interval_end - category_end;

            intervals.push_front(Interval {
                start: category_end,
                length: right_length,
            });

            continue;
        }

        category_map_id += 1;
    }

    while ! intervals.is_empty() {
        let interval_start = intervals[interval_id].start;
        let interval_length = intervals[interval_id].length;
        transposed.push(Interval {
            start: interval_start,
            length: interval_length,
        });
        intervals.pop_front();
    }

    sort_intervals(&mut transposed);

    transposed
}

fn sort_intervals(intervals: &mut Vec<Interval>) -> () {
    intervals.sort_by_key(|interval: &Interval| interval.start);    
}

fn within_range(value: u64, category_map: &CategoryMap) -> bool {
    category_map.source_start <= value && value < category_map.source_start + category_map.range_length
}

fn apply_map(value: u64, category_map: &CategoryMap) -> u64 {
    category_map.destination_start + value - category_map.source_start
}