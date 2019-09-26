use crate::deck;
use std::cmp;

// Enum indicating the type of scoring events encountered during the play phase
// Scoring is based on the entire PlayGroup
// TODO Log Nibs and LastCard event as needed in the relevant portion of the main file
#[derive(Debug, Clone, PartialEq)]
pub enum PlayScoreType {
    Nibs,         // Jack as starter card; two points for dealer
    Pair,         // Two cards with the same value; 2pts
    Triple,       // Three cards with the same value; 6pts
    Quadruple,    // Four cards with the same value; 12pts
    Straight(u8), // Three or more cards with sequential values; 1pt per card; variable is length
    Fifteen,      // When the exact value of the PlayGroup's total is 15; 2pts
    ThirtyOne,    // When the exact value of the PlayGroup's toal is 31; 2pts
    LastCard,     // When a player places the last card of a PlayGroup and does not have 31; 1pt
}

// Enum indicating the type of scoring events encoutered in the show phase
// Enum's options contain the cards used to make up each score event
// Allow manual scoring to count triples and quadruples as multiple pairs and to score double runs,
// triple runs, and double double runs with one selection
#[derive(Debug, Clone, PartialEq)]
pub enum ShowScoreType {
    // Any combination of cards which add to 15; two pts
    Fifteen(Vec<deck::Card>),
    // Two cards with the same value; 2pts
    Pair(Vec<deck::Card>),
    // Three cards with the same value; 6pts
    Triple(Vec<deck::Card>),
    // Four cards with same value; 12pts
    Quadruple(Vec<deck::Card>),
    // Three or more cards with sequential values; 1pt per card
    Straight(Vec<deck::Card>),
    // Four cards not counting the starter card of the same suit; 4pts
    FourFlush(Vec<deck::Card>),
    // Five cards of the same suit; 5pts
    FiveFlush(Vec<deck::Card>),
    // Jack in hand which matches suit of starter card; one pt
    Nobs(Vec<deck::Card>),
}

// Enum for indicating whether a score event was made during the play phase or the show phase
#[derive(Debug, Clone, PartialEq)]
pub enum ScoreType {
    Play(PlayScoreType),
    Show(ShowScoreType),
}

// Instance describing a single increase in score
// Used in logs of the game, manual scoring selection/confirmation, and for automatic scoring
// Vectors of ScoreEvents are returned by the scoring functions in this file to represent the
// correct score of each hand or PlayGroup
#[derive(Debug, Clone)]
pub struct ScoreEvent {
    pub score_type: ScoreType,
    pub player_index: usize,
    pub point_value: u8,
}

