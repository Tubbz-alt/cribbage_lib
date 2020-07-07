use crate::game_process_return;
use crate::score;
use crate::util;

// State that processes a vector of ScoreEvents for when manual scoring is enabled

#[cfg(test)]
mod test {
    use super::super::play_card;
    use super::super::test_util::set_up_game;
    use super::play_score;

    mod no_underpegging {
        use super::play_card;
        use super::play_score;
        use super::set_up_game;
        use crate::game_process_return;
        use crate::settings::RuleVariant;
        use crate::util;
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        // When the last played card does not reward any points and an empty ScoreEvents vector
        // is passed, then return an empty Ok(Success::PlayScore), make no changes to player
        // score, change the state to PlayWaitForCard, and change the index_active
        #[test]
        fn correctly_pegged_empty() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();

            assert_eq!(
                play_score(&mut game, vec![]),
                Ok(game_process_return::Success::PlayScore(vec![]))
            );

            assert_eq!(game.state, crate::GameState::PlayWaitForCard);

            assert_eq!(game.index_active, Some(0));

            assert_eq!(game.players[1].front_peg_pos, 0);
        }

        // When the last played card rewards points from one ScoreEvent and a ScoreEvents
        // vector with that one ScoreEvent is passed, then return an Ok(Success::PlayScore)
        // with that one ScoreEvent, make the appropriate change to player score, change the
        // state to PlayWaitForCard, and change the index_active
        #[test]
        fn correctly_pegged_one() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.players[1].hand = vec![util::return_card('T', 'H')];
            game.players[0].hand = vec![util::return_card('5', 'H')];

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_score(&mut game, vec![]).unwrap();
            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();

