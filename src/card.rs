use std::{fmt::Debug, num::ParseIntError};
use rand::Rng;

#[derive(PartialEq, Clone, Copy)]
pub struct Card<'a> {
    suit: &'a str,
    value: &'a str,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct CardNum {
    pub suit: u32,
    pub value: u32,
}

impl<'a> Debug for Card<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.suit, self.value)
    }
}

impl<'a> Card<'a> {
    fn new_card(cards_suit: &'a Vec<&str>, cards_value: &'a Vec<&str>) -> Self {
        let mut rand = rand::thread_rng();
        let suit = cards_suit[rand.gen_range(0..cards_suit.len())];
        let value = cards_value[rand.gen_range(0..cards_value.len())];

        Card { suit , value }
    }

    pub fn take_cards(cards_suit: &'a Vec<&str>, cards_value: &'a Vec<&str>, count: usize, dependencies: Option<Vec<&Vec<Card>>>) -> Vec<Self> {
        let mut cards: Vec<Self> = Vec::new();

        match dependencies {
            Some(dep) => {
                while cards.len() != count {
                    let card = Self::new_card(&cards_suit, &cards_value);
                    for cards_vec in &dep {
                        for c in *cards_vec {
                            if !cards.contains(c) && !cards.contains(&card) {
                                cards.push(card);
                            }
                        }
                    }
                }
            }
            None => {
                while cards.len() != count {
                    let card = Self::new_card(&cards_suit, &cards_value);
                    if !cards.contains(&card) {
                        cards.push(card)
                    }
                }
            }
        }
        cards
    }

    pub fn to_num(&mut self) -> CardNum {
        let card_suit_num: u32;
        let card_value_num: Result<u32, ParseIntError> = self.value.trim().parse();

        match self.suit.trim() {
            "cross" => card_suit_num = 1,
            "spades" => card_suit_num = 2,
            "hearts" => card_suit_num = 3,
            "diamonds" => card_suit_num = 4,

            _ => panic!("Problem with suits of cards!"),
        }

        match card_value_num {
            Ok(value_num) => {
                CardNum { suit: card_suit_num, value: value_num }
            }

            Err(..) => {
                match self.value.trim() {
                    "jack" => CardNum { suit: card_suit_num, value: 11 },
                    "queen" => CardNum { suit: card_suit_num, value: 12 },
                    "king" => CardNum { suit: card_suit_num, value: 13 },
                    "ace" => CardNum { suit: card_suit_num, value: 14 },

                    _ => panic!("Problem with values of cards!"),
                }
            }
        }
    }
}