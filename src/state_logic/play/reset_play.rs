// State that will either add a new, empty member to play_groups and go back to PlayWaitForCard
// or transition the game to the ShowScore state

#[cfg(test)]
mod test {
    #[test]
    fn cards_remaining_back_to_play_wait_for_card() {}

    #[test]
    fn no_cards_remaining_to_show_score() {}

    #[test]
    fn cards_remaining_two_five_cargd_to_show_score() {}
}