            let io = vec![crate::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: crate::score::ScoreType::Play(crate::score::PlayScoreType::Fifteen),
            }];

            assert_eq!(
                play_score(&mut game, io.clone()),
                Ok(game_process_return::Success::PlayScore(io))
            );

            assert_eq!(game.state, crate::GameState::PlayWaitForCard);

            assert_eq!(game.index_active, Some(1));

            assert_eq!(game.players[0].front_peg_pos, 2);
        }

        // Same as the last one, but return multipleScoreEvents in the Ok in the order that
        // they were passed
        #[test]
        fn correctly_pegged_multiple() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.players[1].hand = vec![util::return_card('4', 'H'), util::return_card('6', 'H')];
            game.players[0].hand = vec![util::return_card('5', 'H')];

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_score(&mut game, vec![]).unwrap();
            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_score(&mut game, vec![]).unwrap();

            play_card(&mut game, crate::PlayTurn::CardSelected(1)).unwrap();

            let mut io = vec![
                crate::score::ScoreEvent {
                    player_index: 1,
                    point_value: 2,
                    score_type: crate::score::ScoreType::Play(crate::score::PlayScoreType::Fifteen),
                },
                crate::score::ScoreEvent {
                    player_index: 1,
                    point_value: 3,
                    score_type: crate::score::ScoreType::Play(
                        crate::score::PlayScoreType::Straight(3),
                    ),
                },
            ];

            let mut rng = thread_rng();
            io.shuffle(&mut rng);

            assert_eq!(
                play_score(&mut game, io.clone()),
                Ok(game_process_return::Success::PlayScore(io))
            );

            assert_eq!(game.state, crate::GameState::PlayWaitForCard);

            assert_eq!(game.index_active, Some(0));

            assert_eq!(game.players[1].front_peg_pos, 5);
        }

        // When a ScoreEvent that is not a part of the optimal scoring of the move, return an
        // error, don't change state, don't change the index_active, and don't change the
        // player's score
        #[test]
        fn overpegged() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();

            let score = crate::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: crate::score::ScoreType::Play(crate::score::PlayScoreType::Pair),
            };

            assert_eq!(
                play_score(&mut game, vec![score.clone()]),
                Err(game_process_return::Error::PlayScoreError(
                    game_process_return::PlayScoreError::OverpeggingIsNotAllowed(score)
                ))
            );
            assert_eq!(game.state, crate::GameState::PlayScore);
            assert_eq!(game.index_active, Some(1));
            assert_eq!(game.players[1].front_peg_pos, 0);
        }

        // When a ScoreEvent that is a part of the optimal scoring of the move is not included
        // with the passed ScoreEvents vector and underpegging is disabled, return an error,
        // don't change state, don't change the index_active, and don't change the player's
        // score
        #[test]
        fn underpegged() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.players[1].hand = vec![util::return_card('5', 'H')];
            game.players[0].hand = vec![util::return_card('T', 'H')];

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();
            play_score(&mut game, vec![]).unwrap();
            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();

            assert_eq!(
                play_score(&mut game, vec![]),
                Err(game_process_return::Error::PlayScoreError(
                    game_process_return::PlayScoreError::UnderpeggingIsNotAllowed
                ))
            );

            assert_eq!(game.state, crate::GameState::PlayScore);
            assert_eq!(game.index_active, Some(0));
            assert_eq!(game.players[0].front_peg_pos, 0);
        }

        // When the PlayTurn is Go and it is not the last card, accept an empty vector, do not
        // change any point values, change the state to ResetPlay, and change the index_active
        #[test]
        fn go_no_last_card() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.play_groups[0].total = 30;

            play_card(&mut game, crate::PlayTurn::Go).unwrap();

            assert_eq!(
                play_score(&mut game, vec![]),
                Ok(game_process_return::Success::PlayScore(vec![]))
            );
            assert_eq!(game.players[1].front_peg_pos, 0);
            assert_eq!(game.state, crate::GameState::PlayWaitForCard);
            assert_eq!(game.index_active, Some(0));
        }

        // When the PlayTurn is Go and it is not the last card, any passed ScoreEvent is
        // overpeggin, so return the relevant error, keep the state at PlayScore, don't change
        // the index_active, and don't change any point values
        #[test]
        fn go_no_last_card_overpegged() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.play_groups[0].total = 30;

            play_card(&mut game, crate::PlayTurn::Go).unwrap();

            let score_event = crate::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: crate::score::ScoreType::Play(crate::score::PlayScoreType::Fifteen),
            };

            assert_eq!(
                play_score(&mut game, vec![score_event.clone()]),
                Err(game_process_return::Error::PlayScoreError(
                    game_process_return::PlayScoreError::OverpeggingIsNotAllowed(score_event)
                ))
            );
            assert_eq!(game.players[1].front_peg_pos, 0);
            assert_eq!(game.state, crate::GameState::PlayScore);
            assert_eq!(game.index_active, Some(1));
        }

        // When the PlayTurn is Go and it is the last card, but a LastCard ScoreEvent is not
        // passed while underpegging is disabled, return the relevant error, keep the state at
        // PlayScore, don't change the index_active, and don't change any point values
        #[test]
        fn go_last_card_underpegged() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.play_groups[0].total = 30;
            game.players[0].hand = vec![util::return_card('2', 'C')];
            game.last_player_index = Some(0);

            play_card(&mut game, crate::PlayTurn::Go).unwrap();
            play_score(&mut game, vec![]).unwrap();
            play_card(&mut game, crate::PlayTurn::Go).unwrap();
            assert_eq!(
                play_score(&mut game, vec![]),
                Err(game_process_return::Error::PlayScoreError(
                    game_process_return::PlayScoreError::UnderpeggingIsNotAllowed
                ))
            );
            assert_eq!(game.state, crate::GameState::PlayScore);
            assert_eq!(game.index_active, Some(0));
            assert_eq!(game.players[0].front_peg_pos, 0);
        }

        // When the PlayTurn is Go, it is the last card, and a LastCard ScoreEvent is passed,
        // return the relevant Success, change the state to ResetPlay, update the scores, and
        // change the index_active
        #[test]
        fn go_last_card() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.play_groups[0].total = 30;
            game.players[0].hand = vec![util::return_card('2', 'C')];
            game.last_player_index = Some(0);

            play_card(&mut game, crate::PlayTurn::Go).unwrap();
            play_score(&mut game, vec![]).unwrap();
            play_card(&mut game, crate::PlayTurn::Go).unwrap();

            let score_event = crate::score::ScoreEvent {
                point_value: 1,
                player_index: 0,
                score_type: crate::score::ScoreType::Play(crate::score::PlayScoreType::LastCard),
            };

            assert_eq!(
                play_score(&mut game, vec![score_event.clone()]),
                Ok(game_process_return::Success::PlayScore(vec![score_event]))
            );
            assert_eq!(game.state, crate::GameState::ResetPlay);
            assert_eq!(game.index_active, Some(1));
            assert_eq!(game.players[0].front_peg_pos, 1);
        }

        // If the PlayTurn is playing a card, the total of the PlayGroup now equals 31, and a
        // ThirtyOne ScoreEvent is not passed to play_score while underpegging is disabled,
        // return the relevant Error, keep the state at PlayScore, do not change scores, and
        // do not change the index_active
        #[test]
        fn thirty_one_underpegged() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.play_groups[0].total = 30;
            game.players[1].hand = vec![util::return_card('A', 'H')];

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();

            assert_eq!(
                play_score(&mut game, vec![]),
                Err(game_process_return::Error::PlayScoreError(
                    game_process_return::PlayScoreError::UnderpeggingIsNotAllowed
                ))
            );
            assert_eq!(game.state, crate::GameState::PlayScore);
            assert_eq!(game.index_active, Some(1));
            assert_eq!(game.players[1].front_peg_pos, 0);
        }

        #[test]
        fn thirty_one_overpegged() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.play_groups[0].total = 30;
            game.players[1].hand = vec![util::return_card('A', 'H')];

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();

            let score_events = vec![
                crate::score::ScoreEvent {
                    player_index: 1,
                    point_value: 2,
                    score_type: crate::score::ScoreType::Play(
                        crate::score::PlayScoreType::ThirtyOne,
                    ),
                },
                crate::score::ScoreEvent {
                    player_index: 1,
                    point_value: 2,
                    score_type: crate::score::ScoreType::Play(crate::score::PlayScoreType::Fifteen),
                },
            ];

            assert_eq!(
                play_score(&mut game, score_events.clone()),
                Err(game_process_return::Error::PlayScoreError(
                    game_process_return::PlayScoreError::OverpeggingIsNotAllowed(
                        score_events[1].clone()
                    )
                ))
            );
            assert_eq!(game.state, crate::GameState::PlayScore);
            assert_eq!(game.index_active, Some(1));
            assert_eq!(game.players[1].front_peg_pos, 0);
        }

        #[test]
        fn thirty_one() {
            let mut game = set_up_game(RuleVariant::TwoStandard, true, false, false);

            game.play_groups[0].total = 30;
            game.players[1].hand = vec![util::return_card('A', 'H')];

            play_card(&mut game, crate::PlayTurn::CardSelected(0)).unwrap();

            let score_event = crate::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: crate::score::ScoreType::Play(crate::score::PlayScoreType::ThirtyOne),
            };

            assert_eq!(
                play_score(&mut game, vec![score_event.clone()]),
                Ok(game_process_return::Success::PlayScore(vec![score_event]))
            );
            assert_eq!(game.state, crate::GameState::ResetPlay);
            assert_eq!(game.index_active, Some(0));
            assert_eq!(game.players[1].front_peg_pos, 2);
        }
    }

    mod underpegging {
        mod no_muggins {
            use super::super::play_card;
            use super::super::set_up_game;
            use crate::game_process_return;
            use crate::util;

            #[test]
            fn no_valid_correctly_pegged() {}

            #[test]
            fn one_valid_underpegged() {}

            #[test]
            fn multiple_valid_completely_underpegged() {}

            #[test]
            fn multiple_valid_partialy_underpegged() {}

            #[test]
            fn go_last_card_underpegged() {}

            #[test]
            fn thirty_one_underpegged() {}
        }
        mod muggins {
            use super::super::play_card;
            use super::super::set_up_game;
            use crate::game_process_return;
            use crate::util;

            #[test]
            fn no_valid_correctly_pegged() {}

            #[test]
            fn one_valid_underpegged() {}

            #[test]
            fn one_valid_correctly_pegged() {}

            #[test]
            fn multiple_valid_completely_underpegged() {}

            #[test]
            fn multiple_valid_partialy_underpegged() {}

            #[test]
            fn multiple_valid_correctly_pegged() {}

            #[test]
            fn go_last_card_underpegged() {}

            #[test]
            fn go_last_card_correctly_pegged() {}

            #[test]
            fn thirty_one_underpegged() {}

            #[test]
            fn thirty_one_correctly_pegged() {}
        }
    }
}

