use super::PlayScoreType;
use super::ScoreEvent;
use super::ScoreType;
use std::cmp;

#[cfg(test)]
mod test {
    extern crate rand;
    use super::play_score;
    use crate::util::return_card;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn check_fifteen() {
        let play_group = crate::PlayGroup {
            total: 15,
            cards: vec![return_card('T', 'H'), return_card('5', 'H')],
        };

        let expected = vec![super::ScoreEvent {
            player_index: 0,
            point_value: 2,
            score_type: super::ScoreType::Play(super::PlayScoreType::Fifteen),
        }];

        assert_eq!(play_score(0, &play_group), expected);
    }

    #[test]
    fn check_thirty_one() {
        // Intentionally avoids a run or any tuples
        let play_group = crate::PlayGroup {
            total: 31,
            cards: vec![
                return_card('T', 'H'),
                return_card('J', 'H'),
                return_card('K', 'H'),
                return_card('A', 'H'),
            ],
        };

        let expected = vec![super::ScoreEvent {
            player_index: 1,
            point_value: 2,
            score_type: super::ScoreType::Play(super::PlayScoreType::ThirtyOne),
        }];

        assert_eq!(play_score(1, &play_group), expected);
    }

    // A straight scores one point per card over three cards, examples include 2,A,3 and 7,8,9,6;
    // the longest a straght can be is seven cards because of the maximum value of a PlayGroup: 1 +
    // 2 + 3 + 4 + 5 + 6 + 7 = 28
    #[test]
    fn check_straights() {
        let straight_lengths = vec![1, 2, 3, 4, 5, 6, 7];

        // Construct a straight containing one to seven cards in a row
        for length in straight_lengths {
            let mut cards = Vec::new();
            let mut total = 0;
            for value in 1..length + 1 {
                cards.push(return_card((value + 48 as u8) as char, 'H'));
                total += value;
            }

            // Shuffle the cards to ensure they can be played in any order
            let mut rng = thread_rng();
            cards.shuffle(&mut rng);

            let expected = if length == 1 || length == 2 {
                vec![]
            } else if length == 5 {
                vec![
                    super::ScoreEvent {
                        score_type: super::ScoreType::Play(super::PlayScoreType::Fifteen),
                        player_index: 0,
                        point_value: 2,
                    },
                    super::ScoreEvent {
                        score_type: super::ScoreType::Play(super::PlayScoreType::Straight(5)),
                        player_index: 0,
                        point_value: 5,
                    },
                ]
            } else {
                vec![super::ScoreEvent {
                    score_type: super::ScoreType::Play(super::PlayScoreType::Straight(length)),
                    player_index: 0,
                    point_value: length,
                }]
            };

            assert_eq!(play_score(0, &crate::PlayGroup { total, cards }), expected);
        }

        // Check that straights that aren't A,2,3,... like generated above also get scored
        // correctly
        let cards1 = vec![
            return_card('3', 'H'),
            return_card('4', 'C'),
            return_card('5', 'S'),
        ];
        let cards2 = vec![
            return_card('J', 'D'),
            return_card('Q', 'H'),
            return_card('K', 'S'),
        ];
        let expected = vec![super::ScoreEvent {
            score_type: super::ScoreType::Play(super::PlayScoreType::Straight(3)),
            player_index: 0,
            point_value: 3,
        }];

        assert_eq!(
            play_score(
                0,
                &crate::PlayGroup {
                    cards: cards1,
                    total: 12
                }
            ),
            expected
        );
        assert_eq!(
            play_score(
                0,
                &crate::PlayGroup {
                    cards: cards2,
                    total: 30,
                }
            ),
            expected
        );
    }

    // A double is worth two points, a triple six, and a quadruple twelve
    #[test]
    fn check_tuples() {
        let card_values = vec![
            'A', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K',
        ];

        for card_value in card_values {
            for num_present in 1..5 {
                let mut cards = Vec::new();
                for num in 1..num_present + 1 {
                    let suit = match num {
                        1 => 'H',
                        2 => 'C',
                        3 => 'D',
                        4 => 'S',
                        _ => panic!("Invalid num in check_tuples()"),
                    };

                    cards.push(return_card(card_value, suit));
                }

                match num_present {
                    1 => assert_eq!(play_score(0, &crate::PlayGroup { cards, total: 0 }), vec![]),
                    2 => assert_eq!(
                        play_score(0, &crate::PlayGroup { cards, total: 0 }),
                        vec![super::ScoreEvent {
                            score_type: super::ScoreType::Play(super::PlayScoreType::Pair),
                            point_value: 2,
                            player_index: 0,
                        }]
                    ),
                    3 => assert_eq!(
                        play_score(0, &crate::PlayGroup { cards, total: 0 }),
                        vec![super::ScoreEvent {
                            score_type: super::ScoreType::Play(super::PlayScoreType::Triple),
                            point_value: 6,
                            player_index: 0,
                        }]
                    ),

                    4 => assert_eq!(
                        play_score(0, &crate::PlayGroup { cards, total: 0 }),
                        vec![super::ScoreEvent {
                            score_type: super::ScoreType::Play(super::PlayScoreType::Quadruple),
                            point_value: 12,
                            player_index: 0,
                        }]
                    ),

                    _ => panic!("Invalid num_present in check_tuples()"),
                };
            }
        }
    }
}

