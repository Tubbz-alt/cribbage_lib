use crate::game_process_return;
use crate::player;

#[cfg(test)]
mod test {
    // The following test check that the entire game_setup function (the public function used by
    // the Game object) works properly in returning either Success::GameStart or forwarding the
    // error received in checking setting validity
    #[test]
    fn game_setup() {
        let mut game = crate::GameImpl::new();
        // Test that game_setup returns Success::GameStart in the event of valid configuration
        assert_eq!(
            super::game_setup(
                &mut game,
                crate::settings::GameSettings {
                    variant: crate::settings::RuleVariant::TwoStandard,
                    victor_dealer_option: crate::settings::VictorDealerOption::TwoPlayers,
                    is_manual_scoring: false,
                    is_underpegging: false,
                    is_muggins: false,
                    is_overpegging: false,
                    is_lowball: false,
                }
            ),
            Ok(super::game_process_return::Success::GameStart)
        );

        // Test that game_setup forwards the error received by the settings validity checking
        // function
        game = crate::GameImpl::new();
        assert_eq!(
            super::game_setup(
                &mut game,
                crate::settings::GameSettings {
                    variant: crate::settings::RuleVariant::TwoStandard,
                    victor_dealer_option: crate::settings::VictorDealerOption::LastPlaceIsDealer,
                    is_manual_scoring: false,
                    is_underpegging: false,
                    is_muggins: false,
                    is_overpegging: false,
                    is_lowball: false,
                }
            ),
            Err(super::game_process_return::Error::GameStartInvalidConfig(
                super::game_process_return::ConfigError::VDOIsNotTwoPlayersWhenVariantIsTwoPlayers
            ))
        );
    }

    // The following three tests check that settings validity is properly handled

    // An error should be returned when the victor_dealer_option is not TwoPlayers when the variant
    // is TwoStandard or TwoFiveCard, when the victor_dealer_option is TwoPlayers with any other
    // variant, or when the victor_dealer_option is LastPlaceIsDealer when the variant is ThreeCaptain
    #[test]
    fn settings_validity_vdo() {
        let variants = crate::util::return_variants();
        let victor_dealer_options = vec![
            crate::settings::VictorDealerOption::LastPlaceIsDealer,
            crate::settings::VictorDealerOption::LosersDrawForDealer,
            crate::settings::VictorDealerOption::TwoPlayers,
        ];

        for var in &variants {
            for vdo in &victor_dealer_options {
                let settings = crate::settings::GameSettings {
                    variant: *var,
                    victor_dealer_option: *vdo,
                    is_manual_scoring: false,
                    is_underpegging: false,
                    is_muggins: false,
                    is_overpegging: false,
                    is_lowball: false,
                };

                // If the variant is TwoStandard or TwoFiveCard and the victor_dealer_option is
                // TwoPlayers return Ok, otherwise return VDOIsNotTwoPlayersWhenVariantIsTwoPlayers
                if settings.variant == crate::settings::RuleVariant::TwoStandard
                    || settings.variant == crate::settings::RuleVariant::TwoFiveCard
                {
                    if settings.victor_dealer_option
                        == crate::settings::VictorDealerOption::TwoPlayers
                    {
                        assert_eq!(super::check_settings_validity(settings), Ok(()));
                    } else {
                        assert_eq!(super::check_settings_validity(settings), Err(
                            super::game_process_return::Error::GameStartInvalidConfig(
                                super::game_process_return::ConfigError::VDOIsNotTwoPlayersWhenVariantIsTwoPlayers
                            )
                        ));
                    }
                }
                // If the variant is ThreeCaptain and the victor_dealer_option is
                // LosersDrawForDealer return Ok, but if the victor_dealer_option is TwoPlayers
                // return VDOIsTwoPlayersWhenVariantIsMoreThanTwoPlayers or if the
                // victor_dealer_option is LastPlaceIsDealer return
                // VDOIsNotLoserDrawsForDealerWhenVariantIsThreeCaptain
                else if settings.variant == crate::settings::RuleVariant::ThreeCaptain {
                    if settings.victor_dealer_option
                        == crate::settings::VictorDealerOption::TwoPlayers
                    {
                        assert_eq!(super::check_settings_validity(settings), Err(
                            super::game_process_return::Error::GameStartInvalidConfig(
                                super::game_process_return::ConfigError::VDOIsTwoPlayersWhenVariantIsMoreThanTwoPlayers
                            )
                        ));
                    } else if settings.victor_dealer_option
                        == crate::settings::VictorDealerOption::LastPlaceIsDealer
                    {
                        assert_eq!(super::check_settings_validity(settings), Err(
                            super::game_process_return::Error::GameStartInvalidConfig(
                                super::game_process_return::ConfigError::VDOIsNotLoserDrawsForDealerWhenVariantIsThreeCaptain
                            )
                        ));
                    } else {
                        assert_eq!(super::check_settings_validity(settings), Ok(()));
                    }
                }
                // For any other variant, return Ok if the victor_dealer_option isn't TwoPlayers
                // and return VDOIsTwoPlayersWhenVariantIsMoreThanTwoPlayers if it is
                else {
                    if settings.victor_dealer_option
                        != crate::settings::VictorDealerOption::TwoPlayers
                    {
                        assert_eq!(super::check_settings_validity(settings), Ok(()));
                    } else {
                        assert_eq!(super::check_settings_validity(settings), Err(
                            super::game_process_return::Error::GameStartInvalidConfig(
                                super::game_process_return::ConfigError::VDOIsTwoPlayersWhenVariantIsMoreThanTwoPlayers
                            )
                        ));
                    }
                }
            }
        }
    }

