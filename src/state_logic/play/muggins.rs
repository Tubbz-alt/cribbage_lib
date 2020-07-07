#[cfg(test)]
mod test {
    // State that deals with any missed ScoreEvents when muggings is enabled
    mod play_muggins {
        #[test]
        fn no_remaining_score_events_overpegged() {}

        #[test]
        fn no_remaining_score_events_proceed() {}

        #[test]
        fn one_remaining_score_event_overpegged() {}

        #[test]
        fn one_remaining_score_event_underpegged() {}

        #[test]
        fn one_remaining_score_event_correctly_pegged() {}

        #[test]
        fn one_remaining_score_event_proceed() {}

        #[test]
        fn multiple_remaining_score_events_overpegged() {}

        #[test]
        fn multiple_remaining_score_events_completely_underpegged() {}

        #[test]
        fn multiple_remaining_score_events_partially_underpegged() {}

        #[test]
        fn multiple_remaining_score_events_corectly_pegged() {}

        #[test]
        fn multiple_remaining_score_events_proceed() {}

        #[test]
        fn last_card_remaining_underpegged_proceed() {}

        #[test]
        fn thirty_one_remaining_underpegged_proceed() {}
    }
}
