pub mod deck;
pub mod game_process_return;
pub mod player;
pub mod score;
pub mod settings;

mod state_logic;

mod util;

use state_logic::cut_initial;
use state_logic::cut_starter_and_nibs_check;
use state_logic::deal;
use state_logic::discard;
use state_logic::game_start;
use state_logic::sort;

// Enum sent to the process_turn function to advance the play of the game model
#[derive(Debug, Clone)]
pub enum GameEvent {
    // Event containing the parameters to start the game
    GameSetup(settings::GameSettings),
    // Event containing a set of cards for each player in the game; used for selecting the
    // cards to send to the crib
    DiscardSelection(Vec<Vec<u8>>),
    // Event containing information on the card played during the play phase (either a reference to
    // a card in the player's hand or a Go)
    Play(PlayTurn),
    // Event containing an option of a vector of ScoreEvents for the manual scoring of a hand
    ManScoreSelection(Option<Vec<score::ScoreEvent>>),
    // Event for whether the dealer calls nibs
    Nibs(Option<score::ScoreEvent>),
    // Event for contesting a player who overscores (or underscores in lowball)
    Contest(Option<score::ScoreEvent>),
    // Event for whether a player calls muggins
    Muggins(Option<Vec<score::ScoreEvent>>),
    // Simple event for continuing to the next game state; used when player input is needed such as
    // when a player must cut the deck or simply when the timing of a state change is decided by
    // the program implementing this library
    Confirmation,
    Denial,
}

// The public game object with appropriate abstraction that processes GameEvents
pub struct Game {
    game: GameImpl,
}

impl Game {
    pub fn new() -> Game {
        Game {
            game: GameImpl::new(),
        }
    }

    // Processes the GameEvent objects to progress the model of the game
    pub fn process_event(
        &mut self,
        event: GameEvent,
    ) -> Result<game_process_return::Success, game_process_return::Error> {
        // Simply routes the game object to the right function depending on the game state and the
        // event passed to this function
        match (self.game.state, event) {
            // Accepts a GameSetup  event to continue to CutInitial
            (GameState::GameStart, GameEvent::GameSetup(settings)) => {
                game_start::game_setup(&mut self.game, settings)
            }
            (GameState::GameStart, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::GameSetup,
            ])),

