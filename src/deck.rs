extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;

#[cfg(test)]
mod test {
    #[test]
    fn return_play_value() {
        assert_eq!(
            super::return_play_value(crate::util::return_card('A', 'H')),
            1
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('2', 'H')),
            2
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('3', 'H')),
            3
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('4', 'H')),
            4
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('5', 'H')),
            5
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('6', 'H')),
            6
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('7', 'H')),
            7
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('8', 'H')),
            8
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('9', 'H')),
            9
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('T', 'H')),
            10
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('J', 'H')),
            10
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('Q', 'H')),
            10
        );
        assert_eq!(
            super::return_play_value(crate::util::return_card('K', 'H')),
            10
        );
    }

    #[test]
    fn return_value() {
        assert_eq!(super::return_value(crate::util::return_card('A', 'H')), 1);
        assert_eq!(super::return_value(crate::util::return_card('2', 'H')), 2);
        assert_eq!(super::return_value(crate::util::return_card('3', 'H')), 3);
        assert_eq!(super::return_value(crate::util::return_card('4', 'H')), 4);
        assert_eq!(super::return_value(crate::util::return_card('5', 'H')), 5);
        assert_eq!(super::return_value(crate::util::return_card('6', 'H')), 6);
        assert_eq!(super::return_value(crate::util::return_card('7', 'H')), 7);
        assert_eq!(super::return_value(crate::util::return_card('8', 'H')), 8);
        assert_eq!(super::return_value(crate::util::return_card('9', 'H')), 9);
        assert_eq!(super::return_value(crate::util::return_card('T', 'H')), 10);
        assert_eq!(super::return_value(crate::util::return_card('J', 'H')), 11);
        assert_eq!(super::return_value(crate::util::return_card('Q', 'H')), 12);
        assert_eq!(super::return_value(crate::util::return_card('K', 'H')), 13);
    }

    #[test]
    fn reset_deck() {
        let mut deck = super::Deck::new();
        deck.reset_deck();
        let first_deck = deck.clone();
        deck.reset_deck();
        assert!(deck != first_deck);
    }

    #[test]
    fn deal() {
        let mut deck = super::Deck::new();
        assert_eq!(deck.deal(), crate::util::return_card('A', 'H'));
        for _ in 0..51 {
            let _ = deck.deal();
        }
    }

    #[test]
    #[should_panic(expected = "Dealt more than 52 cards")]
    fn deal_past_52() {
        let mut deck = super::Deck::new();
        for _ in 0..53 {
            let _ = deck.deal();
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub value: CardValue,
    pub suit: CardSuit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    card_vector: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck {
            card_vector: Vec::new(),
        };

        deck.populate();

        deck
    }

    // Clears the card vector of the deck and populates it with an organized deck of cards
    fn populate(&mut self) {
        // Ensures that the deck does not have cards in it when it's populated
        self.card_vector.clear();

        // For each possible suit
        for suit_loop in (0..4).rev() {
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
            for value_loop in (0..13).rev() {
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
