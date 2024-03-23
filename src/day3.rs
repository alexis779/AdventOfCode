use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Error};

struct Engine {
    grid: Vec<Vec<Cell>>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
struct Cell {
    value: char,
    row: usize,
    col: usize,
}

#[derive(Debug)]
#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Hash)]
struct Number {
    value: u32,
    row: usize,
    start_col: usize,
    end_col: usize,
}

pub fn number_sum(filename: String) -> Result<u32, Error>
{
    let engine = engine(filename)?;

    let numbers = numbers(&engine);

    let adjacent_cells = adjacent_cells_map(&numbers, &engine);

    let sum = numbers
        .iter()
        .filter(|number| !adjacent_cells.get(*number)
                                    .unwrap()
                                    .iter()
                                    .all(|cell| cell.value == '.'))
        .map(|number| number.value)
        .sum();

    Ok(sum)
}

pub fn gear_ratio_sum(filename: String) -> Result<u32, Error>
{
    let engine = engine(filename)?;

    let numbers = numbers(&engine);

    let adjacent_cells = adjacent_cells_map(&numbers, &engine);

    let adjacent_numbers = adjacent_numbers_map(&adjacent_cells);

    let sum = engine.grid
        .iter()
        .flatten()
        .filter(|cell| cell.value == '*')
        .map(|cell| adjacent_numbers.get(cell)
                            .unwrap())
        .filter(|numbers| numbers.len() == 2)
        .map(gear_ratio)
        .sum();

    Ok(sum)
}

fn engine(filename: String) -> Result<Engine, Error> {
    let path = Path::new(&filename);
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    let mut rows = Vec::new();

    let lines: Vec<String> = buf_reader.lines()
        .flatten()
        .collect();

    for (row_index, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (col_index, c) in line.chars().enumerate() {
           let cell = Cell { value: c, row: row_index, col: col_index };
           row.push(cell);
        }
        rows.push(row);
    }

    let row_size = rows.len();

    let col_size = rows[0].len();

    let engine = Engine { grid: rows, rows: row_size, cols: col_size };

    Ok(engine)
}

fn numbers(engine: &Engine) -> Vec<Number>
{
    let mut numbers = Vec::new();
    for (row_index, row) in engine.grid.iter().enumerate() {
        let mut col_index = 0;
        
        loop {
            while col_index < row.len() && !row[col_index].value.is_ascii_digit() {
                col_index += 1;
            }
            if col_index == row.len() {
                break;
            }
            let start_col = col_index;
            while col_index < row.len() && row[col_index].value.is_ascii_digit() {
                col_index += 1;
            }
            let end_col = col_index - 1;

            // parse the chars from start_col to end_col as an int
            // storing the value in value variable
            let value = row[start_col..=end_col].iter()
                .map(|cell| cell.value)
                .collect::<String>()
                .parse::<u32>()
                .expect("invalid number format");

            let number = Number {
                value: value,
                row: row_index,
                start_col,
                end_col,
            };

            numbers.push(number);
        }
    }
    numbers
}

fn adjacent_cells_map<'a>(numbers: &'a Vec<Number>, engine: &'a Engine) -> HashMap<&'a Number, Vec<&'a Cell>>
{
    let mut adjacent_cells = HashMap::new();

    for number in numbers {
        adjacent_cells.insert(number, adjacent_cells_vec(number, engine));
    }

    adjacent_cells
}

fn adjacent_cells_vec<'a>(number: &'a Number, engine: &'a Engine) -> Vec<&'a Cell> {
    let mut adjacent_cells = Vec::new();

    let row = number.row;
    let start_col = number.start_col;
    let end_col = number.end_col;

    [row as isize - 1, row as isize + 1].iter()
        .filter(|symbol_row| **symbol_row >= 0 && **symbol_row < engine.rows as isize)
        .for_each(|symbol_row| {
            (start_col as isize - 1 ..= end_col as isize + 1)
                    .filter(|col| *col >= 0 && *col < engine.cols as isize)
                    .map(|col| engine.grid[*symbol_row as usize].get(col as usize))
                    .map(|cell| cell.unwrap())
                    .filter(symbol)
                    .for_each(|cell| adjacent_cells.push(cell));
            }
        );
    
    [start_col as isize - 1, end_col as isize + 1].iter()
        .filter(|symbol_col| **symbol_col >= 0 && **symbol_col < engine.cols as isize)
        .map(|symbol_col| engine.grid[row].get(*symbol_col as usize))
        .map(|cell| cell.unwrap())
        .filter(symbol)
        .for_each(|cell| adjacent_cells.push(cell));

    adjacent_cells
}

fn adjacent_numbers_map<'a>(adjacent_cells_map: &'a HashMap<&Number, Vec<&Cell>>) -> HashMap<&'a Cell, Vec<&'a Number>>
{
    let mut adjacent_numbers = HashMap::new();

    for (number, cells) in adjacent_cells_map {
        for cell in cells { 
            adjacent_numbers.entry(*cell)
                .or_insert(Vec::new())
                .push(*number);
        }
    }

    adjacent_numbers
}

fn symbol(cell: &&Cell) -> bool {
    let c = cell.value;
    !(c == '.' || c.is_ascii_digit())
}

fn gear_ratio(numbers: &Vec<&Number>) -> u32 {
    numbers[0].value * numbers[1].value
}