use std::str::FromStr;

pub fn puzzle1(input: &str, bag: RGB) {
    let mut c = 0;
    let valid_games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(move |game| game.max.r <= bag.r && game.max.g <= bag.g && game.max.b <= bag.b);
    for game in valid_games {
        c += game.id;
    }
    println!("{c}");
}

pub fn puzzle2(input: &str) {
    let mut c = 0;
    let valid_games = input.lines().map(|line| line.parse::<Game>().unwrap());
    for game in valid_games {
        c += game.max.r * game.max.g * game.max.b;
    }
    println!("{c}");
}

pub struct RGB {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

struct Game {
    id: i32,
    plays: Vec<RGB>,
    max: RGB,
}

impl FromStr for RGB {
    type Err = ();
    fn from_str(input: &str) -> Result<RGB, ()> {
        let mut rgb = RGB { r: 0, g: 0, b: 0 };
        for col in input.split(", ") {
            let sep: Vec<_> = col.split(" ").collect();
            let n = sep[0].parse().unwrap();
            if sep[1] == "red" {
                rgb.r = n;
            } else if sep[1] == "green" {
                rgb.g = n;
            } else if sep[1] == "blue" {
                rgb.b = n;
            }
        }
        Ok(rgb)
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(input: &str) -> Result<Game, ()> {
        let mut game = Game {
            id: 0,
            plays: Vec::new(),
            max: RGB { r: 0, g: 0, b: 0 },
        };
        let sp: Vec<_> = input.split(": ").collect();
        game.id = sp[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();
        for instance in sp[1].split("; ") {
            let rgb: RGB = instance.parse().unwrap();
            if rgb.r > game.max.r {
                game.max.r = rgb.r;
            }
            if rgb.g > game.max.g {
                game.max.g = rgb.g;
            }
            if rgb.b > game.max.b {
                game.max.b = rgb.b;
            }
            game.plays.push(rgb);
        }
        Ok(game)
    }
}
