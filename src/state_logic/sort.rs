use crate as game;
use crate::game_process_return;

pub(crate) fn process_sort(
    game: &mut game::GameImpl,
) -> Result<game_process_return::Success, game_process_return::Error> {
    for player in &mut game.players {
        player.hand.sort();
    }
    Ok(game_process_return::Success::Sort)
}
