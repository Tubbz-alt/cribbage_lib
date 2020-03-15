use crate as game;
use crate::game_process_return;

// Cut the starter card from the deck
pub(crate) fn process_cut(
    game: &mut game::GameImpl,
) -> Result<game_process_return::Success, game_process_return::Error> {
    // Sets the starter card variable when debug mode is disabled
    if !game.is_debug {
        game.starter_card = Some(game.deck.deal());
    }

    Ok(game_process_return::Success::StarterCut)
}

// When underpegging is enabled, process whether the dealer calls nibs or not
pub(crate) fn process_nibs(
    game: &mut game::GameImpl,
) -> Result<game_process_return::Success, game_process_return::Error> {
    Ok(game_process_return::Success::NibsCheck)
}

fn ready_for_play() {}
