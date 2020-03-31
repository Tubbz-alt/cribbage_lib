use crate::deck;
use crate::game_process_return;

#[cfg(test)]
mod test {
    fn set_up_game(variant: crate::settings::RuleVariant) -> crate::GameImpl {
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

        game.deck = crate::deck::Deck::new();

        crate::state_logic::cut_initial::process_cut(&mut game).unwrap();

        game.deck = crate::deck::Deck::new();

        crate::state_logic::deal::process_deal(&mut game).unwrap();

        crate::state_logic::sort::process_sort(&mut game).unwrap();

        game
    }

    #[test]
    fn two_standard_discard_validity() {
        let mut game = set_up_game(crate::settings::RuleVariant::TwoStandard);

        // Test invalid number of inner vectors
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0, 1]];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err( super::game_process_return::Error::ImplementationError(
                    super::game_process_return::ImplError::ThereShouldBeOneDiscardIndicesVectorPerPlayer 
                )
            )
        );

        // Test invalid number of indices4
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![0,1,2]];
        let expected_output: Vec<super::game_process_return::DiscardError> = vec![
            super::game_process_return::DiscardError::TwoCardsAreDiscardedWithTwoPlayers(0),
            super::game_process_return::DiscardError::TwoCardsAreDiscardedWithTwoPlayers(1)
        ];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err(super::game_process_return::Error::DiscardErrors(expected_output))
        );

        // Test repeated index
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0,0], vec![0,1]];
        let expected_output: Vec<super::game_process_return::DiscardError> = vec![
            super::game_process_return::DiscardError::TwoCardIndicesMayNotBeRepeated(0)
        ];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err(super::game_process_return::Error::DiscardErrors(expected_output))
        );

        // Test index out of bounds
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0, 6], vec![0, 1]];
        let expected_output: Vec<super::game_process_return::DiscardError> = vec![
            super::game_process_return::DiscardError::IndicesAreBetween0And5InclusiveWithTwoStandard(0)
        ];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err(super::game_process_return::Error::DiscardErrors(expected_output))
        );

        // Test valid input
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0,1], vec![0,1]];
        let mut expected_discards: Vec<crate::deck::Card>  = vec![
            game.players[0].hand[0],
            game.players[0].hand[1], 
            game.players[1].hand[0],
            game.players[1].hand[1]
        ];
        expected_discards.sort();
        assert_eq!(super::process_discard(&mut game, discard_indices_group), Ok(super::game_process_return::Success::Discard));
        assert_eq!(game.crib, expected_discards);
        assert_eq!(game.state, crate::GameState::CutStarter);
    }

    #[test]
    fn two_five_card_discard_validity() {
        let mut game = set_up_game(crate::settings::RuleVariant::TwoFiveCard);

        // Test invalid number of inner vectors
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0, 1]];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err( super::game_process_return::Error::ImplementationError(
                    super::game_process_return::ImplError::ThereShouldBeOneDiscardIndicesVectorPerPlayer 
                )
            )
        );

        // Test invalid number of indices
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![0,1,2]];
        let expected_output: Vec<super::game_process_return::DiscardError> = vec![
            super::game_process_return::DiscardError::TwoCardsAreDiscardedWithTwoPlayers(0),
            super::game_process_return::DiscardError::TwoCardsAreDiscardedWithTwoPlayers(1)
        ];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err(super::game_process_return::Error::DiscardErrors(expected_output))
        );

        // Test repeated index
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0,0], vec![0,1]];
        let expected_output: Vec<super::game_process_return::DiscardError> = vec![
            super::game_process_return::DiscardError::TwoCardIndicesMayNotBeRepeated(0)
        ];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err(super::game_process_return::Error::DiscardErrors(expected_output))
        );

        // Test index out of bounds
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0, 5], vec![0, 1]];
        let expected_output: Vec<super::game_process_return::DiscardError> = vec![
            super::game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithTwoFiveCard(0)
        ];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err(super::game_process_return::Error::DiscardErrors(expected_output))
        );

        // Test valid input
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0,1], vec![0,1]];
        let mut expected_discards: Vec<crate::deck::Card>  = vec![
            game.players[0].hand[0],
            game.players[0].hand[1], 
            game.players[1].hand[0],
            game.players[1].hand[1]
        ];
        expected_discards.sort();
        assert_eq!(super::process_discard(&mut game, discard_indices_group), Ok(super::game_process_return::Success::Discard));
        assert_eq!(game.crib, expected_discards);
    }

    #[test]
    fn three_player_discard_validity() {
        for variant in vec![crate::settings::RuleVariant::ThreeStandard, crate::settings::RuleVariant::ThreeCaptain] {
            let mut game = set_up_game(variant);

            // Test invalid number of inner vectors
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![0], vec![0], vec![0]];
            assert_eq!(
                super::process_discard(&mut game, discard_indices_group),
                Err( super::game_process_return::Error::ImplementationError(
                        super::game_process_return::ImplError::ThereShouldBeOneDiscardIndicesVectorPerPlayer 
                    )
                )
            );

            // Test invalid number of indices
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![0,1], vec![0]];
            let expected_output: Vec<super::game_process_return::DiscardError> = vec![
                super::game_process_return::DiscardError::OneCardIsDiscardedWithThreePlayers(1)
            ];
            assert_eq!(
                super::process_discard(&mut game, discard_indices_group),
                Err(super::game_process_return::Error::DiscardErrors(expected_output))
            );

            // Test index out of bounds
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![5], vec![0], vec![0]];
            let expected_output: Vec<super::game_process_return::DiscardError> = vec![
                super::game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMorePlayers(0)
            ];
            assert_eq!(
                super::process_discard(&mut game, discard_indices_group),
                Err(super::game_process_return::Error::DiscardErrors(expected_output))
            );

            // Multiple errors test
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![5], vec![0,1], vec![0]];
            let expected_output: Vec<super::game_process_return::DiscardError> = vec![
                super::game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMorePlayers(0),
                super::game_process_return::DiscardError::OneCardIsDiscardedWithThreePlayers(1),
            ];
            assert_eq!(
                super::process_discard(&mut game, discard_indices_group),
                Err(super::game_process_return::Error::DiscardErrors(expected_output))
            );

            // Test valid input
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![0], vec![0]];
            // The card dealt directly to the crib should be a three of diamonds with the sorted
            // debug deck
            let mut expected_discards: Vec<crate::deck::Card>  = vec![
                game.players[0].hand[0],
                game.players[1].hand[0], 
                game.players[2].hand[0],
                crate::util::return_card('3', 'D')
            ];
            expected_discards.sort();
            assert_eq!(super::process_discard(&mut game, discard_indices_group), Ok(super::game_process_return::Success::Discard));
            assert_eq!(game.crib, expected_discards);
        }
    }

    #[test]
    fn four_player_discard_validity() {
        for variant in vec![crate::settings::RuleVariant::FourIndividual, crate::settings::RuleVariant::FourPairs] {

            let mut game = set_up_game(variant);

            // Test invalid number of inner vectors
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![0], vec![0]];
            assert_eq!(
                super::process_discard(&mut game, discard_indices_group),
                Err( super::game_process_return::Error::ImplementationError(
                        super::game_process_return::ImplError::ThereShouldBeOneDiscardIndicesVectorPerPlayer 
                    )
                )
            );

            // Test invalid number of indices
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![0,1], vec![0], vec![0]];
            let expected_output: Vec<super::game_process_return::DiscardError> = vec![
                super::game_process_return::DiscardError::OneCardIsDiscardedWithFourPlayers(1)
            ];
            assert_eq!(
                super::process_discard(&mut game, discard_indices_group),
                Err(super::game_process_return::Error::DiscardErrors(expected_output))
            );

            // Test index out of bounds
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![5], vec![0], vec![0], vec![0]];
            let expected_output: Vec<super::game_process_return::DiscardError> = vec![
                super::game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMorePlayers(0)
            ];
            assert_eq!(
                super::process_discard(&mut game, discard_indices_group),
                Err(super::game_process_return::Error::DiscardErrors(expected_output))
            );

            // Multiple errors test
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![5], vec![0,1], vec![0], vec![0]];
            let expected_output: Vec<super::game_process_return::DiscardError> = vec![
                super::game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMorePlayers(0),
                super::game_process_return::DiscardError::OneCardIsDiscardedWithFourPlayers(1),
            ];
            assert_eq!(
                super::process_discard(&mut game, discard_indices_group),
                Err(super::game_process_return::Error::DiscardErrors(expected_output))
            );

            // Test valid input
            let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![0], vec![0], vec![0]];
            // The card dealt directly to the crib should be a three of diamonds with the sorted
            // debug deck
            let mut expected_discards: Vec<crate::deck::Card>  = vec![
                game.players[0].hand[0],
                game.players[1].hand[0], 
                game.players[2].hand[0],
                game.players[3].hand[0],
            ];
            expected_discards.sort();
            assert_eq!(super::process_discard(&mut game, discard_indices_group), Ok(super::game_process_return::Success::Discard));
            assert_eq!(game.crib, expected_discards);
        }
    }

    #[test]
    fn five_player_discard_validity() {
        let mut game = set_up_game(crate::settings::RuleVariant::FiveStandard);
        
        // Test invalid number of indices
        // In five player cribbage, everyone but the dealer puts one card into the crib; in the game
        // returned by set_up_game the dealer's index is 0
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![], vec![0], vec![0], vec![0]];
        let expected_output: Vec<super::game_process_return::DiscardError> = vec![
            super::game_process_return::DiscardError::NoCardsAreDiscardedForDealerWithFivePlayers(0),
            super::game_process_return::DiscardError::OneCardIsDiscardedForNonDealersWithFivePlayers(1),
        ];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err(super::game_process_return::Error::DiscardErrors(expected_output))
        );

        // Test valid config
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![], vec![0], vec![0], vec![0], vec![0]];
        let mut expected_discards: Vec<crate::deck::Card> = vec![
            game.players[1].hand[0],
            game.players[2].hand[0],
            game.players[3].hand[0],
            game.players[4].hand[0],
        ];
        expected_discards.sort();
        assert_eq!(super::process_discard(&mut game, discard_indices_group), Ok(super::game_process_return::Success::Discard));
        assert_eq!(game.crib, expected_discards);
    }

    #[test]
    fn six_player_discard_validity() {
        let mut game = set_up_game(crate::settings::RuleVariant::SixPairs);
        
        // Test invalid number of indices
        // In six player cribbage, everyone but the dealer and the dealer's partner puts one card
        // into the crib; in the game returned by set_up_game the dealer's index is 0 and the
        // dealer partner's index is 3
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![0], vec![], vec![0], vec![0], vec![0], vec![0]];
        let expected_output: Vec<super::game_process_return::DiscardError> = vec![
            super::game_process_return::DiscardError::NoCardsAreDiscardedForDealerAndDealerPartnerWithSixPlayers(0),
            super::game_process_return::DiscardError::OneCardIsDiscardedForNonDealersAndNonDealerPartnersWithSixPlayers(1),
            super::game_process_return::DiscardError::NoCardsAreDiscardedForDealerAndDealerPartnerWithSixPlayers(3),
        ];
        assert_eq!(
            super::process_discard(&mut game, discard_indices_group),
            Err(super::game_process_return::Error::DiscardErrors(expected_output))
        );

        // Test valid config
        let discard_indices_group: Vec<Vec<u8>> = vec![vec![], vec![0], vec![0], vec![], vec![0], vec![0]];
        let mut expected_discards: Vec<crate::deck::Card> = vec![
            game.players[1].hand[0],
            game.players[2].hand[0],
            game.players[4].hand[0],
            game.players[5].hand[0],
        ];
        expected_discards.sort();
        assert_eq!(super::process_discard(&mut game, discard_indices_group), Ok(super::game_process_return::Success::Discard));
        assert_eq!(game.crib, expected_discards);
    }
}