            // Accepts a confirmation event indicating the players' false (still randomized, but players'
            // confirmation does not actually choose a card) choices for cut and continues to Deal
            (GameState::CutInitial, GameEvent::Confirmation) => {
                cut_initial::process_cut(&mut self.game)
            }
            (GameState::CutInitial, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
            ])),

            // Deals the cards to each player after taking confirmation from dealer
            (GameState::Deal, GameEvent::Confirmation) => deal::process_deal(&mut self.game),
            (GameState::Deal, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
            ])),

            // Sorts each player's hand after confirmation allowing the player to see the order in
            // which the cards were dealt before seeing the sorted version
            (GameState::Sort, GameEvent::Confirmation) => sort::process_sort(&mut self.game),
            (GameState::Sort, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
            ])),

            // Removes the chosen card(s) from each hand and places them in the crib
            (GameState::Discard, GameEvent::DiscardSelection(player_discards)) => {
                discard::process_discard(&mut self.game, player_discards)
            }
            (GameState::Discard, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::DiscardSelection,
            ])),

            // Reveals the starter card after confirmation of the player to the dealer's left
            (GameState::CutStarter, GameEvent::Confirmation) => {
                cut_starter_and_nibs_check::process_cut(&mut self.game)
            }
            (GameState::CutStarter, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
            ])),

            // Processes the dealer's choice of whether or not to call nibs when underpegging is
            // disabled
            (GameState::NibsCheck, GameEvent::Nibs(nibs_call)) => {
                cut_starter_and_nibs_check::process_nibs(&mut self.game, nibs_call)
            }
            (GameState::NibsCheck, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Nibs,
            ])),

            /*
            // Takes the input from the active player and plays it
            (GameState::PlayWaitForCard, GameEvent::Play(play)) => {
                Err(game_process_return::Error::UnimplementedState)
            }
            (GameState::PlayWaitForCard, _) => {
                Err(game_process_return::Error::ExpectedEvent(vec![
                    game_process_return::Event::Play,
                ]))
            }

            // Prepares the game for the next PlayGroup or transitions state to show phase
            (GameState::ResetPlay, GameEvent::Confirmation) => {
                Err(game_process_return::Error::UnimplementedState)
            }
            (GameState::ResetPlay, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
            ])),

            // Scores the play phase of the game automatically or manually
            (GameState::PlayScore, GameEvent::Confirmation) => {
                Err(game_process_return::Error::UnimplementedState)
            }
            (GameState::PlayScore, GameEvent::ManScoreSelection(selection)) => {
                Err(game_process_return::Error::UnimplementedState)
            }
            (GameState::PlayScore, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
                game_process_return::Event::ManScoreSelection,
            ])),

            // Processes any calls of muggins for the play phase of the game
            (GameState::PlayMuggins, GameEvent::Confirmation) => {
                Err(game_process_return::Error::UnimplementedState)
            }
            (GameState::PlayMuggins, GameEvent::Muggins(selection)) => {
                Err(game_process_return::Error::UnimplementedState)
            }
            (GameState::PlayMuggins, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
                game_process_return::Event::Muggins,
            ])),
            // Scores the show phase of the game automatically or manually
            (GameState::ShowScore, GameEvent::Confirmation) => game_process_return::Error::UnimplementedState,
            (GameState::ShowScore, GameEvent::ManScoreSelection(selection)) => game_process_return::Error::UnimplementedState,
            (GameState::ShowScore, _) => game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
                game_process_return::Event::ManScoreSelection,
            ]),

            // Processes any call of muggins for the show phase of the game
            (GameState::ShowMuggins, GameEvent::Confirmation) => game_process_return::Error::UnimplementedState,
            (GameState::ShowMuggins, GameEvent::Muggins(selection)) => game_process_return::Error::UnimplementedState,
            (GameState::ShowMuggins, _) => game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
                game_process_return::Event::Muggins,
            ]),

            // Processes the scoring of the crib automatically or manually
            (GameState::CribScore, GameEvent::Confirmation) => game_process_return::Error::UnimplementedState,,
            (GameState::CribScore, GameEvent::ManScoreSelection(selection)) => game_process_return::Error::UnimplementedState,
            (GameState::CribScore, _) => game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
                game_process_return::Event::ManScoreSelection,
            ]),

            // Processes any call of muggins for the crib
            (GameState::CribMuggins, GameEvent::Confirmation) => game_process_return::Error::UnimplementedState,
            (GameState::CribMuggins, GameEvent::Muggins(selection)) => game_process_return::Error::UnimplementedState,
            (GameState::CribMuggins, _) => game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
                game_process_return::Event::Muggins,
            ]),

            /* Processes the end of a game
            (GameState::Win, GameEvent::Confirmation) => Game::win(self, true),
            (GameState::Win, GameEvent::Denial) => Game::win(self, false),

            // TODO Processes the end of a match
            */*/

            // For unexpected GameState
            (_, _) => Err(game_process_return::Error::UnrecognizedState),
        }
    }
}

// Enum for the event sent during the play phase of the game; simply a selection of the card to be
// played or a Go if no card play is possible
#[derive(Debug, Clone, Copy)]
pub enum PlayTurn {
    CardSelected(deck::Card),
    Go,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    // Initializes the Game object based on the settings passed with the GameSetup event
    GameStart,
    // Performs the initial cut to determine the dealer after receiving a Confirmation event
    CutInitial,
    // Deals the cards to each player after receiving a Confirmation event
    Deal,
    // Sorts the players' hands after receiving a Confirmation event
    Sort,
    // Compiles the discards into the crib after receiving a HandIndices event
    Discard,
    // Determine the starter card after receiving a Confirmation event
    CutStarter,
    // Determines whether the dealer should receive points after calling or failing to call nibs
    // with the Nibs event; muggins does not apply for nibs so there is no muggins state here
    NibsCheck,
    // Deals with the process of playing a single card received with a Play event
    PlayWaitForCard,
    // Deals with the scoring of the last card to be played in the PlayGroup automatically with a
    // Confirmation event or manually with a ManScoreSelection event
    PlayScore,
    // Deals with the calling of muggins of the last card to be played in the PlayGroup with a
    // Muggins event
    PlayMuggins,
    // Handles the creation of a new PlayGroup or the transition to the ShowScore state after the
    // play of cards is no longer possible for the current PlayGroup
    ResetPlay,
    // Deals with the automatic or manual scoring of the Play phase with a Confirmation or
    // ManScoreSelection event
    ShowScore,
    // Deals with the calling of muggins for the last hand to be scored with a Muggins event
    ShowMuggins,
    // Sorts the crib after receiving a Confirmation event
    CribSort,
    // Deals with the automatic or manual scoring of the crib with a Confirmation or
    // ManScoreSelection
    CribScore,
    // Deals with the calling of muggins for the crib with a Muggins event
    CribMuggins,
    // State for when a player has won the game; routes back to Deal for the next game of the
    // match or to End
    // and scores, the crib, the starter card, etc.
    Win,
    // State for the end of play
    End,
}

