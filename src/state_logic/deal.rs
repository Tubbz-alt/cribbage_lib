use crate as game;
use crate::deck;
use crate::game_process_return;

pub(crate) fn process_deal(
    game: &mut game::GameImpl,
) -> Result<game_process_return::Success, game_process_return::Error> {
    // Start with a shuffled deck unless debug mode is enabled in which case use the deck object as
    // it exists
    if !game.is_debug {
        game.deck.reset_deck();
    }

    for player in &mut game.players {
        player.hand.clear();
    }

    game.crib.clear();

    if let Some(settings) = game.settings {
        match settings.variant {
            game::settings::RuleVariant::TwoStandard => deal_two(6, game),
            game::settings::RuleVariant::TwoFiveCard => deal_two(5, game),
            game::settings::RuleVariant::ThreeStandard => deal_three(game),
            game::settings::RuleVariant::ThreeCaptain => deal_three(game),
            game::settings::RuleVariant::FourIndividual => deal_four(game),
            game::settings::RuleVariant::FourPairs => deal_four(game),
            game::settings::RuleVariant::FiveStandard => deal_five(game),
            game::settings::RuleVariant::SixPairs => deal_six(game),
        };
    }

    game.state = game::GameState::Sort;
    Ok(game_process_return::Success::Deal)
}

fn push_card_to_hand(index: u8, game: &mut game::GameImpl) {
    game.players[index as usize].hand.push(game.deck.deal());
}

fn deal_two(cards: u8, game: &mut game::GameImpl) {
    for _card in 0..cards {
        for player in 0..2 {
            push_card_to_hand(player, game);
        }
    }
}

fn deal_three(game: &mut game::GameImpl) {
    for _card in 0..5 {
        for player in 0..3 {
            push_card_to_hand(player, game);
        }
    }

    game.crib.push(game.deck.deal());
}

fn deal_four(game: &mut game::GameImpl) {
    for _card in 0..5 {
        for player in 0..4 {
            push_card_to_hand(player, game);
        }
    }
}

fn deal_five(game: &mut game::GameImpl) {
    // With five players the dealer receives only four cards
    if let Some(index_dealer) = game.index_dealer {
        for card in 0..5 {
            for player in 0..5 {
                if card != 4 || player != index_dealer {
                    push_card_to_hand(player, game);
                }
            }
        }
    }
}

fn deal_six(game: &mut game::GameImpl) {
    // With six players the dealer and their partner receives only four cards
    if let Some(index_dealer) = game.index_dealer {
        if let Some(index_partner) = game.players[index_dealer as usize].partner_index {
            for card in 0..5 {
                for player in 0..6 {
                    if card != 4 || (player != index_dealer && player != index_partner) {
                        push_card_to_hand(player, game);
                    }
                }
            }
        }
    }
}