pub(crate) fn process_discard(
    game: &mut crate::GameImpl,
    discard_indices_group: Vec<Vec<u8>>,
) -> Result<game_process_return::Success, game_process_return::Error> {
    match check_discard_validity(game, &discard_indices_group) {
        Ok(discard_errors) => {
            if discard_errors.len() > 0 {
                Err(game_process_return::Error::DiscardErrors(discard_errors))
            } else {
                execute_discard(game, discard_indices_group);
                Ok(game_process_return::Success::Discard)
            }
        }
        // Should never happen unless you're using the GameImpl directly, but who knows?
        Err(e) => Err(e),
    }
}

fn check_discard_validity(
    game: &mut crate::GameImpl,
    discard_indices_group: &Vec<Vec<u8>>,
) -> Result<Vec<game_process_return::DiscardError>, game_process_return::Error> {
    // There should be one discard_indices vector for each player
    if discard_indices_group.len() != game.players.len() {
        return Err(game_process_return::Error::ImplementationError(
            game_process_return::ImplError::ThereShouldBeOneDiscardIndicesVectorPerPlayer,
        ));
    }

    let mut errors: Vec<game_process_return::DiscardError> = Vec::new();
    for (player_index, discards) in discard_indices_group.iter().enumerate() {
        if let Some(settings) = game.settings {
            let error: Option<game_process_return::DiscardError> = match settings.variant {
                crate::settings::RuleVariant::TwoStandard => {
                    check_two_player_validity(player_index as u8, discards, false)
                }
                crate::settings::RuleVariant::TwoFiveCard => {
                    check_two_player_validity(player_index as u8, discards, true)
                }
                crate::settings::RuleVariant::ThreeStandard => {
                    check_three_or_four_player_validity(player_index as u8, discards, 3)
                }
                crate::settings::RuleVariant::ThreeCaptain => {
                    check_three_or_four_player_validity(player_index as u8, discards, 3)
                }
                crate::settings::RuleVariant::FourIndividual => {
                    check_three_or_four_player_validity(player_index as u8, discards, 4)
                }
                crate::settings::RuleVariant::FourPairs => {
                    check_three_or_four_player_validity(player_index as u8, discards, 4)
                }
                crate::settings::RuleVariant::FiveStandard => check_five_player_validity(
                    player_index as u8,
                    discards,
                    game.index_dealer.unwrap(),
                ),
                crate::settings::RuleVariant::SixPairs => check_six_player_validity(
                    player_index as u8,
                    discards,
                    game.index_dealer.unwrap(),
                    game.players[game.index_dealer.unwrap() as usize]
                        .partner_index
                        .unwrap(),
                ),
            };

            if let Some(error) = error {
                errors.push(error);
            }
        } else {
            // Game should always have a valid settings config if the interface is used properly,
            // but who knows?
            return Err(game_process_return::Error::ImplementationError(
                game_process_return::ImplError::NoConfig,
            ));
        }
    }
    Ok(errors)
}

