use crate::game_process_return;

#[cfg(test)]
mod test {
    // Cut should work the same regardless of the RuleVariant
    fn set_up_game(
        is_partner: bool,
        is_man: bool,
        is_under: bool,
        is_over: bool,
        is_low: bool,
    ) -> crate::GameImpl {
        let mut game = crate::GameImpl::new();
        game.is_debug = true;

        let settings = crate::settings::GameSettings {
            variant: crate::settings::RuleVariant::TwoStandard,
            victor_dealer_option: crate::settings::VictorDealerOption::TwoPlayers,
            is_manual_scoring: is_man,
            is_underpegging: is_under,
            is_muggins: if is_under && is_low { true } else { false },
            is_overpegging: is_over,
            is_lowball: is_low,
        };

        crate::state_logic::game_start::game_setup(&mut game, settings).unwrap();

        // This is an invalid config, but this will only be used to test that the partner's score
        // advances with the dealer who scores nibs
        if is_partner {
            game.players[0].partner_index = Some(1);
            game.players[1].partner_index = Some(0);
        }

        game.deck = crate::deck::Deck::new();

        crate::state_logic::cut_initial::process_cut(&mut game).unwrap();

        game.deck = crate::deck::Deck::new();

        crate::state_logic::deal::process_deal(&mut game).unwrap();

        crate::state_logic::sort::process_sort(&mut game).unwrap();

        let discard_selecions: Vec<Vec<u8>> = vec![vec![0, 1], vec![0, 1]];
        crate::state_logic::discard::process_discard(&mut game, discard_selecions).unwrap();

        game
    }

    mod automatic_scoring {
        #[test]
        fn process_cut_without_underpegging_no_nibs() {
            let mut game = super::set_up_game(false, false, false, false, false);
            assert_eq!(
                super::super::process_cut(&mut game),
                Ok(crate::game_process_return::Success::StarterCut)
            );
            // AH to QH are dealt to the players with the debug deck
            assert_eq!(game.starter_card, Some(crate::util::return_card('K', 'H')));
            assert_eq!(game.state, crate::GameState::PlayWaitForCard);
        }

        #[test]
        fn process_cut_without_underpegging_nibs_no_partner() {
            let mut game = super::set_up_game(false, false, false, false, false);
            // Set the deck to contain only the Jack of hearts
            game.deck = crate::deck::Deck::from_vec(vec![crate::util::return_card('J', 'H')]);

            assert_eq!(
                super::super::process_cut(&mut game),
                Ok(crate::game_process_return::Success::StarterCut)
            );
            assert_eq!(game.starter_card, Some(crate::util::return_card('J', 'H')));
            assert_eq!(game.players[0].front_peg_pos, 2);
            assert_eq!(game.state, crate::GameState::PlayWaitForCard);
        }

        #[test]
        fn process_cut_without_underpegging_nibs_and_win_no_partner() {
            let mut game = super::set_up_game(false, false, false, false, false);
            game.deck = crate::deck::Deck::from_vec(vec![crate::util::return_card('J', 'H')]);
            game.players[0].change_score(120);

            assert_eq!(
                super::super::process_cut(&mut game),
                Ok(crate::game_process_return::Success::StarterCut)
            );
            assert_eq!(game.starter_card, Some(crate::util::return_card('J', 'H')));
            assert_eq!(game.players[0].front_peg_pos, 122);
            assert_eq!(game.state, crate::GameState::Win);
        }

        #[test]
        fn process_cut_without_underpegging_nibs_with_partner() {
            let mut game = super::set_up_game(true, false, false, false, false);
            game.deck = crate::deck::Deck::from_vec(vec![crate::util::return_card('J', 'H')]);

            assert_eq!(
                super::super::process_cut(&mut game),
                Ok(crate::game_process_return::Success::StarterCut)
            );
            assert_eq!(game.starter_card, Some(crate::util::return_card('J', 'H')));
            assert_eq!(game.players[0].front_peg_pos, 2);
            assert_eq!(game.players[1].front_peg_pos, 2);
            assert_eq!(game.state, crate::GameState::PlayWaitForCard);
        }

