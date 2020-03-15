use crate as game;
use crate::deck;
use crate::game_process_return;

pub(crate) fn process_discard(
    game: &mut game::GameImpl,
    discard_indices_group: Vec<Vec<u8>>,
) -> Result<game_process_return::Success, game_process_return::Error> {
    let discard_errors = check_discard_validity(game, &discard_indices_group);

    if discard_errors.len() > 0 {
        Err(game_process_return::Error::DiscardErrors(discard_errors))
    } else {
        execute_discard(game, discard_indices_group);
        Ok(game_process_return::Success::Discard)
    }
}

fn check_discard_validity(
    game: &mut game::GameImpl,
    discard_indices_group: &Vec<Vec<u8>>,
) -> Vec<game_process_return::DiscardError> {
    let mut errors: Vec<game_process_return::DiscardError> = Vec::new();
    for (player_index, discards) in discard_indices_group.iter().enumerate() {
        if let Some(settings) = game.settings {
            let error: Option<game_process_return::DiscardError> = match settings.variant {
                game::settings::RuleVariant::TwoStandard => {
                    check_two_card_validity(player_index as u8, discards, false)
                }
                game::settings::RuleVariant::TwoFiveCard => {
                    check_two_card_validity(player_index as u8, discards, true)
                }
                game::settings::RuleVariant::ThreeStandard => {
                    check_three_or_four_card_validity(player_index as u8, discards.to_vec())
                }
                game::settings::RuleVariant::ThreeCaptain => {
                    check_three_or_four_card_validity(player_index as u8, discards.to_vec())
                }
                game::settings::RuleVariant::FourIndividual => {
                    check_three_or_four_card_validity(player_index as u8, discards.to_vec())
                }
                game::settings::RuleVariant::FourPairs => {
                    check_three_or_four_card_validity(player_index as u8, discards.to_vec())
                }
                game::settings::RuleVariant::FiveStandard => check_five_card_validity(
                    player_index as u8,
                    discards.to_vec(),
                    game.index_dealer.unwrap(),
                ),
                game::settings::RuleVariant::SixPairs => check_six_card_validity(
                    player_index as u8,
                    discards.to_vec(),
                    game.index_dealer.unwrap(),
                    game.players[game.index_dealer.unwrap() as usize]
                        .partner_index
                        .unwrap(),
                ),
            };

            if let Some(error) = error {
                errors.push(error);
            }
        }
    }
    errors
}

fn check_two_card_validity(
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

fn check_three_or_four_card_validity(
    player_index: u8,
    discard_indices: Vec<u8>,
) -> Option<game_process_return::DiscardError> {
    if discard_indices.len() != 1 {
        Some(game_process_return::DiscardError::OneCardIsDiscardedWithThreePlayers(player_index))
    } else {
        if discard_indices[0] > 4 {
            Some(game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMoreCards(player_index))
        } else {
            None
        }
    }
}

fn check_five_card_validity(
    player_index: u8,
    discard_indices: Vec<u8>,
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
            Some(game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMoreCards(player_index))
        } else {
            None
        }
    }
}

fn check_six_card_validity(
    player_index: u8,
    discard_indices: Vec<u8>,
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
            Some(game_process_return::DiscardError::IndicesAreBetween0And4InclusiveWithThreeOrMoreCards(player_index))
        } else {
            None
        }
    }
}

fn execute_discard(game: &mut game::GameImpl, discard_indices_group: Vec<Vec<u8>>) {
    let mut selected_cards: Vec<deck::Card> = Vec::new();
    for (player_index, discard_indices) in discard_indices_group.iter().enumerate() {
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
    }
}