// Returns the score value of the last addition to a given PlayGroup
pub fn play_score(index: usize, current_group: &crate::PlayGroup) -> Vec<ScoreEvent> {
    let mut score_of_play = Vec::new();

    // Nibs counted in CutStarter or NibsCheck
    // Last card counted in PlayWaitForCard

    // 2pts for bringing the PlayGroup to exactly 15
    if current_group.total == 15 {
        score_of_play.push(ScoreEvent {
            score_type: ScoreType::Play(PlayScoreType::Fifteen),
            point_value: 2,
            player_index: index,
        });
    }
    // 2pts for bringing the PlayGroup to exactly 31
    if current_group.total == 31 {
        score_of_play.push(ScoreEvent {
            score_type: ScoreType::Play(PlayScoreType::ThirtyOne),
            point_value: 2,
            player_index: index,
        });
    }

    // 1pt per card for a straight in the cards; between 3 and 7 points
    // Ensures there are at least three cards before starting to check for straights
    if current_group.cards.len() >= 3 {
        // Iterates though the indices of the length of the cards vector of the current
        // PlayGroup to 3, the minimum number of cards, and checks for a straight of each
        // length while breaking at the first and largest present
        for index_offset in (3..current_group.cards.len()).rev() {
            // Calculates the start position of the vector slice by subtracting the offset from
            // the length of the PlayGroup
            let start_position: usize = current_group.cards.len() - index_offset;

            // Array of bools indicating whether a given card value is present in the search
            let mut is_value_present = [false; 13];

            // For each card in the last index_offset elements of the PlayGroup's cards vector
            for card in &current_group.cards[start_position..] {
                // Set the bool located at the position represented by the cards value to true
                is_value_present[deck::return_play_value(*card) as usize - 1] = true;
            }

            // See if the is_value_present array has a continuous section of index_offset true
            // values
            let mut num_continuous_values: u8 = 0;
            for element in &is_value_present {
                // Add one to the number of continuous values for each true element and reset
                // for every false value
                if *element {
                    num_continuous_values += 1;
                } else {
                    num_continuous_values = 0;
                }
            }

            // If the number of continuous values is the number of values searched then a run
            // of index_offset cards is present in play; add that run value to the
            // score_of_play vector and break from the loop to not double-count runs
            if num_continuous_values as usize == index_offset {
                score_of_play.push(ScoreEvent {
                    score_type: ScoreType::Play(PlayScoreType::Straight(num_continuous_values)),
                    point_value: num_continuous_values,
                    player_index: index,
                });
                break;
            }
        }
    }

    // Couple, triple, quadruple check; 2, 6, and 12pts respectivelu
    // Only cbegins checking when the PlayGroup has two cards
    if current_group.cards.len() >= 2 {
        let num_cards_or_4 = cmp::min(4, current_group.cards.len());
        for index_offset in (2..num_cards_or_4).rev() {
            // Value of the last played card and whether or not the same value is found in the
            // other elements in question
            let value_last_card = current_group.cards.last().unwrap().value;
            let mut is_consistent = true;

            // The length minus 4, 3, or 2 such as to start checking the consistency from the
            // last 4, 3, or 2 cards
            let starting_index = current_group.cards.len() - index_offset;

            // For each card in the PlayGroup from the starting index (the length minus 4, 3,
            // or 2) and the end of the
            for card in &current_group.cards[starting_index..] {
                // If the value is not consistent, mark it as such
                if card.value != value_last_card {
                    is_consistent = false;
                }

                if is_consistent {
                    if index_offset == 4 {
                        score_of_play.push(ScoreEvent {
                            score_type: ScoreType::Play(PlayScoreType::Quadruple),
                            point_value: 12,
                            player_index: index,
                        });
                    } else if index_offset == 3 {
                        score_of_play.push(ScoreEvent {
                            score_type: ScoreType::Play(PlayScoreType::Triple),
                            point_value: 6,
                            player_index: index,
                        });
                    } else if index_offset == 2 {
                        score_of_play.push(ScoreEvent {
                            score_type: ScoreType::Play(PlayScoreType::Pair),
                            point_value: 2,
                            player_index: index,
                        });
                    }

                    //Break such as to not double-count the PlayGroup
                    break;
                }
            }
        }
    }

    score_of_play
}