        #[test]
        fn process_cut_without_underpegging_nibs_and_win_with_partner() {
            let mut game = super::set_up_game(true, false, false, false, false);
            game.deck = crate::deck::Deck::from_vec(vec![crate::util::return_card('J', 'H')]);
            game.players[0].change_score(120);
            game.players[1].change_score(120);
            assert_eq!(
                super::super::process_cut(&mut game),
                Ok(crate::game_process_return::Success::StarterCut)
            );
            assert_eq!(game.starter_card, Some(crate::util::return_card('J', 'H')));
            assert_eq!(game.players[0].front_peg_pos, 122);
            assert_eq!(game.players[1].front_peg_pos, 122);
            assert_eq!(game.state, crate::GameState::Win);
        }
    }

    mod manual_scoring {
        #[test]
        fn process_cut_with_underpegging() {
            let mut game = super::set_up_game(false, true, false, false, false);
            assert_eq!(
                super::super::process_cut(&mut game),
                Ok(crate::game_process_return::Success::StarterCut)
            );
            assert_eq!(game.starter_card, Some(crate::util::return_card('K', 'H')));
            assert_eq!(game.state, crate::GameState::NibsCheck);
        }

        mod lowball {}

        mod notlowball {

            mod no_overpegging {
                use super::super::super::super::{process_cut, process_nibs};
                use super::super::super::set_up_game;

                #[test]
                fn process_nibs_no_nibs() {
                    let mut game = set_up_game(false, true, false, false, false);
                    process_cut(&mut game).unwrap();
                    assert_eq!(
                        process_nibs(&mut game, None),
                        Ok(crate::game_process_return::Success::NibsCheck)
                    );
                    assert_eq!(game.state, crate::GameState::PlayWaitForCard);
                }

                #[test]
                fn process_nibs_nibs_but_not_claimed_underpegging_disabled() {
                    let mut game = set_up_game(false, true, false, false, false);
                    process_cut(&mut game).unwrap();
                    game.starter_card = Some(crate::util::return_card('J', 'H'));
                    assert_eq!(
                    process_nibs(&mut game, None),
                    Err(crate::game_process_return::Error::NibsCallError(
                        crate::game_process_return::NibsError::NoNibsCallWhenUnderscoringIsDisabled
                    ))
                );
                    assert_eq!(game.state, crate::GameState::NibsCheck);
                }

                #[test]
                fn process_nibs_nibs_but_not_claimed_underpegging_enabled() {
                    let mut game = set_up_game(false, true, true, false, false);
                    process_cut(&mut game).unwrap();
                    game.starter_card = Some(crate::util::return_card('J', 'H'));
                    assert_eq!(
                        process_nibs(&mut game, None),
                        Ok(crate::game_process_return::Success::NibsCheck)
                    );
                    assert_eq!(game.players[0].front_peg_pos, 0);
                    assert_eq!(game.state, crate::GameState::PlayWaitForCard);
                }

                #[test]
                fn process_nibs_nibs_no_partner() {
                    let mut game = set_up_game(false, true, false, false, false);
                    process_cut(&mut game).unwrap();
                    game.starter_card = Some(crate::util::return_card('J', 'H'));

                    let score_event = crate::score::ScoreEvent {
                        score_type: crate::score::ScoreType::Play(
                            crate::score::PlayScoreType::Nibs,
                        ),
                        player_index: 0,
                        point_value: 2,
                    };

                    assert_eq!(
                        process_nibs(&mut game, Some(score_event)),
                        Ok(crate::game_process_return::Success::NibsCheck)
                    );
                    assert_eq!(game.players[0].front_peg_pos, 2);
                    assert_eq!(game.state, crate::GameState::PlayWaitForCard);
                }