fn check_two_player_validity(
    player_index: u8,
    discard_indices: &Vec<u8>,
    is_five_card: bool,
) -> Option<game_process_return::DiscardError> {
    if discard_indices.len() != 2 {
        Some(game_process_return::DiscardError::TwoCardsAreDiscardedWithTwoPlayers(player_index))
    } else {
        if discard_indices[0] == discard_indices[1] {
            Some(game_process_return::DiscardError::TwoCardIndicesMayNotBeRepeated(player_index))
        } else {
            if is_five_card && (discard_indices[0] > 4 || discard_indices[1] > 4) {
                Some(game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithTwoFiveCard(player_index))
            } else if !is_five_card && (discard_indices[0] > 5 || discard_indices[1] > 5) {
                Some(game_process_return::DiscardError::IndicesAreBetween0And5InclusiveWithTwoStandard(player_index))
            } else {
                None
            }
        }
    }
}

fn check_three_or_four_player_validity(
    player_index: u8,
    discard_indices: &Vec<u8>,
    num_players: u8,
) -> Option<game_process_return::DiscardError> {
    if discard_indices.len() != 1 {
        if num_players == 3 {
            Some(
                game_process_return::DiscardError::OneCardIsDiscardedWithThreePlayers(player_index),
            )
        } else {
            Some(game_process_return::DiscardError::OneCardIsDiscardedWithFourPlayers(player_index))
        }
    } else {
        if discard_indices[0] > 4 {
            Some(game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMorePlayers(player_index))
        } else {
            None
        }
    }
}

