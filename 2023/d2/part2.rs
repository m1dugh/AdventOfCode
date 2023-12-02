use std::fmt;
use std::io::{self, BufReader, BufRead};
use std::env;
use std::fs;

struct GameSet
{
    red: u32,
    green: u32,
    blue: u32,
}

struct Game
{
    id: u32,
    games: Vec<GameSet>,
}

impl GameSet
{
    fn new(red: u32, green: u32, blue: u32) -> GameSet
    {
        GameSet
        {
            red: red,
            green: green,
            blue: blue,
        }
    }

    fn parse(line: &str) -> Result<GameSet, &str>
    {
        let entries = line.split(",").map(|val| val.trim());

        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for entry in entries {
            let values = entry.split(" ").collect::<Vec<&str>>();
            let count = match values[0].parse() {
                Ok(val) => val,
                Err(_) => return Err("Invalid line"),
            };

            match values[1]
            {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => return Err("Invalid line"),
            }
        }

        Ok(GameSet::new(red, green, blue))
    }
}

impl Game
{
    fn new(id: u32, games: Vec<GameSet>) -> Game
    {
        Game
        {
            id: id,
            games: games,
        }
    }

    fn get_max_set(&self) -> GameSet
    {
        let mut res = GameSet::new(0, 0, 0);
        for game in &self.games
        {
            if game.red > res.red
            {
                res.red = game.red;
            }

            if game.green > res.green
            {
                res.green = game.green;
            }

            if game.blue > res.blue
            {
                res.blue = game.blue;
            }
        }

        res
    }

    fn parse(line: &str) -> Result<Game, &str>
    {
        if !line.starts_with("Game ")
        {
            return Err("Invalid line");
        }

        let line = line.strip_prefix("Game ").unwrap();
        let to_strip: usize;
        let id: u32;
        if let Some(index) = line.chars().position(|c| c == ':') {
            id = match line.chars()
                .take(index)
                .collect::<String>()
                .parse() {
                Ok(val) => val,
                Err(_) => return Err("Invalid line"),
            };
            to_strip = index + 1;
        }
        else
        {
            return Err("Invalid line");
        }

        let line_string = line.chars().skip(to_strip).collect::<String>();

        let entries = line_string.split(";")
            .map(|val| val.trim())
            .collect::<Vec::<&str>>();

        let game_sets = entries
            .iter()
            .filter_map(|entry| match GameSet::parse(entry) {
                Err(_) => None,
                Ok(g) => Some(g),
            })
            .collect::<Vec<GameSet>>();

        Ok(Self::new(id, game_sets))
    }
}

impl fmt::Debug for GameSet
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl fmt::Display for Game
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "({}, {:?})", self.id, self.games)
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

    let reader = BufReader::new(file);

    let mut res: u32 = 0;

    for l in reader.lines()
    {
        if let Ok(line) = l
        {
            let game = match Game::parse(line.as_str())
            {
                Err(e) => panic!("{}", e),
                Ok(v) => v,
            };

            let max_set = game.get_max_set();
            let partial = max_set.red * max_set.green * max_set.blue;
            println!("{}: {max_set:?} => {partial}", game.id);
            res += partial;
        }
    }

    println!("{}", res);

    Ok(())
}
