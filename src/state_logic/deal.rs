use crate::game_process_return;

#[cfg(test)]
mod test {

    fn set_up_game(variant: crate::settings::RuleVariant) -> crate::GameImpl {
        let mut game = crate::GameImpl::new();
        game.is_debug = true;

        let vdo = {
            if crate::util::return_num_players_for_variant(variant) == 2 {
                crate::settings::VictorDealerOption::TwoPlayers
            } else if variant == crate::settings::RuleVariant::ThreeCaptain {
                crate::settings::VictorDealerOption::CaptainDeals
            } else {
                crate::settings::VictorDealerOption::LosersDrawForDealer
            }
        };

        let settings = crate::settings::GameSettings {
            variant: variant,
            victor_dealer_option: vdo,
            is_manual_scoring: false,
            is_underpegging: false,
            is_muggins: false,
            is_overpegging: false,
            is_lowball: false,
        };

        crate::state_logic::game_start::game_setup(&mut game, settings).unwrap();

        game.deck = crate::deck::Deck::new();

        crate::state_logic::cut_initial::process_cut(&mut game).unwrap();

        game.deck = crate::deck::Deck::new();

        game
    }

    #[test]
    fn process_deal_two_six_card() {
        let mut game = set_up_game(crate::settings::RuleVariant::TwoStandard);

        assert_eq!(
            super::process_deal(&mut game),
            Ok(super::game_process_return::Success::Deal)
        );

        assert_eq!(game.players[0].hand.len(), 6);
        assert_eq!(game.players[1].hand.len(), 6);
    }

    #[test]
    fn process_deal_two_five_card() {
        let mut game = set_up_game(crate::settings::RuleVariant::TwoFiveCard);

        assert_eq!(
            super::process_deal(&mut game),
            Ok(super::game_process_return::Success::Deal)
        );

        assert_eq!(game.players[0].hand.len(), 5);
        assert_eq!(game.players[1].hand.len(), 5);
    }

    #[test]
    fn process_deal_two_seven_card() {
        let mut game = set_up_game(crate::settings::RuleVariant::TwoSevenCard);

        assert_eq!(
            super::process_deal(&mut game),
            Ok(super::game_process_return::Success::Deal)
        );

        assert_eq!(game.players[0].hand.len(), 7);
        assert_eq!(game.players[1].hand.len(), 7);
        assert_eq!(game.crib.len(), 1);
    }

    #[test]
    fn process_deal_three_players() {
        let mut game = set_up_game(crate::settings::RuleVariant::ThreeStandard);

        assert_eq!(
            super::process_deal(&mut game),
            Ok(super::game_process_return::Success::Deal)
        );

        assert_eq!(game.players[0].hand.len(), 5);
        assert_eq!(game.players[1].hand.len(), 5);
        assert_eq!(game.players[2].hand.len(), 5);
        assert_eq!(game.crib.len(), 1);

        let mut game = set_up_game(crate::settings::RuleVariant::ThreeCaptain);

        assert_eq!(
            super::process_deal(&mut game),
            Ok(super::game_process_return::Success::Deal)
        );

        assert_eq!(game.players[0].hand.len(), 5);
        assert_eq!(game.players[1].hand.len(), 5);
        assert_eq!(game.players[2].hand.len(), 5);
        assert_eq!(game.crib.len(), 1);
    }

    #[test]
    fn process_deal_four_players() {
        let mut game = set_up_game(crate::settings::RuleVariant::FourIndividual);

        assert_eq!(
            super::process_deal(&mut game),
            Ok(super::game_process_return::Success::Deal)
        );

        assert_eq!(game.players[0].hand.len(), 5);
        assert_eq!(game.players[1].hand.len(), 5);
        assert_eq!(game.players[2].hand.len(), 5);
        assert_eq!(game.players[3].hand.len(), 5);

        let mut game = set_up_game(crate::settings::RuleVariant::FourPairs);

        assert_eq!(
            super::process_deal(&mut game),
            Ok(super::game_process_return::Success::Deal)
        );

        assert_eq!(game.players[0].hand.len(), 5);
        assert_eq!(game.players[1].hand.len(), 5);
        assert_eq!(game.players[2].hand.len(), 5);
        assert_eq!(game.players[3].hand.len(), 5);
    }

    #[test]
    fn process_deal_five_card() {
        // Dealer in the game returned to this function is index 0
        let mut game = set_up_game(crate::settings::RuleVariant::FiveStandard);

        println!("{:?}", game.index_dealer);

        assert_eq!(
            super::process_deal(&mut game),
            Ok(super::game_process_return::Success::Deal)
        );

        assert_eq!(game.players[0].hand.len(), 4);
        assert_eq!(game.players[1].hand.len(), 5);
        assert_eq!(game.players[2].hand.len(), 5);
        assert_eq!(game.players[3].hand.len(), 5);
        assert_eq!(game.players[4].hand.len(), 5);
    }

    #[test]
    fn process_deal_six_card() {
        // Dealer index is 0, and the dealer's partner index is 3
        let mut game = set_up_game(crate::settings::RuleVariant::SixPairs);

        assert_eq!(
            super::process_deal(&mut game),
            Ok(super::game_process_return::Success::Deal)
        );

        assert_eq!(game.players[0].hand.len(), 4);
        assert_eq!(game.players[1].hand.len(), 5);
        assert_eq!(game.players[2].hand.len(), 5);
        assert_eq!(game.players[3].hand.len(), 4);
        assert_eq!(game.players[4].hand.len(), 5);
        assert_eq!(game.players[5].hand.len(), 5);
    }
}

pub(crate) fn process_deal(
    game: &mut crate::GameImpl,
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
            crate::settings::RuleVariant::TwoStandard => deal_two(6, game),
            crate::settings::RuleVariant::TwoFiveCard => deal_two(5, game),
            crate::settings::RuleVariant::TwoSevenCard => deal_two(7, game),
            crate::settings::RuleVariant::ThreeStandard
            | crate::settings::RuleVariant::ThreeCaptain => deal_three(game),
            crate::settings::RuleVariant::FourIndividual
            | crate::settings::RuleVariant::FourPairs => deal_four(game),
            crate::settings::RuleVariant::FiveStandard => deal_five(game),
            crate::settings::RuleVariant::SixPairs => deal_six(game),
        };
    }

    game.state = crate::GameState::Sort;
    Ok(game_process_return::Success::Deal)
}

fn push_card_to_hand(index: u8, game: &mut crate::GameImpl) {
    game.players[index as usize].hand.push(game.deck.deal());
}

fn deal_two(cards: u8, game: &mut crate::GameImpl) {
    for _card in 0..cards {
        for player in 0..2 {
            push_card_to_hand(player, game);
        }
    }

    // In seven card cribbage, deal one card to the crib
    if cards == 7 {
        game.crib.push(game.deck.deal());
    }
}

fn deal_three(game: &mut crate::GameImpl) {
    for _card in 0..5 {
        for player in 0..3 {
            push_card_to_hand(player, game);
        }
    }

    game.crib.push(game.deck.deal());
}

fn deal_four(game: &mut crate::GameImpl) {
    for _card in 0..5 {
        for player in 0..4 {
            push_card_to_hand(player, game);
        }
    }
}

fn deal_five(game: &mut crate::GameImpl) {
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

fn deal_six(game: &mut crate::GameImpl) {
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
