use std::env;
use std::fs;
use std::io::{self, BufReader, BufRead};

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

            let numbers: Vec<u32> = val.chars()
                .filter_map(|a| a.to_digit(10))
                .collect();

            let partial = numbers.first().unwrap() * 10 + numbers.last().unwrap();
            res += partial;

        }
    }
    println!("{res}");

    Ok(())
}
