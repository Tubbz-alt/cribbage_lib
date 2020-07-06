use crate::game_process_return;
use crate::score;

#[cfg(test)]
mod test {
    // Helper function to create a GameImpl in the PlayWaitForCard stage of the game
    fn set_up_game(
        variant: crate::settings::RuleVariant,
        is_man: bool,
        is_under: bool,
        is_mug: bool,
    ) -> crate::GameImpl {
        let vdo = {
            if crate::util::return_num_players_for_variant(variant) == 2 {
                crate::settings::VictorDealerOption::TwoPlayers
            } else if variant == crate::settings::RuleVariant::ThreeCaptain {
                crate::settings::VictorDealerOption::CaptainDeals
            } else {
                crate::settings::VictorDealerOption::LosersDrawForDealer
            }
        };
        let mut game = crate::GameImpl::new();
        let settings = crate::settings::GameSettings {
            variant: variant,
            victor_dealer_option: vdo,
            is_manual_scoring: is_man,
            is_underpegging: is_under,
            is_muggins: is_mug,
            is_lowball: false,
        };

        game.is_debug = true;

        crate::state_logic::game_start::game_setup(&mut game, settings).unwrap();

        game.deck = crate::deck::Deck::new();

        crate::state_logic::cut_initial::process_cut(&mut game).unwrap();

        game.deck = crate::deck::Deck::new();

        crate::state_logic::deal::process_deal(&mut game).unwrap();

        crate::state_logic::sort::process_sort(&mut game).unwrap();

        let discard_selecions: Vec<Vec<u8>> =
            match crate::util::return_num_players_for_variant(variant) {
                2 => vec![vec![0, 1], vec![0, 1]],
                3 => vec![vec![0], vec![0], vec![0]],
                4 => vec![vec![0], vec![0], vec![0], vec![0]],
                5 => vec![vec![], vec![0], vec![0], vec![0], vec![0]],
                6 => vec![vec![], vec![0], vec![0], vec![], vec![0], vec![0]],
                _ => panic!(
                "return_num_players_for_variant returned a number not between 2 and 6 inclusive"
                ),
            };
        crate::state_logic::discard::process_discard(&mut game, discard_selecions).unwrap();

        crate::cut_starter_and_nibs_check::process_cut(&mut game).unwrap();

        game
    }

    // State that handles a PlayTurn object (enum with either the card played or a go); must check
    // that the play is valid, adds the card to the last member of the play_groups variable (what
    // I'm calling the Vec<Vec<Cards>> that keeps track of every card played in groups that have a
    // maximum value of 31), and handles scoring with automatic scoring
    mod play_wait_for_card {
        use super::super::play_card;
        use super::set_up_game;
        use crate::game_process_return;
        use crate::util::return_card;

        #[test]
        // Return and error when the index given is not between 0 and 2 inclusive
        fn test_play_turn_index_input_with_five_card() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoFiveCard,
                false,
                false,
                false,
            );