// Object representing the cards played in one of the groups of 31 or less
#[derive(Debug, Clone)]
pub struct PlayGroup {
    total: u8,
    cards: Vec<deck::Card>,
}

// Game object implementation with public variables such as to allow library functions to
// directly modify everything
struct GameImpl {
    // Option for when ther is no configuration during the GameStart state
    pub settings: Option<settings::GameSettings>,

    pub state: GameState,

    pub players: Vec<player::Player>,

    // At most 29 cards are used so the deck should never run out of cards; public for debug
    // purposes only
    pub deck: deck::Deck,

    // The player index of the current dealer
    pub index_dealer: Option<u8>,

    // The player index of who is currently playing or scoring
    pub index_active: Option<u8>,

    // The player index of who last made a valid move while playing
    pub last_player_index: Option<u8>,

    pub starter_card: Option<deck::Card>,

    pub crib: Vec<deck::Card>,

    // The cards played during the play phase; each play group contains up to 13 cards which total
    // no more than 31
    pub play_groups: Vec<PlayGroup>,

    // Vector to hold the ScoreEvents remaining for muggins
    pub remaining_score_events: Vec<score::ScoreEvent>,

    // Vector to hold the invalid ScoreEvents when overpegging is enabled
    pub overpegged_score_events: Vec<score::ScoreEvent>,

    // Whether to reset the play phase of the game for when a 31 score is present
    pub reset_play: bool,

    // The players who are cutting for first deal; it will generally start with all players and
    // reduce based on which players tie; if there are no ties than the player with the lowest card
    // value is the first dealer of the game
    pub initial_cut_between_players_with_these_indices: Vec<u8>,

    // When active the deck will not reset itself such that one can manually enter values into the
    // deck
    pub is_debug: bool,
}

impl GameImpl {
    pub fn new() -> GameImpl {
        GameImpl {
            crib: Vec::new(),
            deck: deck::Deck::new(),
            index_active: None,
            index_dealer: None,
            last_player_index: None,
            overpegged_score_events: Vec::new(),
            play_groups: Vec::new(),
            players: Vec::new(),
            remaining_score_events: Vec::new(),
            reset_play: false,
            settings: None,
            starter_card: None,
            state: GameState::GameStart,
            initial_cut_between_players_with_these_indices: Vec::new(),
            is_debug: false,
        }
    }

