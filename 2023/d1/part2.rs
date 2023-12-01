use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs;
use std::vec::Vec;

fn find_substring(main: &str, sub: &str, start: u32) -> i64
{
    let mut i: usize = start as usize;
    let mut subi: usize = 0;
    let mut res = -1;
    while i < main.len()
    {
        if subi == sub.len()
        {
            return res;
        }

        if sub.chars().nth(subi) == main.chars().nth(i)
        {
            if subi == 0
            {
                res = i as i64;
            }
            subi += 1;
        }
        else
        {
            subi = 0;
        }

        i += 1;
    }

    if subi == sub.len()
    {
        res
    }
    else
    {
        -1
    }

}

static NUMBERS: &'static [&'static str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_characters(line: &str) -> Vec<u32>
{
    let str = line.to_lowercase();
    let mut numbers: Vec<u32> = Vec::<u32>::new();

    let mut i = 0;

    while i < str.len()
    {
        let mut new_start_index = str.len();
        let mut value: usize = 0;
        let mut sublen = 0;
        for (pos, e) in NUMBERS.iter().enumerate()
        {
            let tmp = find_substring(str.as_str(), e, i as u32);
            if tmp != -1 && (tmp as usize) < new_start_index
            {
                new_start_index = tmp as usize;
                value = pos + 1;
                sublen = e.len();
            }
        }

        if value > 0
        {
            let mut sub: Vec<u32> = str.chars()
                .skip(i)
                .take(new_start_index - i)
                .filter_map(|x| x.to_digit(10))
                .collect();

            numbers.append(&mut sub);
            numbers.push(value as u32);
            i = new_start_index + sublen;
        }
        else
        {
            let mut sub: Vec<u32> = str.chars()
                .skip(i)
                .filter_map(|x| x.to_digit(10))
                .collect();

            numbers.append(&mut sub);
            break;
        }
    }

    numbers
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