                #[test]
                fn process_nibs_nibs_and_win_no_partner() {
                    let mut game = set_up_game(false, true, false, false, false);
                    process_cut(&mut game).unwrap();
                    game.starter_card = Some(crate::util::return_card('J', 'H'));
                    game.players[0].change_score(119);

                    let score_event = crate::score::ScoreEvent {
                        score_type: crate::score::ScoreType::Play(
                            crate::score::PlayScoreType::Nibs,
                        ),
                        player_index: 0,
                        point_value: 2,
                    };

                    assert_eq!(
                        process_nibs(&mut game, Some(score_event)),
                        Ok(crate::game_process_return::Success::NibsCheck)
                    );
                    assert_eq!(game.players[0].front_peg_pos, 121);
                    assert_eq!(game.state, crate::GameState::Win);
                }

                #[test]
                fn process_nibs_nibs_invalid_call() {
                    let mut game = set_up_game(false, true, false, false, false);
                    process_cut(&mut game).unwrap();
                    game.starter_card = Some(crate::util::return_card('J', 'H'));

                    let score_event = crate::score::ScoreEvent {
                        score_type: crate::score::ScoreType::Play(
                            crate::score::PlayScoreType::Nibs,
                        ),
                        player_index: 0,
                        point_value: 3,
                    };

                    assert_eq!(
                        process_nibs(&mut game, Some(score_event)),
                        Err(crate::game_process_return::Error::NibsCallError(
                            crate::game_process_return::NibsError::InvalidScoreEventToNibsCheck
                        ))
                    );
                    assert_eq!(game.state, crate::GameState::NibsCheck);
                }

                #[test]
                fn process_nibs_nibs_with_partner() {
                    let mut game = set_up_game(true, true, false, false, false);
                    process_cut(&mut game).unwrap();
                    game.starter_card = Some(crate::util::return_card('J', 'H'));

                    let score_event = crate::score::ScoreEvent {
                        score_type: crate::score::ScoreType::Play(
                            crate::score::PlayScoreType::Nibs,
                        ),
                        player_index: 0,
                        point_value: 2,
                    };

                    assert_eq!(
                        process_nibs(&mut game, Some(score_event)),
                        Ok(crate::game_process_return::Success::NibsCheck)
                    );
                    assert_eq!(game.players[0].front_peg_pos, 2);
                    assert_eq!(game.players[1].front_peg_pos, 2);
                    assert_eq!(game.state, crate::GameState::PlayWaitForCard);
                }

                #[test]
                fn process_nibs_nibs_and_win_with_partner() {
                    let mut game = set_up_game(true, true, false, false, false);
                    process_cut(&mut game).unwrap();
                    game.starter_card = Some(crate::util::return_card('J', 'H'));
                    game.players[0].change_score(119);
                    game.players[1].change_score(119);

                    let score_event = crate::score::ScoreEvent {
                        score_type: crate::score::ScoreType::Play(
                            crate::score::PlayScoreType::Nibs,
                        ),
                        player_index: 0,
                        point_value: 2,
                    };

                    assert_eq!(
                        process_nibs(&mut game, Some(score_event)),
                        Ok(crate::game_process_return::Success::NibsCheck)
                    );
                    assert_eq!(game.players[0].front_peg_pos, 121);
                    assert_eq!(game.players[1].front_peg_pos, 121);
                    assert_eq!(game.state, crate::GameState::Win);
                }
            }

            mod overpegging {
                use super::super::super::super::{process_cut, process_nibs};
                use super::super::super::set_up_game;
                #[test]
                fn process_nibs_with_overpegging_no_nibs_claimed_no_player_contested() {}

                #[test]
                fn process_nibs_with_overpegging_no_nibs_claimed_with_player_contested() {}

                #[test]
                fn process_nibs_with_overpegging_no_nibs_claimed_no_player_not_contested() {}

                #[test]
                fn process_nibs_with_overpegging_no_nibs_claimed_with_player_not_contested() {}
            }
        }
    }
}