    /*
    // Initializes the variables used in the PlayWaitForCard state; function implemented in
    // NibsCheck when is_manual_scoring or in CutStarter otherwise
    fn ready_for_play(&mut self) {
        self.index_active = (self.index_dealer + 1) % self.players.len() as u8;
        self.state = GameState::PlayWaitForCard;
    }

    // Waits for confirmation to cut the starter card from the deck; leads to NibsCheck or
    // PlayWaitForCard/Win depending on whether or not is_manual_scoring is true
    fn process_starter(&mut self) -> Result<&str, &str> {
        // Actually sets the starter card when debug is disabled
        if !self.is_debug {
            self.starter_card = self.deck.deal();
        }

        // Checks for nibs when manual scoring or underpegging is disabled
        if !self.is_manual_scoring || !self.is_underpegging {
            // Checks if the starter card is a Jack for nibs
            if self.starter_card.value == deck::CardValue::Jack {
                self.players[self.index_dealer as usize].change_score(2);
                // If the dealer's score is 121 or higher because of the nibs, go to win state
                if self.players[self.index_dealer as usize].front_peg_pos >= 121 {
                    self.state = GameState::Win;
                    return Ok("Starter card cut; nibs and win");
                }

                //If the score is not 121 or higher
                Game::ready_for_play(self);
                return Ok("Starter card cut; nibs");
            }

            // If the cut was not a jack, but automatic scoring is still disabled
            Game::ready_for_play(self);
            return Ok("Starter card cut; no nibs");
        }

        // When automatic scorind is disabled
        self.state = GameState::NibsCheck;
        Ok("Starter card cut")
    }

    // Logic for when the dealer calls Nibs; leads to PlayWaitForCard or Win
    fn check_nibs(&mut self, input: Option<score::ScoreEvent>) -> Result<&str, &str> {
        match input {
            // If there was a None event sent
            None => {
                Game::ready_for_play(self);
                Ok("Nibs not called")
            }
            Some(score_event) => {
                // If the ScoreEvent sent has a Nibs type
                if score_event.score_type == score::ScoreType::Play(score::PlayScoreType::Nibs) {
                    // If the Nibs call is actually valid
                    if self.starter_card.value == deck::CardValue::Jack {
                        self.players[self.index_dealer as usize].change_score(2);
                        // Check if nibs bumps player over 121
                        if self.players[self.index_dealer as usize].front_peg_pos >= 121 {
                            self.state = GameState::Win;
                            Ok("Nibs call and win")
                        } else {
                            Game::ready_for_play(self);
                            Ok("Nibs call")
                        }
                    } else {
                        if !self.is_overpegging {
                            Err("Invalid nibs call")
                        } else {
                            Err("TODO")
                        }
                    }
                } else {
                    Err("Invalid ScoreEvent at NibsCheck")
                }
            }
        }
    }

    // Returns whether or not a given card has already been played this round
    pub fn has_card_been_played(&mut self, card_in_question: deck::Card) -> bool {
        for card_group in &self.play_groups {
            if card_group.cards.contains(&card_in_question) {
                return true;
            }
        }
        false
    }

    // Processes the play phase of the game; accepts input of either a card or a go in the PlayTurn
    // enum; leads to the Win, ResetPlay, and PlayScore
    fn play_card(&mut self, choice: PlayTurn) -> Result<&str, &str> {
        match choice {
            PlayTurn::Go => {
                // Checks to see if the player could have played a card instead of going
                for card in self.players[self.index_active as usize].hand.clone() {
                    if !Game::has_card_been_played(self, card)
                        && deck::return_play_value(card) + self.play_groups.last().unwrap().total
                            <= 31
                    {
                        return Err("Player must play card if possible; go invalid");
                    }
                }

                // If the active player is the last player to have played a card or when all players
                // have gone
                if self.index_active == self.last_player_index {
                    // Point for the last card
                    // Potential FIXME: If it double scores a player for last point, make it check
                    // if there are cards in the last PlayGroup
                    self.players[self.index_active as usize].change_score(1);

                    // If the point causes the player to reach 121 or not
                    if self.players[self.index_active as usize].front_peg_pos >= 121 {
                        self.state = GameState::Win;
                        return Ok("Player takes last point and wins");
                    }
                    // Else reset play
                    else {
                        self.state = GameState::ResetPlay;
                        return Ok("Player takes last point");
                    }
                }

                // Changes the active player index if go is valid
                self.index_active = (self.index_active + 1) % self.players.len() as u8;
                Ok("Player goes")
            }

            PlayTurn::CardSelected(current_play) => {
                // Cheacks if the current_play is an exact match to any of the card references in
                // the played_cards vector checking whether or not that card is a valid selection
                if Game::has_card_been_played(self, current_play) {
                    return Err("Last card selected has already been played");
                }

                // Checks if the current play brings the current total over 31 checking whether or
                // not that card is a valid selection
                if self.play_groups.last().unwrap().total + deck::return_play_value(current_play)
                    > 31
                {
                    return Err("Last card selected brings total over 31");
                }

                // Checks that the card being played is actually in the player's hand
                let mut is_card_in_hand = false;
                for card in &self.players[self.index_active as usize].hand {
                    if *card == current_play {
                        is_card_in_hand = true;
                    }
                }
                if !is_card_in_hand {
                    return Err("Card played must be in the active player's hand");
                }

                // Set the last player to place a valid card to the player who has just played
                self.last_player_index = self.index_active;

                // Adds a copy of the current_play to the current played_cards group and
                // updates the total
                let temp = self.play_groups.len() - 1;
                self.play_groups[temp].cards.push(current_play);
                self.play_groups[temp].total += deck::return_play_value(current_play);

                // Changes state to PlayScore
                self.state = GameState::PlayScore;
                Ok("Player places card")
            }
        }
    }

    // Changes state to ShowScore when there are no cards remaining, or back to
    // PlayWaitForCard with a new PlayGroup after recieving a Confirmation event
    fn reset_play(&mut self) -> Result<&str, &str> {
        // Determines whether there a cards left to be played
        let mut is_cards_remaining = false;
        // TODO Clean up
        // Uses clone of self.players because I have no idea how references work in Rust :)
        let players_temp_clone = self.players.clone();
        for player in players_temp_clone {
            for card in player.hand {
                if !Game::has_card_been_played(self, card) {
                    is_cards_remaining = true;
                }
            }
        }

        // If there are no cards remaining, set the index_active to the player to the left of the
        // dealer and go to the scoring state
        if is_cards_remaining == false {
            self.index_active = (self.index_dealer + 1) % self.players.len() as u8;
            self.state = GameState::ShowScore;
            return Ok("No cards remaining, proceed to scoring");
        }

        // If there are cards remaining, create the next PlayGroup
        self.play_groups.push(PlayGroup {
            cards: Vec::new(),
            total: 0,
        });
        // FIXME: Probably fucks up here
        self.index_active = (self.index_active + 1) % self.players.len() as u8;
        self.state = GameState::PlayWaitForCard;
        Ok("Ready for next PlayGroup")
    }

    // Processes the actual scoring of the play or show phase
    // Leads to Win, PlayMuggins, ResetPlay, or PlayWaitForCard when starting from PlayScore
    // Leads to Win, ShowMuggins, ShowScore, or CribScore when starting from ShowScore
    // Leads to Win, CribMuggins, or Deal when starting from CribScore
    fn score(&mut self, selection: Option<Vec<score::ScoreEvent>>) -> Result<&str, &str> {
        // The list of scores corresponding to the actual maximum value
        // TODO Change play_score arg from usize to u8
        if self.state == GameState::PlayScore {
            self.remaining_score_events =
                score::play_score(self.index_active as usize, self.play_groups.last().unwrap());
        } else if self.state == GameState::ShowScore {
            self.remaining_score_events = score::score_hand(
                self.index_active,
                self.players[self.index_active as usize].hand.clone(),
                self.starter_card,
            );
        } else if self.state == GameState::CribScore {
            self.remaining_score_events =
                score::score_hand(self.index_dealer, self.crib.clone(), self.starter_card);
        } else {
            return Err("Unexpected state in play_score");
        }

        // Flag set when a valid ThirtyOne PlayScoreType is present; indicates that the state
        // should change to ResetPlay instead of PlayWaitForCard
        self.reset_play = false;

        // Manual scoring
        // TODO: Overpegging; remove 121 checks when overpegging is enabled and check once after
        // scores are contested
        if self.is_manual_scoring {
            match selection {
                // If no scores are sent
                None => {
                    // If underpegging is disabled and a score event is present in
                    // remaining_score_events, return an error; otherwise the None is valid
                    if self.remaining_score_events.len() != 0 && !self.is_underpegging {
                        return Err(
                            "Must enter the correct ScoreEvents when underpegging is disabled",
                        );
                    }
                }
                // If scores are sent
                Some(scores) => {
                    // Final score change to apply at the end
                    let mut score_change = 0;

                    // Create a list of every valid and invalid ScoreEvent in the selection
                    let mut valid_scores: Vec<score::ScoreEvent> = Vec::new();
                    let mut invalid_scores: Vec<score::ScoreEvent> = Vec::new();
                    // TODO Allow invalid scores that are contained by valid scores eg a run of
                    // three in a run of four (but not two runs of three) or a double in a triple
                    for score in &scores {
                        let mut is_score_in_correct_scores = false;
                        for correct_score in &self.remaining_score_events {
                            if *score == *correct_score {
                                // If there is a flush of four in the crib, don't count it
                                let mut skip = false;
                                if self.state == GameState::CribScore {
                                    match &correct_score.score_type {
                                        score::ScoreType::Show(show_score_type) => {
                                            match show_score_type {
                                                score::ShowScoreType::FourFlush(_) => {
                                                    skip = true;
                                                }
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }

                                if !skip {
                                    valid_scores.push(score.clone());
                                }

                                is_score_in_correct_scores = true;

                                // If there is a valid 31 ScoreEvent, set reset_play to true
                                if score.score_type
                                    == score::ScoreType::Play(score::PlayScoreType::ThirtyOne)
                                {
                                    self.reset_play = true;
                                }
                            }
                        }
                        if !is_score_in_correct_scores {
                            invalid_scores.push(score.clone());
                            // Set reset_play to true if there's an invalid 31 ScoreEvent when
                            // overpegging is enabled; can be corrected if another player contests
                            // the score
                            if self.is_overpegging
                                && score.score_type
                                    == score::ScoreType::Play(score::PlayScoreType::ThirtyOne)
                            {
                                self.reset_play = true;
                            }
                        }
                    }

                    // If overpegging is enabled
                    if self.is_overpegging {
                        // TODO Implement penalty for overpegging; opponent must catch
                        // Add each invalid score to a list to check opponents validity call
                        // against
                        return Err("TODO: Overpegging");
                    } else {
                        // Do not allow invalid scores when overpegging is disabled
                        if invalid_scores.len() > 0 {
                            return Err("Invalid ScoreEvent when overpegging is disabled");
                        }
                    }

                    // If underpegging is enabled
                    if self.is_underpegging {
                        // Remove every score in valid_scores from remaining_score_events then
                        self.remaining_score_events.retain({
                            |remaining_score| {
                                // Do not retain the element if a valid score matching the
                                // remaining score is found
                                for valid_score in &valid_scores {
                                    if *valid_score == *remaining_score {
                                        return false;
                                    }
                                }
                                // Retain the element if the valid score is not found -- if the
                                // correct score was not in the selection
                                true
                            }
                        });
                        // Add points for the valid_scores in the selection
                        let mut score_sum = 0;
                        for score in valid_scores {
                            score_sum += score.point_value;
                        }

                        score_change += score_sum;
                    }
                    // If underpegging is disabled
                    else {
                        // If the number of valid scores equals the number of complete valid
                        // scores; whether or not all ScoreEvents have been accounted for
                        if valid_scores.len() == self.remaining_score_events.len() {
                            self.remaining_score_events.clear();
                            // Basically do automatic scoring if all the ScoreEvents are
                            // present
                            let mut score_sum = 0;
                            for score in valid_scores {
                                score_sum += score.point_value;
                            }

                            score_change += score_sum;
                        } else {
                            return Err("Incomplete score selection when underpegging is disabled");
                        }
                    }

                    self.players[self.index_active as usize].change_score(score_change as i8);

                    if self.players[self.index_active as usize].front_peg_pos >= 121 {
                        self.state = GameState::Win;
                        return Ok("Scoring complete and win");
                    }
                }
            }
        }
        // Automatic scoring; simply sums the correct amount
        else {
            let mut score_sum = 0;
            for score in &self.remaining_score_events {
                score_sum += score.point_value;
                if score.score_type == score::ScoreType::Play(score::PlayScoreType::ThirtyOne) {
                    self.reset_play = true;
                }
            }

            self.players[self.index_active as usize].change_score(score_sum as i8);

            // Overpegging must be disabled when automatic scoring is enabled
            if self.players[self.index_active as usize].front_peg_pos >= 121 {
                self.state = GameState::Win;
                return Ok("Play scoring complete and win");
            }
        }

        // TODO OpponnentContest states when overpegging is enable
        if self.state == GameState::PlayScore {
            if self.is_overpegging {
                return Err("TODO");
            } else if self.is_muggins {
                self.state = GameState::PlayMuggins;
            } else {
                self.index_active = (self.index_active + 1) % self.players.len() as u8;
                if !self.reset_play {
                    self.state = GameState::PlayWaitForCard;
                } else {
                    self.state = GameState::ResetPlay;
                }
            }
        } else if self.state == GameState::ShowScore {
            if self.is_overpegging {
                return Err("TODO");
            } else if self.is_muggins {
                self.state = GameState::ShowMuggins;
            } else if self.index_active == self.index_dealer {
                self.state = GameState::CribScore;
            } else {
                self.index_active = (self.index_active + 1) % self.players.len() as u8;
            }
        } else {
            if self.is_overpegging {
                return Err("TODO");
            } else if self.is_muggins {
                self.state = GameState::CribMuggins;
            } else {
                self.index_dealer = (self.index_dealer + 1) % self.players.len() as u8;
                self.state = GameState::Deal;
            }
        }

        Ok("Scoring complete")
    }

    // Processes any muggins calls for any muggins state
    // Leads to Win, ResetPlay, or PlayWaitForCard when starting from PlayMuggins
    // Leads to Win, ShowScore, or CribScore when starting from ShowMuggins
    // Leads to Win or Deal when starting from CribMuggins
    fn muggins(&mut self, selection: Option<Vec<score::ScoreEvent>>) -> Result<&str, &str> {
        match selection {
            None => {
                // Prepares to return to the game
                self.index_active = (self.index_active + 1) % self.players.len() as u8;
                if !self.reset_play {
                    self.state = GameState::PlayWaitForCard;
                } else {
                    self.state = GameState::ResetPlay;
                }
                Ok("No muggins selection")
            }
            Some(muggins_selections) => {
                // Creates a list of score values correspondeing to each player index such as to
                // total the ScoreEvent point values
                let mut score_changes: Vec<i8> = Vec::with_capacity(self.players.len());
                // Potential FIXME; off by one error
                for _i in 0..self.players.len() {
                    score_changes.push(0);
                }

                // Create a list of which muggins selections are valid and which are invalid
                let mut valid_scores: Vec<score::ScoreEvent> = Vec::new();
                let mut invalid_scores: Vec<score::ScoreEvent> = Vec::new();
                for selection_event in &muggins_selections {
                    let mut is_selection_event_in_remaining = false;
                    for remaining_event in &self.remaining_score_events {
                        if remaining_event.score_type == selection_event.score_type
                            && remaining_event.point_value == selection_event.point_value
                        {
                            valid_scores.push(selection_event.clone());
                            is_selection_event_in_remaining = true;
                            if remaining_event.score_type
                                == score::ScoreType::Play(score::PlayScoreType::ThirtyOne)
                            {
                                self.reset_play = true;
                            }
                        }
                    }
                    if !is_selection_event_in_remaining {
                        invalid_scores.push(selection_event.clone());
                    }
                }

                // TODO If overpegging is enabled allow invalid scores
                if self.is_overpegging {
                    return Err("TODO");
                } else {
                    // Disallow invalid scores when overpegging is disabled
                    if invalid_scores.len() > 0 {
                        return Err("Invalid muggins selection");
                    }
                    // Add points for the proper player based on the index given in the
                    // ScoreEvent
                    else {
                        for score in valid_scores {
                            score_changes[score.player_index] += score.point_value as i8;
                        }
                    }
                }

                // Executes the total score changes for each player
                // TODO Rewrite to process muggins calls in the order in which they are sent such
                // that scores are checked appropriately; generally figure out how muggins works
                // with more than two players
                for (index, score_change) in score_changes.iter().enumerate() {
                    if *score_change != 0 {
                        self.players[index].change_score(*score_change);
                        if self.players[index].front_peg_pos >= 121 {
                            self.state = GameState::Win;
                            return Ok("Muggins score and win");
                        }
                    }
                }

                if self.state == GameState::PlayMuggins {
                    // Leads to ResetPlay or PlayWaitForCard when starting from PlayMuggins
                    self.index_active = (self.index_active + 1) % self.players.len() as u8;
                    if !self.reset_play {
                        self.state = GameState::PlayWaitForCard;
                    } else {
                        self.state = GameState::ResetPlay;
                    }
                } else if self.state == GameState::ShowMuggins {
                    // Leads to ShowScore or CribScore when starting from ShowMuggins
                    if self.index_active != self.index_dealer {
                        self.index_active = (self.index_active + 1) % self.players.len() as u8;
                        self.state = GameState::ShowScore;
                    } else {
                        self.state = GameState::CribScore;
                    }
                } else if self.state == GameState::CribMuggins {
                    // Leads to Deal when starting from CribMuggins
                    self.index_dealer = (self.index_dealer + 1) % self.players.len() as u8;
                    self.state = GameState::Deal;
                }

                return Ok("Muggins complete");
            }
        }
    }

    // Deals with the winning of a game and whether or not to prepare for another game
    fn win(&mut self, is_playing_again: bool) -> Result<&str, &str> {
        // TODO, just ends the game here
        self.state = GameState::End;
        if is_playing_again {
            return Err("TODO Play again");
        }

        for player in &self.players {
            if player.front_peg_pos >= 121 {
                return Ok(&player.username);
            }
        }

        Err("Win event when no player has won")
    }

    // Processes the GameEvent objects to progress the model of the game
    pub fn process_event(
        &mut self,
        event: GameEvent,
    ) -> Result<game_process_return::Success, game_process_return::Error> {
        // Simply routes the game object to the right function depending on the game state and the
        // event passed to this function
        match (self.state, event) {
            // Accepts a GameSetup  event to continue to CutInitial
            (GameState::GameStart, GameEvent::GameSetup(settings)) => {
                game_start::game_setup(self, settings)
            }
            (GameState::GameStart, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::GameSetup,
            ])),

            // Accepts a confirmation event indicating the players' false (still randomized, but players'
            // confirmation does not actually choose a card) choices for cut and continues to Deal
            (GameState::CutInitial, GameEvent::Confirmation) => cut_initial::process_cut(self),
            (GameState::CutInitial, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
            ])),

            // Deals the cards to each player after taking confirmation from dealer
            (GameState::Deal, GameEvent::Confirmation) => deal::process_deal(self),
            (GameState::Deal, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
            ])),

            // Sorts each player's hand after confirmation allowing the player to see the order in
            // which the cards were dealt before seeing the sorted version
            (GameState::Sort, GameEvent::Confirmation) => sort::process_sort(self),
            (GameState::Sort, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::Confirmation,
            ])),

            // Removes the chosen card(s) from each hand and places them in the crib
            (GameState::Discard, GameEvent::DiscardSelection(player_discards)) => {
                discard::process_discard(self, player_discards)
            }
            (GameState::Discard, _) => Err(game_process_return::Error::ExpectedEvent(vec![
                game_process_return::Event::DiscardSelection,
            ])),

            // Reveals the starter card after confirmation of the player to the dealer's left
            (GameState::CutStarter, GameEvent::Confirmation) => Game::process_starter(self),
            (GameState::CutStarter, _) => Err("Expected Confirmation event to CutStarter"),

            // Processes the dealer's choice of whether or not to call nibs
            (GameState::NibsCheck, GameEvent::Nibs(nibs_call)) => Game::check_nibs(self, nibs_call),
            (GameState::NibsCheck, _) => Err("Expected Nibs event to NibsCheck"),

            // Takes the input from the active player and plays it
            (GameState::PlayWaitForCard, GameEvent::Play(play)) => Game::play_card(self, play),
            (GameState::PlayWaitForCard, _) => Err("Expected Play event to PlayWaitForCard"),

            // Prepares the game for the next PlayGroup or transitions state to show phase
            (GameState::ResetPlay, GameEvent::Confirmation) => Game::reset_play(self),
            (GameState::ResetPlay, _) => Err("Expected Confirmation event to ResetPlay"),

            // Scores the play phase of the game automatically or manually
            (GameState::PlayScore, GameEvent::Confirmation) => Game::score(self, None),
            (GameState::PlayScore, GameEvent::ManScoreSelection(selection)) => {
                Game::score(self, selection)
            }
            (GameState::PlayScore, _) => {
                Err("Expected Confirmation or ManScoreSelection event to PlayAutoScore")
            }

            // Processes any calls of muggins for the play phase of the game
            (GameState::PlayMuggins, GameEvent::Confirmation) => Game::muggins(self, None),
            (GameState::PlayMuggins, GameEvent::Muggins(selection)) => {
                Game::muggins(self, selection)
            }
            (GameState::PlayMuggins, _) => {
                Err("Expected Confirmation of Muggins event to PlayMuggins")
            }

            // Scores the show phase of the game automatically or manually
            (GameState::ShowScore, GameEvent::Confirmation) => Game::score(self, None),
            (GameState::ShowScore, GameEvent::ManScoreSelection(selection)) => {
                Game::score(self, selection)
            }
            (GameState::ShowScore, _) => {
                Err("Expected Confirmation or ManScoreSelection event to ShowScore")
            }

            // Processes any call of muggins for the show phase of the game
            (GameState::ShowMuggins, GameEvent::Confirmation) => Game::muggins(self, None),
            (GameState::ShowMuggins, GameEvent::Muggins(selection)) => {
                Game::muggins(self, selection)
            }
            (GameState::ShowMuggins, _) => {
                Err("Expected Confirmation or Muggins event to ShowMuggins")
            }

            // Processes the scoring of the crib automatically or manually
            (GameState::CribScore, GameEvent::Confirmation) => Game::score(self, None),
            (GameState::CribScore, GameEvent::ManScoreSelection(selection)) => {
                Game::score(self, selection)
            }
            (GameState::CribScore, _) => {
                Err("Expected Confirmation of ManScoreSelection event to CribScore")
            }

            // Processes any call of muggins for the crib
            (GameState::CribMuggins, GameEvent::Confirmation) => Game::muggins(self, None),
            (GameState::CribMuggins, GameEvent::Muggins(selection)) => {
                Game::muggins(self, selection)
            }
            (GameState::CribMuggins, _) => {
                Err("Expected Confirmation or Muggins event to CribMuggins")
            }

            // Processes the end of a game
            (GameState::Win, GameEvent::Confirmation) => Game::win(self, true),
            (GameState::Win, GameEvent::Denial) => Game::win(self, false),

            // TODO Processes the end of a match

            // For unexpected GameState
            (_, _) => Err("Unrecognized GameState"),
        }
    }
    */
}