            // Returns expected Err
            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(3),),
                Err(game_process_return::Error::PlayWaitForCardError(
                    game_process_return::PlayTurnError::IndexIsBetween0And2InclusiveWithTwoFiveCard
                ))
            );

            // Otherwise returns Ok with an empty ScoreEvent vector in an AutomaticScoring return
            // (because the first card played will never score any points)
            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );
        }

        // Between 0 and 4 inclusive with seven card
        #[test]
        fn test_play_turn_index_input_with_seven_card() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoSevenCard,
                false,
                false,
                false,
            );

            // Returns expected Err
            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(5),),
                Err(game_process_return::Error::PlayWaitForCardError(
                    game_process_return::PlayTurnError::IndexIsBetween0And4InclusiveWithTwoSevenCard
                ))
            );

            // Otherwise returns Ok
            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );
        }

        // Between 0 and 3 inclusive with every other rule variant
        #[test]
        fn test_play_turn_index_input_with_standard() {
            for variant in crate::util::return_variants() {
                if variant != crate::settings::RuleVariant::TwoFiveCard
                    && variant != crate::settings::RuleVariant::TwoSevenCard
                {
                    let mut game = set_up_game(variant, false, false, false);

                    // Returns expected Err
                    assert_eq!(
                        play_card(&mut game, crate::PlayTurn::CardSelected(4)),
                        Err(game_process_return::Error::PlayWaitForCardError(
                            game_process_return::PlayTurnError::IndexIsBetween0And3InclusiveWithThisRuleVariant
                        ))
                    );

                    // Otherwise returns Ok
                    assert_eq!(
                        play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                        Ok(game_process_return::Success::PlayWaitForCard(
                            game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                        ))
                    );
                }
            }
        }

        // Test that the given card index is actually added to the active PlayGroup
        #[test]
        fn test_play_turn_insertion() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );

            assert_eq!(game.play_groups[0].cards[0], game.players[1].hand[0]);
            assert_eq!(
                game.play_groups[0].total,
                crate::deck::return_play_value(game.players[1].hand[0])
            );
            assert_eq!(game.index_active, Some(0));
        }

        #[test]
        fn test_multiple_insertions() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );
            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();

            let expected = crate::PlayGroup {
                total: crate::deck::return_play_value(game.players[1].hand[0])
                    + crate::deck::return_play_value(game.players[0].hand[0]),
                cards: vec![game.players[1].hand[0], game.players[0].hand[0]],
            };
            assert_eq!(game.play_groups.last().unwrap(), &expected);
            assert_eq!(game.index_active, Some(1));
        }

        // Test that a given card has not already been added to the active PlayGroup
        #[test]
        fn test_repeated_play_turn_insertion() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Err(game_process_return::Error::PlayWaitForCardError(
                    game_process_return::PlayTurnError::CardHasAlreadyBeenPlayed
                ))
            );
        }

        // Test that one can not play a card that brings the PlayGroup total over 31
        #[test]
        fn test_maximum_play_group_total() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );

            game.play_groups[0].total = 31;

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Err(game_process_return::Error::PlayWaitForCardError(
                    game_process_return::PlayTurnError::PlayGroupTotalMayNotExceed31
                ))
            );
        }

        // Test that one can not go when they have a card that they could put down
        #[test]
        fn test_invalid_go() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::Go),
                Err(game_process_return::Error::PlayWaitForCardError(
                    game_process_return::PlayTurnError::MustPlayCardIfAble
                ))
            );
        }

        // Test that play_card accepts a Go when the active player has no cards that wouldn't bring
        // the total over 31
        #[test]
        fn test_valid_go() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );

            game.play_groups[0].total = 31;

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::Go),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );
        }

        // Test that the last_player_index of the GameImpl is set during play as it is used for
        // scoring last card
        #[test]
        fn test_set_last_player_index() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );

            assert_eq!(game.last_player_index, None);
            assert_eq!(game.index_active, Some(1));

            game.players[1].hand = vec![return_card('T', 'H'), return_card('K', 'H')];
            game.players[0].hand = vec![return_card('J', 'H'), return_card('2', 'H')];

            // Player 1 goes first because player 0 is dealer
            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );
            assert_eq!(game.last_player_index, Some(1));
            assert_eq!(game.index_active, Some(0));

            // Then player 0
            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );
            assert_eq!(game.last_player_index, Some(0));
            assert_eq!(game.index_active, Some(1));

            // And player 1 again
            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(1)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );
            assert_eq!(game.last_player_index, Some(1));
            assert_eq!(game.index_active, Some(0));

            // And finally player 0 has to go
            assert_eq!(
                play_card(&mut game, crate::PlayTurn::Go),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );
            assert_eq!(game.last_player_index, Some(1));
            assert_eq!(game.index_active, Some(1));

            let mut game = set_up_game(crate::settings::RuleVariant::SixPairs, false, false, false);

            assert_eq!(game.last_player_index, None);
            assert_eq!(game.index_active, Some(1));

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );
            assert_eq!(game.last_player_index, Some(1));
            assert_eq!(game.index_active, Some(2));

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );
            assert_eq!(game.last_player_index, Some(2));
            assert_eq!(game.index_active, Some(3));
        }

        // Test that it processes score and returns correctly with automatic scoring
        #[test]
        fn test_auto_scoring() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );

            game.players[1].hand[0] = return_card('4', 'H');
            game.players[0].hand[0] = return_card('5', 'H');
            game.players[1].hand[1] = return_card('6', 'H');

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(1)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![
                        crate::score::ScoreEvent {
                            player_index: 1,
                            point_value: 2,
                            score_type: crate::score::ScoreType::Play(
                                crate::score::PlayScoreType::Fifteen
                            ),
                        },
                        crate::score::ScoreEvent {
                            player_index: 1,
                            point_value: 3,
                            score_type: crate::score::ScoreType::Play(
                                crate::score::PlayScoreType::Straight(3)
                            ),
                        },
                    ])
                ))
            );

            assert_eq!(game.players[1].front_peg_pos, 5);
        }

        // Test that it awards a point for last card and sets the state to ResetPlay when automatic
        // scoring is enabled
        #[test]
        fn test_last_card() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );

            game.players[1].hand = vec![return_card('T', 'H'), return_card('K', 'H')];
            game.players[0].hand = vec![return_card('J', 'H'), return_card('2', 'H')];

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_card(&mut game, crate::PlayTurn::CardSelected(1)).unwrap();

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::Go),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![])
                ))
            );

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::Go),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![
                        crate::score::ScoreEvent {
                            player_index: 1,
                            point_value: 1,
                            score_type: crate::score::ScoreType::Play(
                                crate::score::PlayScoreType::LastCard
                            )
                        }
                    ])
                ))
            );

            assert_eq!(game.state, crate::GameState::ResetPlay);

            assert_eq!(game.players[1].front_peg_pos, 1);
        }

        // Test that it awards no extra points (you get two from the scoring function already) for
        // a value of 31 and that the state is set to ResetPlay with automatic scoring
        #[test]
        fn test_thirty_one() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                false,
                false,
                false,
            );

            game.players[1].hand = vec![return_card('T', 'H'), return_card('K', 'H')];
            game.players[0].hand = vec![return_card('J', 'H'), return_card('A', 'H')];

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_card(&mut game, crate::PlayTurn::CardSelected(1)).unwrap();

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(1)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![
                        crate::score::ScoreEvent {
                            player_index: 0,
                            point_value: 2,
                            score_type: crate::score::ScoreType::Play(
                                crate::score::PlayScoreType::ThirtyOne
                            )
                        }
                    ])
                ))
            );

            assert_eq!(game.state, crate::GameState::ResetPlay);

            assert_eq!(game.players[0].front_peg_pos, 2);
        }

        // Test that it returns correctly and sets the state to PlayScore when manual scoring is
        // enabled
        #[test]
        fn test_manual_scoring() {
            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                true,
                false,
                false,
            );

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::CardSelected(0)),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::ManualScoring
                ))
            );
            assert_eq!(game.state, crate::GameState::PlayScore);

            let mut game = set_up_game(
                crate::settings::RuleVariant::TwoStandard,
                true,
                false,
                false,
            );

            game.play_groups[0].total = 30;

            assert_eq!(
                play_card(&mut game, crate::PlayTurn::Go),
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::ManualScoring
                ))
            );
            assert_eq!(game.state, crate::GameState::PlayScore);
        }

        // TODO Test how it all works after it goes through the play phase of the game more than
        // once
    }

    // State that processes a ScoreEvent for when manual scoring is enabled
    mod play_score {}

    // State that deals with any missed ScoreEvents when muggings is enabled
    mod play_muggins {}

    // State that will either add a new, empty member to play_groups and go back to PlayWaitForCard
    // or transition the game to the ShowScore state
    mod reset_play {}
}