// Cut the starter card from the deck
pub(crate) fn process_cut(
    game: &mut crate::GameImpl,
) -> Result<game_process_return::Success, game_process_return::Error> {
    game.starter_card = Some(game.deck.deal());

    // If the cut card is a jack, the dealer scores two points
    if !game.settings.unwrap().is_manual_scoring {
        if crate::deck::return_value(game.starter_card.unwrap()) == 11 {
            let dealer_index: usize = game.index_dealer.unwrap() as usize;

            game.players[dealer_index].change_score(2);
            if let Some(partner_index) = game.players[dealer_index].partner_index {
                game.players[partner_index as usize].change_score(2);
            }

            if game.players[dealer_index].front_peg_pos >= 121 {
                game.state = crate::GameState::Win;
                return Ok(game_process_return::Success::StarterCut);
            }
        }

        game.state = crate::GameState::PlayWaitForCard;
    } else {
        game.state = crate::GameState::NibsCheck;
    }

    Ok(game_process_return::Success::StarterCut)
}

// Quick note: the ACC rules specify that you can't score points with muggins on the nibs
// call; when lowball is enabled, the contest function will be used to force the dealer to take
// points

// When underpegging or overpegging is enabled, process whether the dealer calls nibs or not
pub(crate) fn process_nibs(
    game: &mut crate::GameImpl,
    call: Option<crate::score::ScoreEvent>,
) -> Result<game_process_return::Success, game_process_return::Error> {
    if let Some(starter) = game.starter_card {
        if crate::deck::return_value(starter) == 11 {
            // If the starter card is a jack and if there is no call, throw an error if
            // underscoring is disabled or allow the None call
            if call.is_none() {
                if !game.settings.unwrap().is_underpegging {
                    Err(game_process_return::Error::NibsCallError(
                        game_process_return::NibsError::NoNibsCallWhenUnderscoringIsDisabled,
                    ))
                } else {
                    game.state = crate::GameState::PlayWaitForCard;
                    Ok(game_process_return::Success::NibsCheck)
                }
            }
            // If the starter card is a jack and there is a Nibs call, check that the score event
            // passed is valid and either throw an error or proceed to the next state
            else {
                if call
                    == Some(crate::score::ScoreEvent {
                        score_type: crate::score::ScoreType::Play(
                            crate::score::PlayScoreType::Nibs,
                        ),
                        player_index: game.index_dealer.unwrap(),
                        point_value: 2,
                    })
                {
                    game.players[game.index_dealer.unwrap() as usize].change_score(2);
                    if let Some(partner_index) =
                        game.players[game.index_dealer.unwrap() as usize].partner_index
                    {
                        game.players[partner_index as usize].change_score(2);
                    }
                    if game.players[game.index_dealer.unwrap() as usize].front_peg_pos >= 121 {
                        game.state = crate::GameState::Win;
                    } else {
                        game.state = crate::GameState::PlayWaitForCard;
                    }
                    Ok(game_process_return::Success::NibsCheck)
                } else {
                    Err(game_process_return::Error::NibsCallError(
                        game_process_return::NibsError::InvalidScoreEventToNibsCheck,
                    ))
                }
            }
        } else {
            // If the card is not a jack and there is no call, do nothing but proceed to the next
            // state
            if call.is_none() {
                game.state = crate::GameState::PlayWaitForCard;
                Ok(game_process_return::Success::NibsCheck)
            }
            // If the card is not a jack and there is a call, throw an error if overscoring is
            // disabled or allow the call and proceed to the contest state
            else {
                Err(game_process_return::Error::UnimplementedState)
            }
        }
    } else {
        Err(game_process_return::Error::ImplementationError(
            game_process_return::ImplError::NibsCheckWhenNoStarterCard,
        ))
    }
}

// When overpegging is enabled, process whether someone contests the nibs call
pub(crate) fn process_nibs_contest(
    game: &mut crate::GameImpl,
    call: Option<crate::score::ScoreEvent>,
) -> Result<game_process_return::Success, game_process_return::Error> {
    Ok(game_process_return::Success::NibsContest)
}

fn ready_for_play() {}
