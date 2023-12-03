use std::cmp::PartialEq;
use std::io::{self, BufRead, BufReader, Read};
use std::fs;
use std::env;
use std::fmt;

#[derive(Clone)]
struct Position
{
    x: u32,
    y: u32,
}

impl fmt::Debug for Position
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position
{
    fn new(x: u32, y: u32) -> Position
    {
        Position { x: x, y: y }
    }

    fn is_adjacent(&self, other: &Position, boundaries: &Position) -> bool
    {
        let minx: u32;
        let maxx: u32;
        if self.x == 0
        {
            minx = self.x;
        }
        else
        {
            minx = self.x - 1;
        }

        if self.x >= boundaries.x
        {
            maxx = boundaries.x;
        }
        else
        {
            maxx = self.x + 1;
        }

        let miny: u32;
        let maxy: u32;
        if self.y == 0
        {
            miny = self.y;
        }
        else
        {
            miny = self.y - 1;
        }

        if self.y >= boundaries.y
        {
            maxy = boundaries.y;
        }
        else
        {
            maxy = self.y + 1;
        }

        other.y <= maxy && other.y >= miny && other.x >= minx && other.x <= maxx
    }

    fn generate_range(&self, other: &Position) -> Vec<Position>
    {
        let minx: u32;
        let maxx: u32;
        if self.x < other.x
        {
            minx = self.x;
            maxx = other.x;
        }
        else
        {
            minx = other.x;
            maxx = self.x;
        }

        let miny: u32;
        let maxy: u32;
        if self.y < other.y
        {
            miny = self.y;
            maxy = other.y;
        }
        else
        {
            miny = other.y;
            maxy = self.y;
        }

        let mut res = Vec::<Position>::new();

        for x in minx..(maxx + 1)
        {
            for y in miny..(maxy + 1)
            {
                res.push(Position::new(x, y));
            }
        }

        res
    }
}

impl PartialEq for Position
{
    fn eq(&self, other: &Self) -> bool
    {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Clone)]
struct Value
{
    value: u32,
    positions: Vec<Position>,
}

impl Value
{
    fn is_adjacent(&self, position: &Position, boundaries: &Position) -> bool
    {
        (&self.positions).iter()
            .any(|pos| pos.is_adjacent(position, boundaries))
    }

    fn new(value: u32, start: &Position, len: u32) -> Self
    {
        let end = &Position::new(start.x + len, start.y);
        Value { value: value, positions: Position::generate_range(start, end) }
    }
}

impl fmt::Debug for Value
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {{{:?}, {:?}}})", self.value, self.positions.first().unwrap(), self.positions.last().unwrap())
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

    let line_len: u32 = lines[0].len() as u32;
    let count: u32 = lines.len() as u32;

    let boundaries = Position::new(line_len - 1, count - 1);
    let mut values = Vec::<Value>::new();
    let mut symbols = Vec::<Position>::new();

    let mut y = 0;
    for line in lines
    {
        let mut acc: u32 = 0;
        let mut acc_len = 0;

        let mut x = 0;
        for c in line.chars()
        {
            if c.is_digit(10)
            {
                acc = 10 * acc + c.to_digit(10).unwrap();
                acc_len += 1;
            }
            else
            {
                if acc > 0
                {
                    let start = Position::new(x - acc_len, y);
                    let value = Value::new(acc, &start, acc_len - 1);
                    values.push(value);
                    acc = 0;
                    acc_len = 0;
                }
                if c != '.'
                {
                    symbols.push(Position::new(x, y));
                }

            }

            x += 1;
        }

        if acc > 0
        {
            let start = Position::new(x - acc_len, y);
            let value = Value::new(acc, &start, acc_len - 1);
            values.push(value.clone());
            acc = 0;
            acc_len = 0;
        }
        
        y += 1;
    }

    let mut res = 0;
    for symbol in symbols
    {
        let gears = values.clone()
            .into_iter()
            .filter(|val| val.is_adjacent(&symbol, &boundaries))
            .collect::<Vec<Value>>();

        if gears.len() == 2
        {
            res += gears.into_iter().map(|a| a.value).reduce(|a, b| a * b).unwrap();
            values = values
                .clone()
                .into_iter()
                .filter(|val| !val.is_adjacent(&symbol, &boundaries))
                .collect::<Vec<Value>>();
        }
    }
    
    println!("{res}");

    Ok(())
}
