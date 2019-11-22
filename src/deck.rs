extern crate rand;
extern crate serde;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

pub fn return_value(card: Card) -> u8 {
    match card.value {
        CardValue::Ace => 1,
        CardValue::Two => 2,
        CardValue::Three => 3,
        CardValue::Four => 4,
        CardValue::Five => 5,
        CardValue::Six => 6,
        CardValue::Seven => 7,
        CardValue::Eight => 8,
        CardValue::Nine => 9,
        CardValue::Ten => 10,
        CardValue::Jack => 11,
        CardValue::Queen => 12,
        CardValue::King => 13,
    }
}

pub fn return_play_value(card: Card) -> u8 {
    match card.value {
        CardValue::Ace => 1,
        CardValue::Two => 2,
        CardValue::Three => 3,
        CardValue::Four => 4,
        CardValue::Five => 5,
        CardValue::Six => 6,
        CardValue::Seven => 7,
        CardValue::Eight => 8,
        CardValue::Nine => 9,
        CardValue::Ten => 10,
        CardValue::Jack => 10,
        CardValue::Queen => 10,
        CardValue::King => 10,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}

#[derive(Debug, Clone)]
pub struct Deck {
    // card_vector public for debugging purposes, deal cards with reset_deck() and deal()
    pub card_vector: Vec<Card>,
}

pub fn new_deck() -> Deck {
    Deck {
        card_vector: Vec::new(),
    }
}

impl Deck {
    // Clears the card vector of the deck and populates it with an organized deck of cards
    fn populate(&mut self) {
        // Ensures that the deck does not have cards in it when it's populated
        self.card_vector.clear();

        // For each possible suit
        for suit_loop in 0..4 {
            let active_suit = match suit_loop {
                0 => CardSuit::Hearts,
                1 => CardSuit::Diamonds,
                2 => CardSuit::Clubs,
                3 => CardSuit::Spades,
                _ => {
                    panic!("Deck populate function atttempted to create invalid card suit");
                }
            };

            // For each possible value
            for value_loop in 0..13 {
                let active_value = match value_loop {
                    0 => CardValue::Ace,
                    1 => CardValue::Two,
                    2 => CardValue::Three,
                    3 => CardValue::Four,
                    4 => CardValue::Five,
                    5 => CardValue::Six,
                    6 => CardValue::Seven,
                    7 => CardValue::Eight,
                    8 => CardValue::Nine,
                    9 => CardValue::Ten,
                    10 => CardValue::Jack,
                    11 => CardValue::Queen,
                    12 => CardValue::King,
                    _ => panic!("Deck populate function attemted to create invalid card value"),
                };

                // Push a card of the given suit and value to the deck
                self.card_vector.push(Card {
                    suit: active_suit,
                    value: active_value,
                })
            }
        }
    }

    // Randomizes the order of the deck
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.card_vector.shuffle(&mut rng);
    }

    // Resets the deck to a full and shuffled state
    pub fn reset_deck(&mut self) {
        self.populate();
        self.shuffle();
    }

    // Return a single card and pop it off the deck
    pub fn deal(&mut self) -> Card {
        match self.card_vector.pop() {
            None => {
                //This should never occur with regular play
                panic!("Dealt more than 52 cards");
            }
            Some(card) => card,
        }
    }
}