    // An error should be returned with the variant is ThreeCaptain and Lowball is enabled
    #[test]
    fn settings_validity_lowball_three_captain() {
        let bool_options = vec![false, true];
        for option in &bool_options {
            let settings = crate::settings::GameSettings {
                variant: crate::settings::RuleVariant::ThreeCaptain,
                victor_dealer_option: crate::settings::VictorDealerOption::LosersDrawForDealer,
                is_manual_scoring: false,
                is_underpegging: false,
                is_muggins: false,
                is_overpegging: false,
                is_lowball: *option,
            };

            if settings.is_lowball {
                assert_eq!(super::check_settings_validity(settings),
                Err(super::game_process_return::Error::GameStartInvalidConfig(
                    super::game_process_return::ConfigError::LowballEnabledWhenVariantIsThreeCaptain
                )));
            } else {
                assert_eq!(super::check_settings_validity(settings), Ok(()));
            }
        }
    }

    // An error should be returned when underpegging, overpegging, or muggins are enabled when
    // manual scoring is not enabled or when muggins is enabled when underpegging is not enabled
    #[test]
    fn settings_validity_boolean_flags() {
        let bool_options = vec![false, true];
        for man_option in &bool_options {
            for under_option in &bool_options {
                for muggins_option in &bool_options {
                    for over_option in &bool_options {
                        let settings = crate::settings::GameSettings {
                            variant: crate::settings::RuleVariant::TwoStandard,
                            victor_dealer_option: crate::settings::VictorDealerOption::TwoPlayers,
                            is_manual_scoring: *man_option,
                            is_underpegging: *under_option,
                            is_muggins: *muggins_option,
                            is_overpegging: *over_option,
                            is_lowball: false,
                        };

                        match (*man_option, *under_option, *muggins_option, *over_option) {
                            (false, true, _, _) => {
                                assert_eq!(super::check_settings_validity(settings),
                                    Err(crate::game_process_return::Error::GameStartInvalidConfig(
                                        crate::game_process_return::ConfigError::UnderpeggingEnabledWhenManualScoringIsDisabled
                                    ))
                                );
                            }
                            (false, _, _, true) => {
                                assert_eq!(super::check_settings_validity(settings),
                                    Err(crate::game_process_return::Error::GameStartInvalidConfig(
                                        crate::game_process_return::ConfigError::OverpeggingEnabledWhenManualScoringIsDisabled
                                    ))
                                );
                            }
                            (false, _, true, _) => {
                                assert_eq!(super::check_settings_validity(settings),
                                    Err(crate::game_process_return::Error::GameStartInvalidConfig(
                                        crate::game_process_return::ConfigError::MugginsEnabledWhenManualScoringIsDisabled
                                    ))
                                );
                            }
                            (true, false, true, _) => {
                                assert_eq!(super::check_settings_validity(settings),
                                    Err(crate::game_process_return::Error::GameStartInvalidConfig(
                                        crate::game_process_return::ConfigError::MugginsEnabledWhenUnderpeggingIsDisabled
                                    ))
                                );
                            }
                            _ => {
                                assert_eq!(super::check_settings_validity(settings), Ok(()));
                            }
                        };
                    }
                }
            }
        }
    }

