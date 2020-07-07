pub mod wait_for_card;
pub(crate) use wait_for_card::play_card;
pub mod score;
pub(crate) use score::play_score;
pub mod muggins;
pub mod reset_play;

#[cfg(test)]
mod test_util {
    // Helper function to create a GameImpl in the PlayWaitForCard stage of the game
    pub(crate) fn set_up_game(
        variant: crate::settings::RuleVariant,
        is_man: bool,
        is_under: bool,
        is_mug: bool,
    ) -> crate::GameImpl {
        let vdo = {
            if crate::util::return_num_players_for_variant(variant) == 2 {
                crate::settings::VictorDealerOption::TwoPlayers
            } else if variant == crate::settings::RuleVariant::ThreeCaptain {
                crate::settings::VictorDealerOption::CaptainDeals
            } else {
                crate::settings::VictorDealerOption::LosersDrawForDealer
            }
        };
        let mut game = crate::GameImpl::new();
        let settings = crate::settings::GameSettings {
            variant: variant,
            victor_dealer_option: vdo,
            is_manual_scoring: is_man,
            is_underpegging: is_under,
            is_muggins: is_mug,
            is_lowball: false,
        };

        game.is_debug = true;

        crate::state_logic::game_start::game_setup(&mut game, settings).unwrap();

        game.deck = crate::deck::Deck::new();

        crate::state_logic::cut_initial::process_cut(&mut game).unwrap();

        game.deck = crate::deck::Deck::new();

        crate::state_logic::deal::process_deal(&mut game).unwrap();

        crate::state_logic::sort::process_sort(&mut game).unwrap();

        let discard_selecions: Vec<Vec<u8>> =
            match crate::util::return_num_players_for_variant(variant) {
                2 => vec![vec![0, 1], vec![0, 1]],
                3 => vec![vec![0], vec![0], vec![0]],
                4 => vec![vec![0], vec![0], vec![0], vec![0]],
                5 => vec![vec![], vec![0], vec![0], vec![0], vec![0]],
                6 => vec![vec![], vec![0], vec![0], vec![], vec![0], vec![0]],
                _ => panic!(
                "return_num_players_for_variant returned a number not between 2 and 6 inclusive"
                ),
            };
        crate::state_logic::discard::process_discard(&mut game, discard_selecions).unwrap();

        crate::cut_starter_and_nibs_check::process_cut(&mut game).unwrap();

        game
    }
}
