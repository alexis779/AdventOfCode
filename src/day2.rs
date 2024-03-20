use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct Game {
    id: u32,
    color_sets: Vec<ColorSet>,
}

#[derive(Debug)]
struct ColorSet {
    red: u32,
    green: u32,
    blue: u32,
}

const MAX_SIZE_SET: ColorSet = ColorSet { red: 12, green: 13, blue: 14 };

pub fn id_sum(filename: String) -> Result<u32, Error>
{
    let path = Path::new(&filename);
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let sum = buf_reader.lines()
        .flatten()
        .map(parse_game)
        .filter(valid_game)
        .map(|game| game.id)
        .sum();
    Ok(sum)
}

pub fn power_sum(filename: String) -> Result<u32, Error>
{
    let path = Path::new(&filename);
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let sum = buf_reader.lines()
        .flatten()
        .map(parse_game)
        .map(game_power)
        .sum();
    Ok(sum)
}

fn parse_game(line: String) -> Game {
    let line_tokens: Vec<&str> = line.split(": ").collect();

    let game_tokens: Vec<&str> = line_tokens[0].split(" ").collect();
    let id = game_tokens[1].parse()
        .expect("Game id format is not a number");

    let color_sets = line_tokens[1].split("; ")
        .map(|color_set_token| parse_color_set(color_set_token))
        .collect();

    Game { id: id, color_sets: color_sets }
}

fn valid_game(game: &Game) -> bool {
    game.color_sets
        .iter()
        .all(|color_set| valid_color_set(&color_set))
}

fn valid_color_set(color_set: &ColorSet) -> bool {
    color_set.red <= MAX_SIZE_SET.red &&
    color_set.green <= MAX_SIZE_SET.green &&
    color_set.blue <= MAX_SIZE_SET.blue
}

fn parse_color_set(color_set_token: &str) -> ColorSet {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for color_token in color_set_token.split(", ") {
        let tokens: Vec<&str> = color_token.split(" ").collect();

        let quantity: u32 = tokens[0].parse()
            .expect("invalid quantity for color");

        match tokens[1] {
            "red" => red = quantity,
            "green" => green = quantity,
            "blue" => blue = quantity,
            _ => panic!("Invalid color {}", tokens[1]),
        }
    }

    ColorSet { red: red, green: green, blue: blue }
}

fn game_power(game: Game) -> u32 {
    let color_set = max_color_set(game);
    return color_set.red * color_set.green * color_set.blue;
}

fn max_color_set(game: Game) -> ColorSet {
    let max_red = game.color_sets.iter()
        .map(|color_set| color_set.red)
        .max()
        .unwrap_or(0);

    let max_green = game.color_sets.iter()
        .map(|color_set| color_set.green)
        .max()
        .unwrap_or(0);

    let max_blue = game.color_sets.iter()
        .map(|color_set| color_set.blue)
        .max()
        .unwrap_or(0);

    ColorSet { red: max_red, green: max_green, blue: max_blue }
}