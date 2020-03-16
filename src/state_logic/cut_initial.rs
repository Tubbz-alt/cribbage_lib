use crate as game;
use crate::deck;
use crate::game_process_return;
use crate::player;
use std::cmp;

#[cfg(test)]
mod test {
    // Returns a GameImpl with a valid configuration in debug mode with a given rule variant and
    // an initial_cut_between_number_number
    fn set_up_game(num_between: u8, variant: crate::settings::RuleVariant) -> crate::GameImpl {
        let mut game = crate::GameImpl::new();
        game.is_debug = true;

        let vdo = match variant {
            crate::settings::RuleVariant::TwoStandard => {
                crate::settings::VictorDealerOption::TwoPlayers
            }
            crate::settings::RuleVariant::TwoFiveCard => {
                crate::settings::VictorDealerOption::TwoPlayers
            }
            _ => crate::settings::VictorDealerOption::LosersDrawForDealer,
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

        if num_between > game.players.len() as u8 || num_between == 0 {
            panic!(
                "Invalid num_between in set_up_game() helper function for state_logic/cut_initial"
            );
        }

        let mut indices: Vec<u8> = Vec::new();

        for index in 0..num_between {
            indices.push(index);
        }

        game.initial_cut_between_players_with_these_indices = indices;

        game
    }

    #[test]
    fn process_cut_between_only_one() {
        let variants = crate::util::return_variants();

        for variant in variants {
            let mut game = set_up_game(1, variant);
            assert_eq!(
                super::process_cut(&mut game),
                Ok(super::game_process_return::Success::InitialCut(
                    super::game_process_return::InitialCutReturn::DealerChosen
                ))
            );
            assert_eq!(game.index_dealer, Some(0));
        }
    }

    #[test]
    fn process_cut_between_several_no_tie() {
        for variant in crate::util::return_variants() {
            for cut_between in 1..crate::util::return_num_players_for_variant(variant) + 1 {
                let mut game = set_up_game(cut_between, variant);
                // Creates a deck which will descend such that the last player to receive a card is
                // the victor of the cut
                let mut debug_deck = Vec::new();
                for card_value in 1..cut_between + 1 {
                    debug_deck.push(crate::util::return_card((card_value + 48) as char, 'H'));
                }

                game.deck = crate::deck::Deck::from_vec(debug_deck);

                assert_eq!(
                    super::process_cut(&mut game),
                    Ok(super::game_process_return::Success::InitialCut(
                        super::game_process_return::InitialCutReturn::DealerChosen
                    ))
                );

                assert_eq!(game.index_dealer, Some(cut_between - 1));
            }
        }
    }

    #[test]
    fn process_cut_between_several_with_tie() {
        for variant in crate::util::return_variants() {
            for cut_between in 2..crate::util::return_num_players_for_variant(variant) + 1 {
                // For each possible number of ties (2 with a 2 cut between; 2,3 with a 3 cut
                // between; and 2,3,4 with a 4 or more cut between)
                for tie_between in 2..super::cmp::min(5, cut_between + 1) {
                    let mut game = set_up_game(cut_between, variant);
                    let mut debug_deck = Vec::new();
                    // Push tie_between number of Aces to the deck such that the last tie_between
                    // players are the players who tie and continue to the next cut
                    for _ in 0..tie_between {
                        debug_deck.push(crate::util::return_card('A', 'H'));
                    }
                    // The remaining cards are 3,4,5... until there are cards for every player who
                    // the cut is between
                    for card_value in 2..(2 + cut_between - tie_between) {
                        debug_deck.push(crate::util::return_card((card_value + 48) as char, 'H'));
                    }

                    game.deck = crate::deck::Deck::from_vec(debug_deck);

                    // Assert that the cut was a tie
                    assert_eq!(
                        super::process_cut(&mut game),
                        Ok(super::game_process_return::Success::InitialCut(
                            super::game_process_return::InitialCutReturn::CutTie
                        ))
                    );

                    // Assert that the new value of the 'initial_cut_between_players_with_these_
                    // indices' are a vector of u8s of the last tie_between player indices
                    let mut expected_indices = Vec::new();
                    for index in (cut_between - tie_between)..cut_between {
                        expected_indices.push(index);
                    }
                    assert_eq!(
                        game.initial_cut_between_players_with_these_indices,
                        expected_indices
                    );

                    assert_eq!(game.state, crate::GameState::CutInitial);
                }
            }
        }
    }

    // Ensure that the loser of the cut in the TwoFiveCard variant receives three points
    #[test]
    fn process_cut_in_two_five_card_variant() {
        for index_victor in 0..2 {
            let mut index_loser = 0;
            if index_victor == 0 {
                index_loser = 1;
            }

            let mut game = set_up_game(2, crate::settings::RuleVariant::TwoFiveCard);

            // With a default deck, the first player will always win so setup a debug_deck if the
            // winner should be the player with the index 1
            if index_victor == 1 {
                game.deck = crate::deck::Deck::from_vec(vec![
                    crate::util::return_card('A', 'H'),
                    crate::util::return_card('2', 'H'),
                ]);
            }

            assert_eq!(
                super::process_cut(&mut game),
                Ok(super::game_process_return::Success::InitialCut(
                    super::game_process_return::InitialCutReturn::DealerChosen
                ))
            );

            for index in 0..2 {
                println!(
                    "Player {} cut card {:?}",
                    index, game.players[index].hand[0]
                );
            }

            assert_eq!(game.index_dealer, Some(index_victor));
            assert_eq!(game.players[index_loser].front_peg_pos, 3);
        }
    }

    #[test]
    fn process_cut_out_of_debug() {}
}

pub(crate) fn process_cut(
    game: &mut game::GameImpl,
) -> Result<game_process_return::Success, game_process_return::Error> {
    // Start with a shuffled deck by default or whatever the existing deck is when in debug mode
    if !game.is_debug {
        game.deck.reset_deck();
    }

    // If there is only one valid player (because they scored last when the victor_dealer_option is
    // LastPlaceIsDealer or they lost in a two player variant) then set them to be the dealer
    // without cutting
    if game.initial_cut_between_players_with_these_indices.len() == 1 {
        return Ok(set_dealer(
            game.initial_cut_between_players_with_these_indices[0],
            game,
        ));
    }

    deal_card_to_valid_players(game);
    game.initial_cut_between_players_with_these_indices = compare_cards(
        &game.players,
        &game.initial_cut_between_players_with_these_indices,
    );

    // Check number of players left after processing the cut too
    if game.initial_cut_between_players_with_these_indices.len() == 1 {
        Ok(set_dealer(
            game.initial_cut_between_players_with_these_indices[0],
            game,
        ))
    } else {
        Ok(game_process_return::Success::InitialCut(
            game_process_return::InitialCutReturn::CutTie,
        ))
    }
}

fn set_dealer(index: u8, game: &mut game::GameImpl) -> game_process_return::Success {
    game.index_dealer = Some(index);
    // If the rules variant is TwoFiveCard, then award the loser of the cut three points
    if let Some(settings) = game.settings {
        if settings.variant == crate::settings::RuleVariant::TwoFiveCard {
            if game.index_dealer == Some(0) {
                game.players[1].change_score(3);
            } else {
                game.players[0].change_score(3);
            }
        }
    }
    game.state = game::GameState::Deal;
    game_process_return::Success::InitialCut(game_process_return::InitialCutReturn::DealerChosen)
}

fn deal_card_to_valid_players(game: &mut game::GameImpl) {
    for (index, player) in game.players.iter_mut().enumerate() {
        player.hand.clear();

        // Deal a card to each player who the cut is between; this is everyone if coming from
        // GameSetup, but if there is a tie it'll be only between tied players and if it is not the
        // first game of the match it'll be either the player with the lowest score or between all
        // the losing players
        if game
            .initial_cut_between_players_with_these_indices
            .contains(&(index as u8))
        {
            player.hand.push(game.deck.deal());
        }
    }
}

// Takes an immutable reference to a  vector of players and a immutable reference to a Vec<u8> of
// player indices who the cut is between and returns a new Vec<u8> with the reduced set of indices
fn compare_cards(players: &Vec<player::Player>, indices: &Vec<u8>) -> Vec<u8> {
    // The highest value is 13/King so this function will work with any valid cards
    let mut lowest_value = 14;
    // At most four players will tie when the cut is between four or more players as there are four
    // cards of each suit, otherwise it'll be at most the number of players the cut is between
    let mut player_indices_with_cuts_of_lowest_values: Vec<u8> =
        Vec::with_capacity(cmp::min(4, indices.len()));

    for (index, player) in players.iter().enumerate() {
        if indices.contains(&(index as u8)) {
            // For every player who the cut is between
            let value = deck::return_value(player.hand[0]);
            // If their cut is the new lowest, make it known and set the vector tracking indices to
            // just the index of this player
            if value < lowest_value {
                lowest_value = value;
                player_indices_with_cuts_of_lowest_values = vec![index as u8];
            }
            // If their cut is equal to the lowest, add them to the vector tracking indices
            else if value == lowest_value {
                player_indices_with_cuts_of_lowest_values.push(index as u8);
            }
        }
    }

    player_indices_with_cuts_of_lowest_values
}
