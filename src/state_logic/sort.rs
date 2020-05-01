use crate::game_process_return;

// Honestly this function is so short that I don't really think I have to test it :)
pub(crate) fn process_sort(
    game: &mut crate::GameImpl,
) -> Result<game_process_return::Success, game_process_return::Error> {
    for player in &mut game.players {
        player.hand.sort();
    }
    game.state = crate::GameState::Discard;
    Ok(game_process_return::Success::Sort)
}
