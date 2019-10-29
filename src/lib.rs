pub mod deck;
pub mod score;
#[cfg(test)]
mod tests;

use std::convert::TryFrom;
use std::mem;

// Object representing a specific player in the game; keeps track of score and the hand
#[derive(Debug, Clone)]
pub struct Player {
    pub username: String,
    pub back_peg_pos: u8,
    pub front_peg_pos: u8,
    pub hand: Vec<deck::Card>,
}

impl Player {
    fn change_score(&mut self, change: i8) {
        // Move the back peg up and then the front peg forward to simulate moving the back peg
        // forward past the front peg when the change is positive
        if change > 0 {
            self.back_peg_pos = self.front_peg_pos;
            self.front_peg_pos += change as u8;
        } else if change < 0 {
            // TODO with score penalties for overpegging; ensure check to make sure score does not
            // go below zero
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

// Enum sent to the process_turn function to advance the play of the game model
#[derive(Debug, Clone)]
pub enum GameEvent {
    // Event containing the parameters to start the game
    GameSetup {
        input_player_names: Vec<String>,
        input_manual: bool,
        input_underscoring: bool,
        input_muggins: bool,
        input_overscoring: bool,
    },
    // Event containing a set of hand indices for each player in the game; used for selecting the
    // cards to send to the crib
    DiscardSelection(Vec<Vec<deck::Card>>),
    // Event containing information on the card played during the play phase (either a reference to
    // a card in the player's hand or a Go)
    Play(PlayTurn),
    // Event containing an option of a vector of ScoreEvents for the manual scoring of a hand
    ManScoreSelection(Option<Vec<score::ScoreEvent>>),
    // Event for whether the dealer calls nibs
    Nibs(Option<score::ScoreEvent>),
    // Event for whether a player calls muggins
    Muggins(Option<Vec<score::ScoreEvent>>),
    // Simple event for continuing to the next game state; used when player input is needed such as
    // when a player must cut the deck or simply when the timing of a state change is decided by
    // the program implementing this library
    Confirmation,
    Declination,
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

// Object representing the logging of game events
enum GameLog<T> {
    EventInput(GameEvent),
    StateResult {
        name: String,
        result: Result<String, String>,
    },
    WatchedVarChange {
        name: String,
        initial: T,
        current: T,
    },
    LogOut(String),
}

// Game object which tracks game variables and processes GameEvents
pub struct Game {
    // Game options; given with the player names in the GameSetup event
    is_manual_scoring: bool,
    is_underpegging: bool,
    is_muggins: bool,
    is_overpegging: bool,

    // Log of game events for debug purposes
    // log: Vec<GameLog>,

    // The current GameState that the game is in
    pub state: GameState,

    // A vector of two to five Player objects
    pub players: Vec<Player>,

    // The deck of cards to be used in the game; at most 25 cards are used so the deck should never
    // run out of cards; public for debug purposes only
    deck: deck::Deck,

    // The player index of the current dealer
    pub index_dealer: u8,

    // The player index of who is currently playing or scoring
    pub index_active: u8,

    // The player index of who last made a valid move while playing
    last_player_index: u8,

    // The card that is cut and shared by all players' hands
    pub starter_card: deck::Card,

    // The extra hand given to the dealer after scoring their hand
    pub crib: Vec<deck::Card>,

    // The cards played during the play phase; each play group contains between 3 and 13 cards and
    // the maximum number of cards in the play phase total is 24
    pub play_groups: Vec<PlayGroup>,

    // Vector to hold the ScoreEvents remaining for muggins
    remaining_score_events: Vec<score::ScoreEvent>,

    // Vector to hold the invalid ScoreEvents when overpegging is enabled
    overpegged_score_events: Vec<score::ScoreEvent>,

    // Whether to reset the play phase of the game for when a 31 score is present
    reset_play: bool,

    // When active the deck will not reset itself such that one can manually enter values into the
    // deck
    pub is_debug: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            crib: Vec::new(),
            deck: deck::new_deck(),
            index_active: 0,
            index_dealer: 0,
            is_manual_scoring: false,
            is_muggins: false,
            is_overpegging: false,
            is_underpegging: false,
            last_player_index: 0,
            overpegged_score_events: Vec::new(),
            play_groups: Vec::new(),
            players: Vec::new(),
            remaining_score_events: Vec::new(),
            reset_play: false,
            starter_card: deck::Card {
                value: deck::CardValue::Ace,
                suit: deck::CardSuit::Spades,
            },
            state: GameState::GameStart,
            is_debug: false,
        }
    }
    // Sets up the game object with the parameters given in the GameSetup event; leads to
    // CutInitial
    fn game_setup(
        &mut self,
        player_names: Vec<String>,
        set_manual_scoring: bool,
        set_underpegging: bool,
        set_muggins: bool,
        set_overpegging: bool,
    ) -> Result<&str, &str> {
        // Checks that number of players is valid
        if player_names.len() > 5 || player_names.len() < 2 {
            return Err("Expected GameSetup with 2 to 5 player names");
        }

        // Checks that the options set are valid eg. that set_manual_scoring is true when
        // set_allow_underpegging is true
        if set_underpegging && !set_manual_scoring {
            return Err("Manual scoring must be enabled for underpegging to be enabled");
        }

        if set_muggins && (!set_manual_scoring || !set_underpegging) {
            return Err(
                "Manual scoring and underpegging must be enabled for muggins to be enabled",
            );
        }

        if set_overpegging && !set_manual_scoring {
            return Err("Manual scoring must be enabled for overpegging to be enabled");
        }

        // Sets game parameters
        self.is_manual_scoring = set_manual_scoring;
        self.is_underpegging = set_underpegging;
        self.is_overpegging = set_overpegging;
        self.is_muggins = set_muggins;

        // Creates vector of player objects to be used in game
        self.players = Vec::new();
        for name in player_names {
            self.players.push(Player {
                username: name,
                back_peg_pos: 0,
                front_peg_pos: 0,
                hand: Vec::with_capacity(6),
            })
        }

        self.deck = deck::new_deck();
        self.crib = Vec::with_capacity(4);

        // Creates empty vector of PlayGroups and pushes an empty PlayGroup
        self.play_groups = Vec::with_capacity(5);
        self.play_groups.push(PlayGroup {
            cards: Vec::new(),
            total: 0,
        });

        self.state = GameState::CutInitial;
        Ok("Received valid GameSetup event")
    }

    // Processes the cut phase determining the first dealer after confirmation by player left of
    // dealer; leads to Deal
    fn process_cut(&mut self) -> Result<&str, &str> {
        // Start with shuffled deck; disabled in debug mode for manual editing of deck
        if !self.is_debug {
            self.deck.reset_deck();
        }

        // Deal one card to each player
        for player in &mut self.players {
            player.hand.push(self.deck.deal());
        }

        // Compare values of each player's card
        let mut lowest_value = 14;
        let mut player_indices_of_lowest_value = Vec::with_capacity(4);
        for (index, player) in self.players.iter().enumerate() {
            // Find value of given card
            let card_value = deck::return_value(player.hand[0]);
            // If card is the new lowest card, change lowest_value and make that player's index the
            // only member of player_indices_of_lowest_value
            if card_value < lowest_value {
                lowest_value = card_value;
                player_indices_of_lowest_value = Vec::new();
                player_indices_of_lowest_value.push(index);
            }
            // Else if the card is tied for lowest, add that player's index to
            // player_indices_of_lowest_value
            else if card_value == lowest_value {
                player_indices_of_lowest_value.push(index);
            }
        }

        // If the lowest value occurs twice, do not change the state
        if player_indices_of_lowest_value.len() > 1 {
            return Ok("Cut resulted in tie; redoing");
        }
        // Else change the dealer to the correct player index and change the state
        // Unwrap should never fail as highest index is three
        self.index_dealer = TryFrom::try_from(player_indices_of_lowest_value[0]).unwrap();
        self.state = GameState::Deal;
        Ok("First dealer chosen with cut")
    }

    // Deals the cards to each player's hand after confirmation call from the dealer; leads to Sort
    fn process_deal(&mut self) -> Result<&str, &str> {
        // Starts with shuffled deck
        if !self.is_debug {
            self.deck.reset_deck();
        }

        // Removes cards from the cut or the previous hand
        for player in &mut self.players {
            player.hand.clear();
        }

        // With two players, each player is dealt six cards; with three or four players, each
        // player is dealt five cards; with five players, five cards are dealt to everyone, but the
        // dealer who gets four
        if self.players.len() == 2 {
            for _i in 0..6 {
                for player in &mut self.players {
                    player.hand.push(self.deck.deal());
                }
            }
        } else if self.players.len() <= 4 {
            for _i in 0..5 {
                for player in &mut self.players {
                    player.hand.push(self.deck.deal());
                }
            }
        } else {
            for i in 0..5 {
                for (index, player) in &mut self.players.iter_mut().enumerate() {
                    // Excludes the dealer from being dealt a fifth card
                    if i != 4 || self.index_dealer != index as u8 {
                        player.hand.push(self.deck.deal());
                    }
                }
            }
        }

        self.state = GameState::Sort;
        Ok("Dealt cards to each player")
    }

    // Sorts each player's hand; separate from process_deal such as to allow the player to see the
    // order in which the cards were dealt before seeing the organized hand; leads to Discard
    fn sort_hand(&mut self) -> Result<&str, &str> {
        for player in &mut self.players {
            player.hand.sort();
        }

        self.state = GameState::Discard;
        Ok("Organized hands of each player")
    }

    // Places the cards in the selected discards into the crib; leads to CutStarter
    // Order of vector of discards match the order of the players vector
    fn process_discard(&mut self, player_discards: Vec<Vec<deck::Card>>) -> Result<&str, &str> {
        // Resets the crib vector to prevent extra cards
        self.crib.clear();

        // For every player's hand
        if player_discards.len() != self.players.len() {
            return Err("There must be a discard vector for every player");
        }
        for (player_index, discards) in player_discards.iter().enumerate() {
            // Check that number of discarded cards is correct
            if self.players.len() == 2 {
                if discards.len() != 2 {
                    return Err("Improper number of discarded cards; with two players, two cards are discarded");
                }
            } else if self.players.len() <= 4 {
                if discards.len() != 1 {
                    return Err("Improper number of discarded cards; with three of four players, one card is discarded");
                }
            } else {
                if (player_index == self.index_dealer as usize && discards.len() != 0)
                    || (player_index != self.index_dealer as usize && discards.len() != 1)
                {
                    return Err("Improper number of discarded cards; with five players, one card is discarded by everyone but the dealer");
                }
            }

            // Removes the card elements matching the discards from the player's hand
            self.players[player_index].hand.retain({
                |&card_hand| {
                    let mut is_card_in_discards = false;
                    for card_discard in discards {
                        if *card_discard == card_hand {
                            is_card_in_discards = true;
                        }
                    }
                    !is_card_in_discards
                }
            });

            // Add the discards to the crib
            for card in discards {
                self.crib.push(*card);
            }
        }

        // Adds a card from the deck when the number of players is three
        if self.players.len() == 3 {
            self.crib.push(self.deck.deal());
        }

        self.state = GameState::CutStarter;
        Ok("Processed players' discards")
    }

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
                return Ok("Starter card cut; nib");
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
                        //TODO Penalty for false call
                        Err("Invalid nibs call")
                    }
                } else {
                    Err("Invalid ScoreEvent at NibsCheck")
                }
            }
        }
    }

    // Returns whether or not a given card has already been played this round
    fn has_card_been_played(&mut self, card_in_question: deck::Card) -> bool {
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
                            return Err("Invalid ScoreEvent when overpegging is disable");
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
            } else if self.is_underpegging {
                self.state = GameState::CribMuggins;
            } else {
                self.index_dealer = (self.index_dealer + 1) % self.players.len() as u8;
                self.state = GameState::Deal;
            }
        }

        Ok("Scoring complete")
    }

    // Processes any muggins calls for any muggins state
    fn play_muggins(&mut self, selection: Option<Vec<score::ScoreEvent>>) -> Result<&str, &str> {
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
                // Create a list of which muggins selections are valid and which are invalid
                let mut valid_scores: Vec<score::ScoreEvent> = Vec::new();
                let mut invalid_scores: Vec<score::ScoreEvent> = Vec::new();
                for selection_event in &muggins_selections {
                    let mut is_selection_event_in_remaining = false;
                    for remaining_event in &self.remaining_score_events {
                        if *remaining_event == *selection_event {
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
                    } else {
                        // TODO Combine selections by the same player into one score change
                        for score in valid_scores {
                            self.players[score.player_index].change_score(score.point_value as i8);
                        }
                    }
                }

                // Prepares the game for the next card to be played or the ResetGame state to
                // prepare the next play group
                self.index_active = (self.index_active + 1) % self.players.len() as u8;
                if !self.reset_play {
                    self.state = GameState::PlayWaitForCard;
                } else {
                    self.state = GameState::ResetPlay;
                }
                return Ok("Play muggins complete");
            }
        }
    }

    // Processes the GameEvent objects to progress the model of the game
    pub fn process_event(&mut self, event: GameEvent) -> Result<&str, &str> {
        match (self.state, event) {
            // Accepts a GameSetip  event to continue to CutShuffleAndDeal
            (
                GameState::GameStart,
                GameEvent::GameSetup {
                    input_player_names,
                    input_manual,
                    input_underscoring,
                    input_muggins,
                    input_overscoring,
                },
            ) => Game::game_setup(
                self,
                input_player_names,
                input_manual,
                input_underscoring,
                input_muggins,
                input_overscoring,
            ),
            (GameState::GameStart, _) => Err("Expected GameSetup event to GameStart"),

            // Accepts a confirmation event indicating the players' false choices for cut
            (GameState::CutInitial, GameEvent::Confirmation) => Game::process_cut(self),
            (GameState::CutInitial, _) => Err("Expected Confirmation event to CutInitial"),

            // Deals the cards to each player taking confirmation from dealer
            (GameState::Deal, GameEvent::Confirmation) => Game::process_deal(self),
            (GameState::Deal, _) => Err("Expected Confirmation event to Deal"),

            // Sorts each player's hand after allowing the player to see the order in which the
            // cards were dealt
            (GameState::Sort, GameEvent::Confirmation) => Game::sort_hand(self),
            (GameState::Sort, _) => Err("Expected Confirmation event to Sort"),

            // Removes the chosen card(s) from each hand and places them in the crib
            (GameState::Discard, GameEvent::DiscardSelection(player_discards)) => {
                Game::process_discard(self, player_discards)
            }
            (GameState::Discard, _) => Err("Expected HandIndices event to Discard"),

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
            (GameState::PlayMuggins, GameEvent::Confirmation) => Game::play_muggins(self, None),
            (GameState::PlayMuggins, GameEvent::Muggins(selection)) => {
                Game::play_muggins(self, selection)
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
            (GameState::ShowMuggins, GameEvent::Confirmation) => {
                Err("TODO Confirmation ShowMuggins")
            }
            (GameState::ShowMuggins, GameEvent::Muggins(selection)) => {
                Err("TODO Muggins ShowMuggins")
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
            (GameState::CribMuggins, GameEvent::Confirmation) => {
                Err("TODO Confirmation CribMuggins")
            }
            (GameState::CribMuggins, GameEvent::Muggins(selection)) => {
                Err("TODO Muggins CribMuggins")
            }
            (GameState::CribMuggins, _) => {
                Err("Expected Confirmation or Muggins event to CribMuggins")
            }

            // Processes the end of a game
            (GameState::Win, GameEvent::Confirmation) => Err("TODO Confirmation Win"),
            (GameState::Win, GameEvent::Declination) => Err("TODO Declination Win"),
            // Processes the end of a match

            // For unexpected GameState
            (_, _) => Err("Unrecognized GameState"),
        }
    }
}
