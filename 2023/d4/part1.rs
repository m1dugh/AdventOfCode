use std::cmp::PartialEq;
use std::io::{self, BufRead, BufReader, Read};
use std::fs;
use std::env;
use std::fmt;

#[derive(Debug)]
struct Card
{
    id: u32,
    winings: Vec<u32>,
    values: Vec<u32>,
}

impl Card
{
    fn get_results(&self) -> u32
    {
        let found = self.values
            .iter()
            .filter(|v| self.winings.contains(&v))
            .enumerate()
            .map(|(i, v)| match i {
                1 | 2 | 3 => 2,
                _ => 1,
            }).count();
        match found {
            0 => 0,
            n => 2_u32.pow((n - 1) as u32),
        }
    }

    fn extract_numbers(line: &str) -> Vec<u32>
    {
        line
            .split(" ")
            .filter_map(|v| match v.parse() {
                Ok(v) => Some(v),
                Err(_) => None,
            })
            .collect::<Vec<u32>>()
    }

    fn parse(line: &str) -> Result<Card, &str>
    {
        if !line.starts_with("Card ")
        {
            return Err("Invalid line");
        }

        let line = line.strip_prefix("Card ").unwrap();
        let to_strip: usize;
        let id: u32;
        if let Some(index) = line.chars().position(|c| c == ':') {
            id = match line
                .chars()
                .take(index)
                .collect::<String>()
                .trim()
                .parse() {
                Ok(val) => val,
                Err(_) => return Err("Invalid card number"),
            };
            to_strip = index + 1;
        }
        else
        {
            return Err("Missing colon");
        }

        let line_string = line.chars().skip(to_strip).collect::<String>();

        let parts = line_string
            .split("|")
            .collect::<Vec<&str>>();
        if parts.len() != 2
        {
            return Err("Invalid line");
        }

        Ok(Card {
            id: id,
            winings: Card::extract_numbers(parts[0]),
            values: Card::extract_numbers(parts[1]),
        })
    }
}

fn main() -> io::Result<()>
{
    if env::args().len() != 2
    {
        panic!("Usage: ./main <filename>");
    }

    let filename = env::args().nth_back(0).unwrap();

    let file = fs::File::open(filename)?;

    let mut reader = BufReader::new(file);

    let mut res: u32 = 0;

    let lines = reader.lines()
        .filter_map(|l| match l {
            Err(_) => None,
            Ok(v) => Some(v),
        })
        .collect::<Vec<String>>();

    let res = lines
        .iter()
        .filter_map(|l| match Card::parse(l.as_str()) {
            Ok(v) => Some(v.get_results()),
            Err(e) => {
                println!("error on {l} => {e}");
                None
            },
        })
        .reduce(|a, b| a + b)
        .unwrap_or(0);

    println!("{res}");

    Ok(())
}
