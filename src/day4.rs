use std::collections::HashSet;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Error};

struct Card {
    winning: HashSet<u32>,
    hand: Vec<u32>,
}

pub fn card_point_sum(filename: String) -> Result<u32, Error>
{
    let sum = parse_cards(filename)?
        .iter()
        .map(points)
        .sum();

    Ok(sum)
}

pub fn card_count(filename: String) -> Result<u32, Error>
{
    let cards = parse_cards(filename)?;

    // create an array of size cards.len()
    let mut counts = vec![1; cards.len()];

    for (i, card) in cards.iter().enumerate() {
        let winning = winning_cards(card);
        for j in i+1..=i+winning {
            counts[j] += counts[i];
        }
    }

    // return the sum of elements in counts array
    let sum = counts.iter()
        .sum();

    Ok(sum)
}

fn parse_cards(filename: String) -> Result<Vec<Card>, Error> {
    let path = Path::new(&filename);
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    let cards = buf_reader.lines()
        .flatten()
        .map(parse_card)
        .collect();

    Ok(cards)
}

fn parse_card(line: String) -> Card {
    let tokens: Vec<&str> = line.split(": ").collect();
    let tokens: Vec<&str> = tokens[1].split(" | ").collect();

    let winning: HashSet<u32> = parse_sequence(tokens[0]).collect();
    let hand: Vec<u32> = parse_sequence(tokens[1]).collect();

    Card { winning, hand }
}

fn parse_sequence(string: &str) -> impl Iterator<Item=u32> + '_ {
    string.split_whitespace()
        .map(|x| x.parse())
        .map(|x| x.expect("Invalid number format"))
}

fn points(card: &Card) -> u32 {
    let winning = winning_cards(card);
    if winning == 0 {
        return 0;
    }

    return 2u32.pow(winning as u32 - 1);
}

fn winning_cards(card: &Card) -> usize {
    card.hand.iter()
        .filter(|x| card.winning.contains(x))
        .count()
}