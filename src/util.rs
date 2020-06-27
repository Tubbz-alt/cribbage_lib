use std::char;

// Returns a card object based on a specified value and suit character for the purpose of
// testing
pub(crate) fn return_card(set_value: char, set_suit: char) -> super::deck::Card {
    let set_value: super::deck::CardValue = match set_value {
        'A' => super::deck::CardValue::Ace,
        '1' => super::deck::CardValue::Ace,
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

// Returns a list of all possible rule variants for the purpose of testing
pub(crate) fn return_variants() -> Vec<super::settings::RuleVariant> {
    vec![
        super::settings::RuleVariant::TwoStandard,
        super::settings::RuleVariant::TwoFiveCard,
        super::settings::RuleVariant::TwoSevenCard,
        super::settings::RuleVariant::ThreeStandard,
        super::settings::RuleVariant::ThreeCaptain,
        super::settings::RuleVariant::FourIndividual,
        super::settings::RuleVariant::FourPairs,
        super::settings::RuleVariant::FiveStandard,
        super::settings::RuleVariant::SixPairs,
    ]
}

// Returns a u8 with the number of players for a given rule variant
pub(crate) fn return_num_players_for_variant(variant: super::settings::RuleVariant) -> u8 {
    match variant {
        crate::settings::RuleVariant::TwoStandard => 2,
        crate::settings::RuleVariant::TwoFiveCard => 2,
        crate::settings::RuleVariant::TwoSevenCard => 2,
        crate::settings::RuleVariant::ThreeStandard => 3,
        crate::settings::RuleVariant::ThreeCaptain => 3,
        crate::settings::RuleVariant::FourIndividual => 4,
        crate::settings::RuleVariant::FourPairs => 4,
        crate::settings::RuleVariant::FiveStandard => 5,
        crate::settings::RuleVariant::SixPairs => 6,
    }
}

// Processes a score change; ensures change also occurs with partner and check for victory (or loss
// with lowball). Returns true if a player has reached the threshold, otherwise return false
pub(crate) fn process_score(
    game: &mut crate::GameImpl,
    player_index: usize,
    score_change: u8,
) -> bool {
    game.players[player_index].change_score(score_change);
    if let Some(partner_index) = game.players[player_index].partner_index {
        game.players[partner_index as usize].change_score(score_change);
    }

    match game.settings.unwrap().variant {
        crate::settings::RuleVariant::TwoFiveCard => {
            if game.players[player_index].front_peg_pos >= 61 {
                game.state = crate::GameState::Win;
                true
            } else {
                false
            }
        }
        crate::settings::RuleVariant::TwoSevenCard => {
            if game.players[player_index].front_peg_pos >= 151 {
                game.state = crate::GameState::Win;
                true
            } else {
                false
            }
        }
        crate::settings::RuleVariant::ThreeCaptain => {
            if game.players[player_index].partner_index.is_some() {
                if game.players[player_index].front_peg_pos >= 121 {
                    game.state = crate::GameState::Win;
                    true
                } else {
                    false
                }
            } else {
                if game.players[player_index].front_peg_pos >= 61 {
                    game.state = crate::GameState::Win;
                    true
                } else {
                    false
                }
            }
        }
        _ => {
            if game.players[player_index].front_peg_pos >= 121 {
                game.state = crate::GameState::Win;
                true
            } else {
                false
            }
        }
    }
}
