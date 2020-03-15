use std::char;

// Returns a card object based on a specified value and suit character for the purpose of
// testing
pub fn return_card(set_value: char, set_suit: char) -> super::deck::Card {
    let set_value: super::deck::CardValue = match set_value {
        'A' => super::deck::CardValue::Ace,
        '2' => super::deck::CardValue::Two,
        '3' => super::deck::CardValue::Three,
        '4' => super::deck::CardValue::Four,
        '5' => super::deck::CardValue::Five,
        '6' => super::deck::CardValue::Six,
        '7' => super::deck::CardValue::Seven,
        '8' => super::deck::CardValue::Eight,
        '9' => super::deck::CardValue::Nine,
        'T' => super::deck::CardValue::Ten,
        'J' => super::deck::CardValue::Jack,
        'Q' => super::deck::CardValue::Queen,
        'K' => super::deck::CardValue::King,
        _ => panic!("Unexpexted value in return_card()"),
    };

    let set_suit: super::deck::CardSuit = match set_suit {
        'H' => super::deck::CardSuit::Hearts,
        'D' => super::deck::CardSuit::Diamonds,
        'C' => super::deck::CardSuit::Clubs,
        'S' => super::deck::CardSuit::Spades,
        _ => panic!("Unexpected suit in return_card()"),
    };

    super::deck::Card {
        value: set_value,
        suit: set_suit,
    }
}