pub(crate) fn play_card(
    game: &mut crate::GameImpl,
    play_turn: crate::PlayTurn,
) -> Result<game_process_return::Success, game_process_return::Error> {
    // When the PlayTurn is CardSelected, ensure that the given index is valid for that variant
    if let crate::PlayTurn::CardSelected(index) = play_turn {
        match game.settings.unwrap().variant {
            crate::settings::RuleVariant::TwoFiveCard => {
                if index > 2 {
                    return Err(game_process_return::Error::PlayWaitForCardError(
                        game_process_return::PlayTurnError::IndexIsBetween0And2InclusiveWithTwoFiveCard
                    ));
                }
            }
            crate::settings::RuleVariant::TwoSevenCard => {
                if index > 4 {
                    return Err(game_process_return::Error::PlayWaitForCardError(
                        game_process_return::PlayTurnError::IndexIsBetween0And4InclusiveWithTwoSevenCard
                    ));
                }
            }
            _ => {
                if index > 3 {
                    return Err(game_process_return::Error::PlayWaitForCardError(
                        game_process_return::PlayTurnError::IndexIsBetween0And3InclusiveWithThisRuleVariant
                    ));
                }
            }
        };

        if has_card_been_played(game, index) {
            return Err(game_process_return::Error::PlayWaitForCardError(
                game_process_return::PlayTurnError::CardHasAlreadyBeenPlayed,
            ));
        }

        let card = game.players[game.index_active.unwrap() as usize].hand[index as usize];

        // Return an error if the card would bring the play total over 31
        if game.play_groups.last().unwrap().total + crate::deck::return_play_value(card) > 31 {
            return Err(game_process_return::Error::PlayWaitForCardError(
                game_process_return::PlayTurnError::PlayGroupTotalMayNotExceed31,
            ));
        }

        // Update the last PlayGroup to include the given card index and add the value to the
        // PlayGroup total
        game.play_groups.last_mut().unwrap().cards.push(card);
        game.play_groups.last_mut().unwrap().total += crate::deck::return_play_value(card);
        game.last_player_index = game.index_active;

        if game.settings.unwrap().is_manual_scoring {
            game.state = crate::GameState::PlayScore;

            Ok(game_process_return::Success::PlayWaitForCard(
                game_process_return::PlayWaitForCardReturn::ManualScoring,
            ))
        } else {
            let index_active = game.index_active.unwrap();

            // Caluclate ScoreEvents
            let scoring_vec =
                score::play::play_score(index_active, game.play_groups.last().unwrap());

            // Calculate total score change
            let mut score_change = 0;
            for score_event in &scoring_vec {
                score_change += score_event.point_value;
            }

            // Process score change
            crate::util::process_score(game, index_active as usize, score_change);

            // Change index_active to next player
            game.index_active = Some((game.index_active.unwrap() + 1) % game.players.len() as u8);

            // Change state to ResetPlay when the PlayGroup total is 31
            if game.play_groups.last().unwrap().total == 31 {
                game.state = crate::GameState::ResetPlay;
            }

            // Return
            Ok(game_process_return::Success::PlayWaitForCard(
                game_process_return::PlayWaitForCardReturn::AutomaticScoring(scoring_vec),
            ))
        }
    }
    // When the PlayTurn is Go
    else {
        // For every card in the player's hand that has not already been played
        for (index, card) in game.players[game.index_active.unwrap() as usize]
            .hand
            .iter()
            .enumerate()
        {
            // Return an error if the player can play a card, but has sent a Go
            if !has_card_been_played(game, index as u8) {
                if game.play_groups.last().unwrap().total + crate::deck::return_play_value(*card)
                    <= 31
                {
                    return Err(game_process_return::Error::PlayWaitForCardError(
                        game_process_return::PlayTurnError::MustPlayCardIfAble,
                    ));
                }
            }
        }

        // Send game to PlayScore with manual scoring
        if game.settings.unwrap().is_manual_scoring {
            game.state = crate::GameState::PlayScore;
            Ok(game_process_return::Success::PlayWaitForCard(
                game_process_return::PlayWaitForCardReturn::ManualScoring,
            ))
        } else {
            // With automatic scoring, check if everyone has gone and if the player represented by
            // the last_player_index should receive a point
            let last_card = game.index_active == game.last_player_index;
            game.index_active = Some((game.index_active.unwrap() + 1) % game.players.len() as u8);
            if last_card {
                game.state = crate::GameState::ResetPlay;
                crate::util::process_score(game, game.last_player_index.unwrap() as usize, 1);
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![
                        score::ScoreEvent {
                            player_index: game.last_player_index.unwrap(),
                            point_value: 1,
                            score_type: score::ScoreType::Play(score::PlayScoreType::LastCard),
                        },
                    ]),
                ))
            } else {
                Ok(game_process_return::Success::PlayWaitForCard(
                    game_process_return::PlayWaitForCardReturn::AutomaticScoring(vec![]),
                ))
            }
        }
    }
}

fn has_card_been_played(game: &crate::GameImpl, index: u8) -> bool {
    let card_play_turn = game.players[game.index_active.unwrap() as usize].hand[index as usize];

    for play_group in &game.play_groups {
        for card in &play_group.cards {
            if card_play_turn == *card {
                return true;
            }
        }
    }

    false
}