fn check_five_player_validity(
    player_index: u8,
    discard_indices: &Vec<u8>,
    index_dealer: u8,
) -> Option<game_process_return::DiscardError> {
    if player_index == index_dealer {
        if discard_indices.len() != 0 {
            Some(
                game_process_return::DiscardError::NoCardsAreDiscardedForDealerWithFivePlayers(
                    player_index,
                ),
            )
        } else {
            None
        }
    } else {
        if discard_indices.len() != 1 {
            Some(
                game_process_return::DiscardError::OneCardIsDiscardedForNonDealersWithFivePlayers(
                    player_index,
                ),
            )
        } else if discard_indices[0] > 4 {
            Some(game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMorePlayers(player_index))
        } else {
            None
        }
    }
}

fn check_six_player_validity(
    player_index: u8,
    discard_indices: &Vec<u8>,
    index_dealer: u8,
    index_dealer_partner: u8,
) -> Option<game_process_return::DiscardError> {
    if player_index == index_dealer || player_index == index_dealer_partner {
        if discard_indices.len() != 0 {
            Some(game_process_return::DiscardError::NoCardsAreDiscardedForDealerAndDealerPartnerWithSixPlayers(player_index))
        } else {
            None
        }
    } else {
        if discard_indices.len() != 1 {
            Some(game_process_return::DiscardError::OneCardIsDiscardedForNonDealersAndNonDealerPartnersWithSixPlayers(player_index))
        } else if discard_indices[0] > 4 {
            Some(game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMorePlayers(player_index))
        } else {
            None
        }
    }
}

fn execute_discard(game: &mut crate::GameImpl, discard_indices_group: Vec<Vec<u8>>) {
    for (player_index, discard_indices) in discard_indices_group.iter().enumerate() {
        let mut selected_cards: Vec<deck::Card> = Vec::new();
        for discard_index in discard_indices {
            selected_cards.push(game.players[player_index as usize].hand[*discard_index as usize]);
        }
        
        for card in &mut selected_cards {
            game.crib.push(*card);
            game.players[player_index as usize].hand.retain({
                |&hand_card| {
                    if hand_card == *card {
                        false
                    } else {
                        true
                    }
                }
            });
        }

        game.crib.sort();

        game.state = crate::GameState::CutStarter;
    }
}
