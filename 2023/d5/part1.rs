use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
struct InputLine
{
    source_start: u32,
    destination_start: u32,
    len: usize,
}

type InputRange = Vec<InputLine>;

fn input_range_get_value(range: &InputRange, seed: u32) -> u32
{
    for line in range
    {
        if let Some(found) = line.find_value(seed)
        {
            return found;
        }
    }

    return seed;
}

impl InputLine
{
    fn parse(line: &str) -> Result<Self, &str>
    {
        let parts = line
            .split(" ")
            .collect::<Vec<&str>>();
        if parts.len() != 3
        {
            return Err("invalid number of fields");
        }

        let values = parts
            .iter()
            .map(|v| v.trim().parse())
            .filter_map(|r| match r {
                Ok(v) => Some(v),
                Err(e) => panic!("{}", e),
            })
            .collect::<Vec<u32>>();

        Ok(InputLine {
            destination_start: values[0],
            source_start: values[1],
            len: values[2] as usize,
        })
    }

    fn find_value(&self, value: u32) -> Option<u32>
    {
        if value >= self.source_start
        {
            let offset = value - self.source_start;
            if offset < self.len as u32 { 
                return Some(self.destination_start + offset);
            }
        }

        None
    }
}

#[derive(Debug)]
struct Input
{
    seeds: Vec<u32>,
    seed_to_soil: InputRange,
    soil_to_fertilizer: InputRange,
    fertilizer_to_water: InputRange,
    water_to_light: InputRange,
    light_to_temp: InputRange,
    temp_to_humid: InputRange,
    humid_to_loc: InputRange,
}

impl Input
{
    fn new() -> Self
    {
        Input {
            seeds: Vec::<u32>::new(),
            seed_to_soil: Vec::<InputLine>::new(),
            soil_to_fertilizer: Vec::<InputLine>::new(),
            fertilizer_to_water: Vec::<InputLine>::new(),
            water_to_light: Vec::<InputLine>::new(),
            light_to_temp: Vec::<InputLine>::new(),
            temp_to_humid: Vec::<InputLine>::new(),
            humid_to_loc: Vec::<InputLine>::new(),
        }
    }

    fn collect_line(&mut self, values: &Vec<String>) -> Result<(), &str>
    {
        let key = values[0].clone();
        let values = values
            .iter()
            .skip(1)
            .map(|s| s.clone())
            .collect::<Vec<String>>();
        let begin = key.split("-").nth(0).unwrap();
        let inputs = values
            .iter()
            .filter_map(|v| match InputLine::parse(v) {
                Ok(v) => Some(v),
                Err(e) => {
                    println!("error at parsing entry {e}");
                    None
                },
            })
        .collect::<Vec<InputLine>>();

        match begin {
            "seed" => self.seed_to_soil = inputs,
            "soil" => self.soil_to_fertilizer = inputs,
            "fertilizer" => self.fertilizer_to_water = inputs,
            "water" => self.water_to_light = inputs,
            "light" => self.light_to_temp = inputs,
            "temperature" => self.temp_to_humid = inputs,
            "humidity" => self.humid_to_loc = inputs,
            _ => return Err("unknown key {key}"),
        };

        Ok(())
    }

    fn build_location_map(&self) -> HashMap<u32, u32>
    {
        let mut res = HashMap::<u32, u32>::new();

        for seed in &self.seeds {
            let matched = *seed;
            let matched = input_range_get_value(&self.seed_to_soil, matched);
            let matched = input_range_get_value(&self.soil_to_fertilizer, matched);
            let matched = input_range_get_value(&self.fertilizer_to_water, matched);
            let matched = input_range_get_value(&self.water_to_light, matched);
            let matched = input_range_get_value(&self.light_to_temp, matched);
            let matched = input_range_get_value(&self.temp_to_humid, matched);

            let matched = input_range_get_value(&self.humid_to_loc, matched);

            res.insert(*seed, matched);
        }

        res
    }

    fn parse(lines: &mut io::Lines<BufReader<fs::File>>) -> Result<Input, &str>
    {
        let line = match lines.next() {
            Some(Ok(v)) => v,
            None | Some(Err(_)) => return Err("could not parse line"),
        };

        if !line.starts_with("seeds:")
        {
            return Err("missing seeds");
        }

        let mut res = Input::new();

        let line = line
            .strip_prefix("seeds:")
            .unwrap()
            .trim();
        let seeds = line
            .split(" ")
            .filter_map(|v| match v.trim().parse() {
                Ok(v) => Some(v),
                Err(e) => {
                    println!("err {e}");
                    None
                }
            })
            .collect::<Vec<u32>>();

        res.seeds = seeds;

        let mut collected_lines = Vec::<String>::new();
        for line in lines
        {
            if let Ok(value) = line {
                if value.len() == 0 {
                    if collected_lines.len() != 0 {
                        let collect_line_res = res.collect_line(&collected_lines);
                        match collect_line_res {
                            Err(e) => panic!("{}", e),
                            Ok(_) => (),
                        };
                    }
                    collected_lines.clear();
                }
                else
                {
                    collected_lines.push(value.clone());
                }
            }
        }

        if collected_lines.len() != 0
        {
            res.collect_line(&collected_lines);
        }

        Ok(res)
    }
}

fn get_reader() -> BufReader<fs::File>
{
    if env::args().len() != 2
    {
        panic!("Usage: ./main <filename>");
    }

    let filename = env::args().nth_back(0).unwrap();

    let file = match fs::File::open(filename) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    BufReader::new(file)
}

fn main() -> io::Result<()>
{
    let reader = get_reader();

    let mut lines = reader.lines();
    let input = match Input::parse(&mut lines) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };

    let location_map = input.build_location_map();

    let res = location_map
        .values()
        .reduce(|a, b| match a < b {
            true => a,
            false => b,
        }).unwrap_or(&0);

    println!("{res}");

    Ok(())
}
