use crate::game_process_return;

#[cfg(test)]
mod test {
    // Helper function to create a GameImpl in the PlayWaitForCard stage of the game
    fn set_up_game() -> crate::GameImpl {
        crate::GameImpl::new()
    }

    // State that handles a PlayTurn object (enum with either the card played or a go); must check
    // that the play is valid, adds the card to the last member of the play_groups variable (what
    // I'm calling the Vec<Vec<Cards>> that keeps track of every card played in groups that have a
    // maximum value of 31), and handles scoring with automatic scoring
    mod play_wait_for_card {}

    // State that processes a ScoreEvent for when manual scoring is enabled
    mod play_score {}

    // State that deals with any missed ScoreEvents when muggings is enabled
    mod play_muggins {}

    // State that will either add a new, empty member to play_groups and go back to PlayWaitForCard
    // or transition the game to the ShowScore state
    mod reset_play {}
}

fn play_card(game: &mut crate::GameImpl, play_turn: crate::PlayTurn) {}