    // Helper function to return a game with a given RuleVariant for testing the player_setup
    // function
    fn set_up_game(variant: crate::settings::RuleVariant) -> crate::GameImpl {
        let mut game = crate::GameImpl::new();
        game.settings = Some(crate::settings::GameSettings {
            variant: variant,
            victor_dealer_option: crate::settings::VictorDealerOption::TwoPlayers,
            is_manual_scoring: false,
            is_underpegging: false,
            is_muggins: false,
            is_overpegging: false,
            is_lowball: false,
        });

        game
    }

    // The following eight tests check that the set_up_players functions correctly for every
    // RuleVariant

    #[test]
    fn set_up_two_standard() {
        let mut game = set_up_game(crate::settings::RuleVariant::TwoStandard);
        super::set_up_players(&mut game);
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.players[0].partner_index, None);
        assert_eq!(game.players[1].partner_index, None);
    }

    #[test]
    fn set_up_two_five_card() {
        let mut game = set_up_game(crate::settings::RuleVariant::TwoFiveCard);
        super::set_up_players(&mut game);
        assert_eq!(game.players.len(), 2);
        assert_eq!(game.players[0].partner_index, None);
        assert_eq!(game.players[1].partner_index, None);
    }

    #[test]
    fn set_up_three_standard() {
        let mut game = set_up_game(crate::settings::RuleVariant::ThreeStandard);
        super::set_up_players(&mut game);
        assert_eq!(game.players.len(), 3);
        assert_eq!(game.players[0].partner_index, None);
        assert_eq!(game.players[1].partner_index, None);
        assert_eq!(game.players[2].partner_index, None);
    }

    #[test]
    fn set_up_three_captain() {
        let mut game = set_up_game(crate::settings::RuleVariant::ThreeCaptain);
        super::set_up_players(&mut game);
        assert_eq!(game.players.len(), 3);
        assert_eq!(game.players[0].partner_index, None);
        assert_eq!(game.players[1].partner_index, Some(2));
        assert_eq!(game.players[2].partner_index, Some(1));
    }

    #[test]
    fn set_up_four_individual() {
        let mut game = set_up_game(crate::settings::RuleVariant::FourIndividual);
        super::set_up_players(&mut game);
        assert_eq!(game.players.len(), 4);
        assert_eq!(game.players[0].partner_index, None);
        assert_eq!(game.players[1].partner_index, None);
        assert_eq!(game.players[2].partner_index, None);
        assert_eq!(game.players[3].partner_index, None);
    }

    #[test]
    fn set_up_four_pairs() {
        let mut game = set_up_game(crate::settings::RuleVariant::FourPairs);
        super::set_up_players(&mut game);
        assert_eq!(game.players.len(), 4);
        assert_eq!(game.players[0].partner_index, Some(2));
        assert_eq!(game.players[1].partner_index, Some(3));
        assert_eq!(game.players[2].partner_index, Some(0));
        assert_eq!(game.players[3].partner_index, Some(1));
    }

    #[test]
    fn set_up_five_standard() {
        let mut game = set_up_game(crate::settings::RuleVariant::FiveStandard);
        super::set_up_players(&mut game);
        assert_eq!(game.players.len(), 5);
        assert_eq!(game.players[0].partner_index, None);
        assert_eq!(game.players[1].partner_index, None);
        assert_eq!(game.players[2].partner_index, None);
        assert_eq!(game.players[3].partner_index, None);
        assert_eq!(game.players[4].partner_index, None);
    }

    #[test]
    fn set_up_six_pairs() {
        let mut game = set_up_game(crate::settings::RuleVariant::SixPairs);
        super::set_up_players(&mut game);
        assert_eq!(game.players.len(), 6);
        assert_eq!(game.players[0].partner_index, Some(3));
        assert_eq!(game.players[1].partner_index, Some(4));
        assert_eq!(game.players[2].partner_index, Some(5));
        assert_eq!(game.players[3].partner_index, Some(0));
        assert_eq!(game.players[4].partner_index, Some(1));
        assert_eq!(game.players[5].partner_index, Some(2));
    }
}

