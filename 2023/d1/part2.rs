use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs;
use std::vec::Vec;

fn get_characters(line: &str) -> Vec<u32>
{
    let str = line.to_lowercase().to_string();
    let str = str.replace("one", "o1e");
    let str = str.replace("two", "t2e");
    let str = str.replace("three", "t3e");
    let str = str.replace("four", "f4r");
    let str = str.replace("five", "f5e");
    let str = str.replace("six", "s6x");
    let str = str.replace("seven", "s7n");
    let str = str.replace("eight", "e8t");
    let str = str.replace("nine", "n9e");

    let res = str.chars()
        .filter_map(|x| x.to_digit(10))
        .collect();

    res
}

fn main() -> io::Result<()> 
{

    if env::args().len() != 2
    {
        panic!("Usage: ./main <filename>");
    }

    let filename = env::args().nth_back(0).unwrap();

    let file = fs::File::open(filename)?;

    let reader = BufReader::new(file);

    let mut res: u32 = 0;

    for line in reader.lines()
    {
        if let Ok(val) = line {
            let numbers = get_characters(val.as_str());
            let partial = numbers.first().unwrap() * 10 + numbers.last().unwrap();
            res += partial;
            println!("{val} => {numbers:?} => {partial}");
        }
    }
    println!("{res}");

    Ok(())
}
