pub enum Cubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    pub fn new() -> Set {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn is_possible(&self, max: Set) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    pub fn is_possible(&self, max: Set) -> bool {
        self.sets.iter().all(|s| s.is_possible(max))
    }
}

peg::parser! {
    grammar parser() for str {
        rule number() -> u32 = n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        rule red() -> Cubes = " " n:number() " red" { Cubes::Red(n) }
        rule green() -> Cubes = " " n:number() " green" { Cubes::Green(n) }
        rule blue() -> Cubes = " " n:number() " blue" { Cubes::Blue(n) }

        rule set() -> Set = s:(red() / green() / blue()) ++ "," {
            s.into_iter().fold(Set::new(), |mut set, e| {
                match e {
                    Cubes::Red(n) => set.red = n,
                    Cubes::Green(n) => set.green = n,
                    Cubes::Blue(n) => set.blue = n,
                };
                set
            })
        }

        rule sets() -> Vec<Set> = set() ++ ";"
        rule id() -> u32 = "Game " n:number() ":" { n }

        pub rule game() -> Game = id:id() sets:sets() { Game { id, sets } }
    }
}

pub fn part_1(input: String) {
    let max_set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut result = 0;

    for line in input.lines() {
        if let Ok(game) = parser::game(line) {
            if game.is_possible(max_set) {
                result += game.id;
            }
        }
    }

    println!("{result}");
}

pub fn part_2(input: String) {
    let mut result = 0;

    for line in input.lines() {
        if let Ok(game) = parser::game(line) {
            let mut max = Set::new();

            for set in game.sets {
                max = Set {
                    red: u32::max(max.red, set.red),
                    green: u32::max(max.green, set.green),
                    blue: u32::max(max.blue, set.blue),
                }
            }

            result += max.power();
        }
    }

    println!("{result}");
}