pub(crate) fn game_setup(
    game: &mut crate::GameImpl,
    settings: crate::settings::GameSettings,
) -> Result<game_process_return::Success, game_process_return::Error> {
    // If the settings are invalid, return the error given, otherwise set the game settings to the
    // given settings
    match check_settings_validity(settings) {
        Err(e) => return Err(e),
        Ok(()) => {}
    }
    game.settings = Some(settings);

    set_up_players(game);

    game.deck.reset_deck();
    game.crib = Vec::with_capacity(4);

    // When a game is being setup, the initial cut is between all players
    for index in 0..game.players.len() {
        game.initial_cut_between_players_with_these_indices
            .push(index as u8);
    }

    game.state = crate::GameState::CutInitial;

    Ok(game_process_return::Success::GameStart)
}

fn check_settings_validity(
    settings: crate::settings::GameSettings,
) -> Result<(), game_process_return::Error> {
    // If the variant is one of the two player variants but the VictorDealerOption isn't TwoPlayers
    if (settings.variant == crate::settings::RuleVariant::TwoStandard
        || settings.variant == crate::settings::RuleVariant::TwoFiveCard)
        && settings.victor_dealer_option != crate::settings::VictorDealerOption::TwoPlayers
    {
        return Err(game_process_return::Error::GameStartInvalidConfig(
            game_process_return::ConfigError::VDOIsNotTwoPlayersWhenVariantIsTwoPlayers,
        ));
    }

    // If the variant is three or more players but the VictorDealerOption is TwoPlayers
    if settings.variant != crate::settings::RuleVariant::TwoStandard
        && settings.variant != crate::settings::RuleVariant::TwoFiveCard
        && settings.victor_dealer_option == crate::settings::VictorDealerOption::TwoPlayers
    {
        return Err(game_process_return::Error::GameStartInvalidConfig(
            game_process_return::ConfigError::VDOIsTwoPlayersWhenVariantIsMoreThanTwoPlayers,
        ));
    }

    // If the variant is ThreeCaptain then do not allow the victor_dealer_option to be
    // LastPlaceIsDealer because the partners change so the losers should always draw for first crib
    if settings.variant == crate::settings::RuleVariant::ThreeCaptain
        && settings.victor_dealer_option == crate::settings::VictorDealerOption::LastPlaceIsDealer
    {
        return Err(game_process_return::Error::GameStartInvalidConfig(
            game_process_return::ConfigError::VDOIsNotLoserDrawsForDealerWhenVariantIsThreeCaptain,
        ));
    }

    // If the variant is Captain's Cribbage, then it doesn't really work with lowball; maybe change
    // this in the future but it really seems like the pair has a real advantage were this
    // configuration allowed
    if settings.variant == crate::settings::RuleVariant::ThreeCaptain && settings.is_lowball {
        return Err(game_process_return::Error::GameStartInvalidConfig(
            game_process_return::ConfigError::LowballEnabledWhenVariantIsThreeCaptain,
        ));
    }

    // Ensure underpegging, overpegging, and muggins are only enabled when manual scoring is
    // enabled
    if !settings.is_manual_scoring {
        if settings.is_underpegging {
            return Err(game_process_return::Error::GameStartInvalidConfig(
                game_process_return::ConfigError::UnderpeggingEnabledWhenManualScoringIsDisabled,
            ));
        } else if settings.is_overpegging {
            return Err(game_process_return::Error::GameStartInvalidConfig(
                game_process_return::ConfigError::OverpeggingEnabledWhenManualScoringIsDisabled,
            ));
        } else if settings.is_muggins {
            return Err(game_process_return::Error::GameStartInvalidConfig(
                game_process_return::ConfigError::MugginsEnabledWhenManualScoringIsDisabled,
            ));
        }
    }

    // Ensure muggins is only enabled when underpegging is enabled
    if !settings.is_underpegging && settings.is_muggins {
        return Err(game_process_return::Error::GameStartInvalidConfig(
            game_process_return::ConfigError::MugginsEnabledWhenUnderpeggingIsDisabled,
        ));
    }

    Ok(())
}

