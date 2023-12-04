use std::collections::VecDeque;

#[derive(Debug)]
pub struct Card {
    id: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
    copies: u32,
}

impl Card {
    pub fn winnings_you_have(&self) -> u32 {
        self.have
            .iter()
            .filter(|h| self.winning.contains(h))
            .count() as u32
    }

    pub fn score(&self) -> u32 {
        let winnings_you_have = self.winnings_you_have();

        if winnings_you_have == 0 {
            0
        } else {
            2u32.pow(winnings_you_have - 1)
        }
    }
}

peg::parser! {
    grammar parser() for str {
        rule number() -> u32 = " "* n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

        pub rule card() -> Card = "Card " id:number() ": " w:number() ++ " " " | " h:number() ++ " " ![_] {
            Card {
                id,
                winning: w,
                have: h,
                copies: 1,
            }
        }
    }
}

pub fn part_1(input: String) {
    let mut result = 0;

    for line in input.lines() {
        if let Ok(card) = parser::card(line) {
            result += card.score();
        }
    }

    println!("{result}");
}

pub fn part_2(input: String) {
    let mut result = 0;

    let mut cards: VecDeque<_> = input.lines().flat_map(parser::card).collect();

    while let Some(card) = cards.pop_front() {
        let winnings = card.winnings_you_have() as usize;

        for _ in 0..card.copies {
            for next_card in cards.iter_mut().take(winnings) {
                next_card.copies += 1;
            }
            result += 1;
        }
    }

    println!("{result}");
}