pub(crate) fn play_score(
    game: &mut crate::GameImpl,
    selections: Vec<score::ScoreEvent>,
) -> Result<game_process_return::Success, game_process_return::Error> {
    // If the index_active is the last_player_index, that means the player had just played a card
    // in the previous PlayWaitForCard state or that the index_active has looped around to the
    // last_player_index
    if game.index_active == game.last_player_index {
        let mut optimal_scoring = score::play::play_score(
            game.index_active.unwrap(),
            &game.play_groups.last().unwrap(),
        );

        // If the last play was a go, add the LastCard ScoreEvent to the optimal scoring
        if game.last_play_was_go {
            optimal_scoring.push(score::ScoreEvent {
                point_value: 1,
                player_index: game.index_active.unwrap(),
                score_type: score::ScoreType::Play(score::PlayScoreType::LastCard),
            });
        }

        let mut score_total = 0;

        for selection in &selections {
            let mut is_in_optimal = false;

            for score in &optimal_scoring {
                if *selection == *score {
                    is_in_optimal = true;
                }
            }

            //TODO Remove that clone somehow
            if !is_in_optimal {
                return Err(game_process_return::Error::PlayScoreError(
                    game_process_return::PlayScoreError::OverpeggingIsNotAllowed(selection.clone()),
                ));
            }

            score_total += selection.point_value;
        }

        for score in &optimal_scoring {
            let mut is_in_selections = false;

            for selection in &selections {
                if *selection == *score {
                    is_in_selections = true;
                }
            }

            if !is_in_selections {
                return Err(game_process_return::Error::PlayScoreError(
                    game_process_return::PlayScoreError::UnderpeggingIsNotAllowed,
                ));
            }
        }

        util::process_score(game, game.index_active.unwrap() as usize, score_total);

        if !game.last_play_was_go && game.play_groups.last().unwrap().total != 31 {
            game.state = crate::GameState::PlayWaitForCard;
        } else {
            game.state = crate::GameState::ResetPlay;
        }

        game.index_active = Some((game.index_active.unwrap() + 1) % game.players.len() as u8);

        Ok(game_process_return::Success::PlayScore(selections))
    }
    // If the player scoring sent a Go in the last PlayWaitForCard and it is not the last card
    else {
        if selections.len() > 0 {
            Err(game_process_return::Error::PlayScoreError(
                game_process_return::PlayScoreError::OverpeggingIsNotAllowed(selections[0].clone()),
            ))
        } else {
            game.state = crate::GameState::PlayWaitForCard;
            game.index_active = Some((game.index_active.unwrap() + 1) % game.players.len() as u8);

            Ok(game_process_return::Success::PlayScore(vec![]))
        }
    }
}