fn set_up_players(game: &mut crate::GameImpl) {
    if let Some(settings) = game.settings {
        game.players = match settings.variant {
            crate::settings::RuleVariant::TwoStandard
            | crate::settings::RuleVariant::TwoFiveCard => set_up_no_partners(2),
            crate::settings::RuleVariant::ThreeStandard => set_up_no_partners(3),
            crate::settings::RuleVariant::ThreeCaptain => set_up_three_captain(),
            crate::settings::RuleVariant::FourIndividual => set_up_no_partners(4),
            crate::settings::RuleVariant::FourPairs => set_up_four_pairs(),
            crate::settings::RuleVariant::FiveStandard => set_up_no_partners(5),
            crate::settings::RuleVariant::SixPairs => set_up_six_pairs(),
        };
    }
}

// Return a player with no score and the given partner
fn set_up_player(partner_index: Option<u8>) -> player::Player {
    player::Player {
        back_peg_pos: 0,
        front_peg_pos: 0,
        hand: Vec::with_capacity(6),
        partner_index: partner_index,
    }
}

// The following four functions return a vector of Players to be given to the GameImpl object

fn set_up_no_partners(num_players: u8) -> Vec<player::Player> {
    let mut players = Vec::new();
    for _ in 0..num_players {
        players.push(set_up_player(None));
    }
    players
}

fn set_up_three_captain() -> Vec<player::Player> {
    let mut players = Vec::new();
    for index in 0..3 {
        players.push(match index {
            0 => set_up_player(None),
            1 => set_up_player(Some(2)),
            2 => set_up_player(Some(1)),
            _ => panic!("Invalid index set_up_three_captain"),
        })
    }
    players
}

fn set_up_four_pairs() -> Vec<player::Player> {
    let mut players = Vec::new();
    for index in 0..4 {
        players.push(match index {
            0 => set_up_player(Some(2)),
            1 => set_up_player(Some(3)),
            2 => set_up_player(Some(0)),
            3 => set_up_player(Some(1)),
            _ => panic!("Invalid index set_up_four_pairs"),
        })
    }
    players
}

fn set_up_six_pairs() -> Vec<player::Player> {
    let mut players = Vec::new();
    for index in 0..6 {
        players.push(match index {
            0 => set_up_player(Some(3)),
            1 => set_up_player(Some(4)),
            2 => set_up_player(Some(5)),
            3 => set_up_player(Some(0)),
            4 => set_up_player(Some(1)),
            5 => set_up_player(Some(2)),
            _ => panic!("Invalid index set_up_six_pairs"),
        })
    }
    players
}