// Scores a given hand of five cards
pub fn score_hand(index: u8, hand: Vec<deck::Card>, starter: deck::Card) -> Vec<ScoreEvent> {
    let mut found_scores: Vec<ScoreEvent> = Vec::new();

    // Variables tracking the runs of four and five such that runs are not
    let mut num_runs_four_or_higher = 0;
    let mut cards_of_runs_four_or_higher: Vec<Vec<deck::Card>> = Vec::new();

    // Create combinations of the five cards with a binary counter
    let mut combinations: Vec<Vec<deck::Card>> = Vec::new();
    let mut card_active = [false; 5];
    while card_active[4] && card_active[3] && card_active[2] && card_active[1] && card_active[0] {
        let mut current_combination: Vec<deck::Card> = Vec::new();
        if card_active[4] {
            current_combination.push(starter);
        }
        if card_active[3] {
            current_combination.push(hand[3]);
        }
        if card_active[2] {
            current_combination.push(hand[2]);
        }
        if card_active[1] {
            current_combination.push(hand[1]);
        }
        if card_active[0] {
            current_combination.push(hand[0]);
        }

        if current_combination.len() > 1 {
            combinations.push(current_combination);
        }

        if !card_active[4] && card_active[3] && card_active[2] && card_active[1] && card_active[0] {
            card_active[4] = true;
            card_active[3] = false;
            card_active[2] = false;
            card_active[1] = false;
            card_active[0] = false;
        }
        if !card_active[3] && card_active[2] && card_active[1] && card_active[0] {
            card_active[3] = true;
            card_active[2] = false;
            card_active[1] = false;
            card_active[0] = false;
        }
        if !card_active[2] && card_active[1] && card_active[0] {
            card_active[2] = true;
            card_active[1] = false;
            card_active[0] = false;
        }
        if !card_active[1] && card_active[0] {
            card_active[1] = true;
            card_active[0] = false;
        }
        if !card_active[0] {
            card_active[0] = true;
        }
    }

    for combination in &combinations {
        // Flag for whether the combination is a double, triple, or quadruple; if any card does
        // not match the first card of the combination, then this flag is set to false
        let mut is_tuple = true;
        // Array of booleans representing whether a given card value is in the combination; if
        // there is a continuous group the length of the combination (such as not to count a
        // straight several times, then a straight is present
        let mut is_present = [false; 13];

        // Value representing the sum of the given combination such as to check whether or not
        // a combination adds to 15
        let mut sum = 0;
        for card in combination {
            if card.value != combination[0].value {
                is_tuple = false;
            }
            sum += deck::return_play_value(*card);
            is_present[deck::return_value(*card) as usize - 1] = true;
        }

        // If all the cards in the combination had equal values
        if is_tuple {
            if combination.len() == 2 {
                found_scores.push(ScoreEvent {
                    player_index: index as usize,
                    point_value: 2,
                    score_type: ScoreType::Show(ShowScoreType::Pair(combination.to_vec())),
                });
            }
        }
        // If the number of consecutive values equal the number of cards in the
        // combination
        let mut num_consecutive_values = 0;
        for element in &is_present {
            if *element {
                num_consecutive_values += 1;
            } else {
                num_consecutive_values = 0;
            }
        }
        if num_consecutive_values == combination.len() {
            if combination.len() >= 4 {
                num_runs_four_or_higher += 1;
                cards_of_runs_four_or_higher.push(combination.to_vec());
            }
            found_scores.push(ScoreEvent {
                player_index: index as usize,
                point_value: combination.len() as u8,
                score_type: ScoreType::Show(ShowScoreType::Straight(combination.to_vec())),
            });
        }

        // If the sum is 15
        if sum == 15 {
            found_scores.push(ScoreEvent {
                player_index: index as usize,
                point_value: 2,
                score_type: ScoreType::Show(ShowScoreType::Fifteen(combination.to_vec())),
            });
        }
    }

    // Combines pairs into triples and quadruples; there's probably a much easier way to do this,
    // but I don't heckin' know

    // A list of the cards already encountered in the all the pairs in the vector of ScoreEvents
    let mut cards_encountered: Vec<deck::Card> = Vec::new();
    // The CardValue of the card which is a part of a triple or quadruple; because the hand is five
    // cards total there can only be one triple or quadruple in a hand at once
    let mut matching_card_value: Option<deck::CardValue> = None;

    // Identifies the CardValue of each card in the triple or quadruple
    for score in &found_scores {
        // If the ScoreType is a Show(Pair())
        match &score.score_type {
            ScoreType::Show(ShowScoreType::Pair(cards)) => {
                // For every combination for previous pairs and the cards of the current pair
                for card_current_pair in cards {
                    for card_previous in &cards_encountered {
                        // If the current pair's cards match any of the previous pairs' cards
                        // set the matching_card_value to the value of the current pair element
                        if *card_current_pair == *card_previous {
                            matching_card_value = Some(card_current_pair.value);
                        }
                    }

                    cards_encountered.push(*card_current_pair);
                }
            }
            _ => {}
        }
    }

    // For every card in the cards encountered with the specified CardValue, add one to the count
    // and add a copy to the filtered list of cards
    let mut num_cards_in_triple_or_quadruple = 0;
    let mut cards_copies_of_triple_or_quadruple: Vec<deck::Card> = Vec::new();
    match matching_card_value {
        Some(value) => {
            for card in cards_encountered {
                if card.value == value {
                    num_cards_in_triple_or_quadruple += 1;
                    cards_copies_of_triple_or_quadruple.push(card)
                }
            }
        }
        None => {}
    };

    // If the num_of_cards_in_triple_or_quadruple is 3 or 4, remove the pairs that make up the
    // triple or quadruple from found_scores and add the triple or quadruple score event
    if num_cards_in_triple_or_quadruple == 3 {
        found_scores.push(ScoreEvent {
            player_index: index as usize,
            point_value: 6,
            score_type: ScoreType::Show(ShowScoreType::Triple(cards_copies_of_triple_or_quadruple)),
        });

        // Delete all instances of a score of the pair type with a matching card value to the value
        // of the cards in the triple
        found_scores.retain({
            |score| match &score.score_type {
                ScoreType::Show(ShowScoreType::Pair(cards)) => {
                    let mut is_match = false;
                    for card in cards {
                        if card.value == matching_card_value.unwrap() {
                            is_match = true;
                        }
                    }

                    !is_match
                }
                _ => true,
            }
        });
    } else if num_cards_in_triple_or_quadruple == 4 {
        found_scores.push(ScoreEvent {
            player_index: index as usize,
            point_value: 12,
            score_type: ScoreType::Show(ShowScoreType::Quadruple(
                cards_copies_of_triple_or_quadruple,
            )),
        });

        // As a hand has five cards, a hand with a quadruple will not contain any pairs that don't
        // make up the quadruple
        found_scores.retain({
            |score| match &score.score_type {
                ScoreType::Show(ShowScoreType::Pair(_)) => false,
                _ => true,
            }
        });
    }

    // Checks for multiple counting eg. a run of four and a run of three with the same cards;
    // there may be one run of five and one or two runs of four

    if num_runs_four_or_higher > 0 {
        // For each run in the runs of four or higher, henceforth the big runs; there can be at most two big runs
        for big_run in cards_of_runs_four_or_higher {
            // For each score event that is a run
            found_scores.retain({
                |score| match &score.score_type {
                    // If the ScoreType is a run
                    ScoreType::Show(ShowScoreType::Straight(small_run)) => {
                        let mut large_contains_small = true;
                        // For every card in the run of equal or smaller length, henceforth the
                        // small run
                        for card_of_small in small_run {
                            let mut is_current_card_in_larger_run = false;
                            // For every card in the big run, check if the card of the small run is
                            // contained and set the flag accordingly
                            for card_of_big in &big_run {
                                if *card_of_big == *card_of_small {
                                    is_current_card_in_larger_run = true;
                                }
                            }
                            // If any card in the smaller run is not contained in the bigger, then
                            // the large run does not contain the small run
                            if !is_current_card_in_larger_run {
                                large_contains_small = false;
                            }
                        }

                        // Retain the score if the large run does not contain the small score
                        !large_contains_small
                    }
                    // If the ScoreType is not a run, retain the score
                    _ => true,
                }
            });
        }
    }

    // Checks for nobs
    for card in &hand {
        if card.value == deck::CardValue::Jack && card.suit == starter.suit {
            let mut nobs: Vec<deck::Card> = Vec::new();
            nobs.push(*card);
            nobs.push(starter);
            found_scores.push(ScoreEvent {
                player_index: index as usize,
                point_value: 1,
                score_type: ScoreType::Show(ShowScoreType::Nobs(nobs)),
            });
        }
    }

    // Checks for flushes
    // num_matching_cards will equal 4 if the hand contins a flush
    let suit: deck::CardSuit = hand[0].suit;
    let mut num_matching_cards: u8 = 0;
    for card in &hand {
        if card.suit == suit {
            num_matching_cards += 1;
        }
    }

    // If the hand contains a flush, check if the starter matches the suit too
    if num_matching_cards == 4 && starter.suit == suit {
        num_matching_cards += 1;
    }

    // If the hand contains a flush of 4 or 5, push the relevant ScoreEvents
    if num_matching_cards == 4 {
        found_scores.push(ScoreEvent {
            player_index: index as usize,
            point_value: 4,
            score_type: ScoreType::Show(ShowScoreType::FourFlush(hand)),
        });
    } else if num_matching_cards == 5 {
        let mut all_cards: Vec<deck::Card> = Vec::new();

        for card in hand {
            all_cards.push(card);
        }
        all_cards.push(starter);

        found_scores.push(ScoreEvent {
            player_index: index as usize,
            point_value: 5,
            score_type: ScoreType::Show(ShowScoreType::FiveFlush(all_cards)),
        });
    }

    found_scores
}
