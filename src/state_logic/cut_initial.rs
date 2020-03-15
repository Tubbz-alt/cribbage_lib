use crate as game;
use crate::deck;
use crate::game_process_return;

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
    compare_cards(game);

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
        // first game of the match it'll be only the players who lost
        if game
            .initial_cut_between_players_with_these_indices
            .contains(&(index as u8))
        {
            player.hand.push(game.deck.deal());
        }
    }
}

fn compare_cards(game: &mut game::GameImpl) {
    let mut lowest_value = 14;
    let mut player_indices_with_cuts_of_lowest_values: Vec<u8> = Vec::with_capacity(4);

    for (index, player) in game.players.iter().enumerate() {
        if game
            .initial_cut_between_players_with_these_indices
            .contains(&(index as u8))
        {
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

    game.initial_cut_between_players_with_these_indices = player_indices_with_cuts_of_lowest_values;
}