// Returns a vector of ScoreEvents that is the perfect scoring of the latest addition to the
// PlayGroup
//
// Score types that this function will handle: Pair, Triple, Quadruple, Straight, Fifteen, ThirtyOne
// Nibs is dealt with in either CutStarter or NibsCheck and  LastCard isdealt with in the
// PlayWaitForCard state
//
// index is the index of the player who played the last card and play_group is the last PlayGroup
// of the GameImpl
//
// Note: this function assumes that the PlayGroup total is correct, if you're getting bad totals,
// it's because of the PlayWaitForCard state
pub fn play_score(index: u8, play_group: &crate::PlayGroup) -> Vec<ScoreEvent> {
    let mut output = Vec::new();

    if play_group.total == 15 {
        output.push(ScoreEvent {
            score_type: ScoreType::Play(PlayScoreType::Fifteen),
            player_index: index,
            point_value: 2,
        });
    } else if play_group.total == 31 {
        output.push(ScoreEvent {
            score_type: ScoreType::Play(PlayScoreType::ThirtyOne),
            player_index: index,
            point_value: 2,
        });
    }

    // Start checking for straights when there are at least three cards
    // One point per card, between three and seven cards
    if play_group.cards.len() >= 3 {
        // So given n cards, first we check all n cards, then the n-1 last cards, then the n-2 last
        // cards, down until the last three cards -- the minimum length of the run -- and adds a
        // Straight ScoreEvent for the largest straight found. But because a run may only be seven
        // cards at longest, we first check min(n, 7), then min(n-1, 6), and so on.
        let length_or_7 = cmp::min(play_group.cards.len(), 7);
        for index_offset in (3..length_or_7 + 1).rev() {
            let start_position: usize = play_group.cards.len() - index_offset;
            let mut is_value_present = [false; 13];

            // For every card in the PlayGroup's cards at or after the start position calculated
            // above
            for card in &play_group.cards[start_position..] {
                is_value_present[(crate::deck::return_value(*card) as usize) - 1] = true;
            }

            // See if the is_value_present array has a continuous section of true values with a
            // length equal to the index_offset (whether it has a straight of the number of cards
            // it has checked
            let mut num_continuous_values: u8 = 0;
            let mut max_num_continuous_values: u8 = 0;
            for element in &is_value_present {
                if *element {
                    num_continuous_values += 1;
                    if num_continuous_values > max_num_continuous_values {
                        max_num_continuous_values = num_continuous_values;
                    }
                } else {
                    num_continuous_values = 0;
                }
            }

            // If the cards checked are a straight, push the ScoreEvent with the appropriate values
            // and break from the loop such as not to continue searching for straights (eg. if there
            // is a straight of four, don't then check the last three cards for a straight)
            if max_num_continuous_values as usize == index_offset {
                output.push(ScoreEvent {
                    score_type: ScoreType::Play(PlayScoreType::Straight(max_num_continuous_values)),
                    point_value: max_num_continuous_values,
                    player_index: index,
                });
                break;
            }
        }
    }

    // Check for tuples
    // Only begin checking when there are at least two cards
    if play_group.cards.len() >= 2 {
        // Check the last four cards for a quadruple, then the last three for a triple, then the
        // last two for a double; if there are less than four cards only check starting from that
        // number

        let num_cards_or_4 = cmp::min(4, play_group.cards.len());
        for index_offset in (2..num_cards_or_4 + 1).rev() {
            let value_last_card = play_group.cards.last().unwrap().value;
            let mut is_consistent = true;

            let starting_index = play_group.cards.len() - index_offset;

            for card in &play_group.cards[starting_index..] {
                if card.value != value_last_card {
                    is_consistent = false;
                }
            }

            if is_consistent {
                if index_offset == 4 {
                    output.push(ScoreEvent {
                        score_type: ScoreType::Play(PlayScoreType::Quadruple),
                        point_value: 12,
                        player_index: index,
                    });
                    break;
                } else if index_offset == 3 {
                    output.push(ScoreEvent {
                        score_type: ScoreType::Play(PlayScoreType::Triple),
                        point_value: 6,
                        player_index: index,
                    });
                    break;
                } else if index_offset == 2 {
                    output.push(ScoreEvent {
                        score_type: ScoreType::Play(PlayScoreType::Pair),
                        point_value: 2,
                        player_index: index,
                    });
                }
            }
        }
    }

    output
}
