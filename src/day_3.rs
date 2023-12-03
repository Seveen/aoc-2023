#[derive(Debug, Clone, Copy)]
pub struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    pub fn is_adjacent(&self, other: Pos) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

#[derive(Debug)]
pub struct Number {
    value: u32,
    positions: Vec<Pos>,
}

impl Number {
    pub fn new() -> Self {
        Self {
            value: 0,
            positions: Vec::new(),
        }
    }
}

pub struct Symbol {
    value: char,
    position: Pos,
}

fn parse_input(input: String) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut number: Option<Number> = None;

        for (x, character) in line.char_indices() {
            if character.is_ascii_digit() {
                if let Some(digit) = character.to_digit(10) {
                    let mut n = match number {
                        Some(n) => n,
                        None => Number::new(),
                    };
                    n.value = n.value * 10 + digit;
                    n.positions.push(Pos { x, y });
                    number = Some(n);
                }
            } else {
                if let Some(n) = number.take() {
                    numbers.push(n);
                }
                if character != '.' {
                    symbols.push(Symbol {
                        position: Pos { x, y },
                        value: character,
                    });
                }
            }
        }

        // Number at end of line
        if let Some(n) = number.take() {
            numbers.push(n);
        }
    }

    (numbers, symbols)
}

pub fn part_1(input: String) {
    let (numbers, symbols) = parse_input(input);

    let mut result = 0;
    'part: for number in numbers {
        for position in number.positions {
            if symbols.iter().any(|symbol| symbol.position.is_adjacent(position)) {
                result += number.value;
                continue 'part;
            }
        }
    }

    println!("{result}");
}

pub fn part_2(input: String) {
    let (numbers, symbols) = parse_input(input);

    let mut result = 0;
    for symbol in symbols {
        if symbol.value == '*' {
            let mut adjacents = Vec::new();
            for number in &numbers {
                if number.positions.iter().any(|p| p.is_adjacent(symbol.position)) {
                    adjacents.push(number.value);
                }
            }
            
            if adjacents.len() == 2 {
                result += adjacents[0] * adjacents[1];
            }
        }
    }

    println!("{result}")
}
