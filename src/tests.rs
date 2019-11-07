use std::char;

// Returns a card object based on a specified value and suit character for the purpose of
// testing
fn return_card(set_value: char, set_suit: char) -> super::deck::Card {
    let set_value: super::deck::CardValue = match set_value {
        'A' => super::deck::CardValue::Ace,
        '2' => super::deck::CardValue::Two,
        '3' => super::deck::CardValue::Three,
        '4' => super::deck::CardValue::Four,
        '5' => super::deck::CardValue::Five,
        '6' => super::deck::CardValue::Six,
        '7' => super::deck::CardValue::Seven,
        '8' => super::deck::CardValue::Eight,
        '9' => super::deck::CardValue::Nine,
        'T' => super::deck::CardValue::Ten,
        'J' => super::deck::CardValue::Jack,
        'Q' => super::deck::CardValue::Queen,
        'K' => super::deck::CardValue::King,
        _ => panic!("Unexpexted value in return_card()"),
    };

    let set_suit: super::deck::CardSuit = match set_suit {
        'H' => super::deck::CardSuit::Hearts,
        'D' => super::deck::CardSuit::Diamonds,
        'C' => super::deck::CardSuit::Clubs,
        'S' => super::deck::CardSuit::Spades,
        _ => panic!("Unexpected suit in return_card()"),
    };

    super::deck::Card {
        value: set_value,
        suit: set_suit,
    }
}

// Return a basic game with a specified length and debug status
fn return_basic_game(len: u8, debug: bool) -> super::Game {
    let mut names = Vec::new();
    if len >= 2 {
        names.push("Alice".to_string());
        names.push("Bob".to_string());
    }
    if len >= 3 {
        names.push("Carol".to_string());
    }
    if len >= 4 {
        names.push("Dan".to_string());
    }
    if len == 5 {
        names.push("Erin".to_string());
    }

    let mut game = super::Game::new();
    if debug {
        game.is_debug = true;
    }

    game.process_event(super::GameEvent::GameSetup {
        input_player_names: names,
        input_manual: false,
        input_underscoring: false,
        input_muggins: false,
        input_overscoring: false,
    });

    game
}

fn cut_until_dealer_chosen(game: &mut super::Game) {
    while game.state == super::GameState::CutInitial {
        game.process_event(super::GameEvent::Confirmation);
    }
}

fn cut_deal_and_sort(game: &mut super::Game) {
    cut_until_dealer_chosen(game);
    game.process_event(super::GameEvent::Confirmation);
    game.process_event(super::GameEvent::Confirmation);
}

// Sets up a game with the given hands, crib, and starter card such as to test playing and/or showing
fn game_setup(
    hands_and_crib: Vec<Vec<super::deck::Card>>,
    starter: super::deck::Card,
    state: super::GameState,
) -> super::Game {
    // Ensures the given state is a valid location to start the simulation from
    assert!(state == super::GameState::PlayWaitForCard || state == super::GameState::ShowScore);
    // There will be one hand for every player and a crib in hands_and_crib so the length minus one
    // is the number of players
    let mut game = return_basic_game(hands_and_crib.len() as u8 - 1, false);

    // Push the card to the hand or crib
    for (index, hand) in hands_and_crib.iter().enumerate() {
        for card in hand {
            if index != hands_and_crib.len() - 1 {
                game.players[index].hand.push(*card);
            } else {
                game.crib.push(*card);
            }
        }
    }

    game.starter_card = starter;

    game.state = state;

    game.index_active = 1;

    game
}

#[test]
fn game_setup_test() {
    let mut names = Vec::new();
    names.push("Alice".to_string());
    let mut test: super::Game = super::Game::new();
    // Test that game accepts does not accept less than two players
    assert!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: false,
            input_underscoring: false,
            input_muggins: false,
            input_overscoring: false,
        }) == Err("Expected GameSetup with 2 to 5 player names")
    );

    names.push("Bob".to_string());

    // Test that game does not accept invalid settings eg. muggins being on while manual
    // scoring is off
    assert!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: false,
            input_underscoring: true,
            input_muggins: false,
            input_overscoring: false,
        }) == Err("Manual scoring must be enabled for underpegging to be enabled")
    );

    assert!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: true,
            input_underscoring: false,
            input_muggins: true,
            input_overscoring: false,
        }) == Err("Manual scoring and underpegging must be enabled for muggins to be enabled")
    );

    assert!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: false,
            input_underscoring: false,
            input_muggins: true,
            input_overscoring: false,
        }) == Err("Manual scoring and underpegging must be enabled for muggins to be enabled")
    );

    assert!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: false,
            input_underscoring: false,
            input_muggins: false,
            input_overscoring: true,
        }) == Err("Manual scoring must be enabled for overpegging to be enabled")
    );

    // Tests that valid play options succeed
    assert!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: false,
            input_underscoring: false,
            input_muggins: false,
            input_overscoring: false,
        }) == Ok("Received valid GameSetup event")
    );

    test = super::Game::new();
    names.push("Carol".to_string());

    assert_eq!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: true,
            input_underscoring: false,
            input_muggins: false,
            input_overscoring: false,
        }),
        Ok("Received valid GameSetup event")
    );

    assert!(test.state == super::GameState::CutInitial);

    test = super::Game::new();
    names.push("Dan".to_string());

    assert!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: true,
            input_underscoring: false,
            input_muggins: false,
            input_overscoring: true,
        }) == Ok("Received valid GameSetup event")
    );

    assert!(test.state == super::GameState::CutInitial);

    test = super::Game::new();
    names.push("Erin".to_string());

    assert!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: true,
            input_underscoring: true,
            input_muggins: true,
            input_overscoring: false,
        }) == Ok("Received valid GameSetup event")
    );

    assert!(test.state == super::GameState::CutInitial);

    // Tests that no more than five players are allowed
    test = super::Game::new();
    names.push("Frank".to_string());

    assert!(
        test.process_event(super::GameEvent::GameSetup {
            input_player_names: names.clone(),
            input_manual: false,
            input_underscoring: false,
            input_muggins: false,
            input_overscoring: false,
        }) == Err("Expected GameSetup with 2 to 5 player names")
    );
}

#[test]
fn cut_initial_test() {
    // Set up test game
    let mut test = return_basic_game(2, true);

    // Set the last two cards of the deck to cards of equal value
    test.deck.reset_deck();
    test.deck.card_vector[51] = return_card('A', 'S');
    test.deck.card_vector[50] = return_card('A', 'C');
    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("Cut resulted in tie; redoing"),
    );

    // Set the last two cards of the deck to cards of different value
    for player in &mut test.players {
        player.hand.clear();
    }
    test.deck.card_vector[49] = return_card('2', 'S');
    test.deck.card_vector[48] = return_card('A', 'S');
    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("First dealer chosen with cut")
    );
    assert_eq!(test.index_dealer, 1);

    // Add third player, and set last three cards of the deck to equal value
    test = return_basic_game(3, true);
    test.deck.reset_deck();
    test.deck.card_vector[51] = return_card('A', 'S');
    test.deck.card_vector[50] = return_card('A', 'C');
    test.deck.card_vector[49] = return_card('A', 'D');

    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("Cut resulted in tie; redoing"),
    );

    // With third player, set the last cards of the deck to two cards of equal value and one
    // card of higher value
    for player in &mut test.players {
        player.hand.clear();
    }
    test.deck.card_vector[48] = return_card('A', 'S');
    test.deck.card_vector[47] = return_card('A', 'C');
    test.deck.card_vector[46] = return_card('2', 'S');

    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("Cut resulted in tie; redoing"),
    );

    // With third player, set the last cards of the deck to two cards of equal value and one
    // card of lower value
    for player in &mut test.players {
        player.hand.clear();
    }
    test.deck.card_vector[45] = return_card('2', 'S');
    test.deck.card_vector[44] = return_card('2', 'C');
    test.deck.card_vector[43] = return_card('A', 'S');

    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("First dealer chosen with cut"),
    );
    assert_eq!(test.index_dealer, 2);

    // With third player, set the last three cards of the deck to cards of different value
    for player in &mut test.players {
        player.hand.clear();
    }
    test.state = super::GameState::CutInitial;
    test.deck.card_vector[42] = return_card('A', 'S');
    test.deck.card_vector[41] = return_card('2', 'S');
    test.deck.card_vector[40] = return_card('3', 'S');

    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("First dealer chosen with cut"),
    );
    assert_eq!(test.index_dealer, 0);
}

#[test]
fn deal_test() {
    // Set up test game
    let mut test = super::Game::new();

    // Confirm that program deals six cards to each player when there are two players
    test = return_basic_game(2, false);
    cut_until_dealer_chosen(&mut test);
    test.process_event(super::GameEvent::Confirmation).unwrap();
    for player in &test.players {
        assert_eq!(player.hand.len(), 6);
    }

    // Confirm that program deals five cards to each player when there are three players
    test = return_basic_game(3, false);
    cut_until_dealer_chosen(&mut test);
    test.process_event(super::GameEvent::Confirmation).unwrap();
    for player in &test.players {
        assert_eq!(player.hand.len(), 5);
    }

    // Confirm that program deals five cards to each player when there are four players
    test = return_basic_game(4, false);
    cut_until_dealer_chosen(&mut test);
    test.process_event(super::GameEvent::Confirmation).unwrap();
    for player in &test.players {
        assert_eq!(player.hand.len(), 5);
    }

    // Confirm that program deals five cards to every player but the dealer and four card to
    // the dealer when there are five players
    test = return_basic_game(5, false);
    cut_until_dealer_chosen(&mut test);
    test.process_event(super::GameEvent::Confirmation).unwrap();
    for (index, player) in test.players.iter().enumerate() {
        if index == test.index_dealer as usize {
            assert_eq!(player.hand.len(), 4);
        } else {
            assert_eq!(player.hand.len(), 5);
        }
    }
}

#[test]
fn discards_test() {
    // Sets up test for two players
    let mut test = return_basic_game(2, false);
    cut_deal_and_sort(&mut test);
    let mut discards: Vec<Vec<super::deck::Card>> = Vec::new();

    // Tests that the number of players check works
    assert_eq!(
        test.process_event(super::GameEvent::DiscardSelection(discards.clone())),
        Err("There must be a discard vector for every player"),
    );
    // Pushes one discard to both players and tests that the discard number check works
    for (index, player) in test.players.iter().enumerate() {
        discards.push(Vec::new());
        discards[index].push(player.hand[0]);
    }
    assert_eq!(
        test.process_event(super::GameEvent::DiscardSelection(discards.clone())),
        Err("Improper number of discarded cards; with two players, two cards are discarded")
    );
    // Pushes another discard to both players and tests that a proper input works
    for (index, player) in test.players.iter().enumerate() {
        discards[index].push(player.hand[1]);
    }
    // Tests that the discard of a proper input results in an Ok
    assert_eq!(
        test.process_event(super::GameEvent::DiscardSelection(discards.clone())),
        Ok("Processed players' discards"),
    );
    // Tests that the crib is of proper length and is made up of the discards
    assert_eq!(test.crib.len(), 4);
    for crib_card in test.crib {
        let mut is_in_discards = false;
        for player_discards in &discards {
            for discard in player_discards {
                if crib_card == *discard {
                    is_in_discards = true;
                }
            }
        }
        assert_eq!(is_in_discards, true);
    }
    // Tests that the player hands are of proper length
    for player in test.players {
        assert_eq!(player.hand.len(), 4);
    }

    // Sets up tests for three players
    test = return_basic_game(3, false);
    cut_deal_and_sort(&mut test);
    discards[0].clear();
    discards[1].clear();
    discards.push(Vec::new());

    // Tests that the number discarded check works
    assert_eq!(
        test.process_event(super::GameEvent::DiscardSelection(discards.clone())),
        Err(
            "Improper number of discarded cards; with three of four players, one card is discarded"
        ),
    );
    // Pushes a card to each discards member to test a proper input
    for (index, player) in test.players.iter().enumerate() {
        discards[index].push(player.hand[0]);
    }
    // Tests that a correct number of discards results in Ok
    assert_eq!(
        test.process_event(super::GameEvent::DiscardSelection(discards.clone())),
        Ok("Processed players' discards"),
    );
    // Tests that the crib is the right length
    assert_eq!(test.crib.len(), 4);
    // Tests that the crib has three cards from the discard selections and one from the deck
    let mut num_matching_cards = 0;
    let mut num_nonmatching_cards = 0;
    for crib_card in test.crib {
        let mut was_found = false;
        for player_discards in &discards {
            for discard in player_discards {
                if crib_card == *discard {
                    was_found = true;
                }
            }
        }
        if was_found {
            num_matching_cards += 1;
        } else {
            num_nonmatching_cards += 1;
        }
    }
    assert_eq!(num_matching_cards, 3);
    assert_eq!(num_nonmatching_cards, 1);
    // Tests that each player's hand is the right length
    for player in test.players {
        assert_eq!(player.hand.len(), 4);
    }

    // Checks for four players are the same as for three players

    // Sets up the test for five players
    test = return_basic_game(5, false);
    cut_deal_and_sort(&mut test);
    discards[0].clear();
    discards[1].clear();
    discards[2].clear();
    discards.push(Vec::new());
    discards.push(Vec::new());

    for (index, player) in test.players.iter().enumerate() {
        discards[index].push(player.hand[0]);
    }
    // Tests check for number of cards
    assert_eq!(
        test.process_event(super::GameEvent::DiscardSelection(discards.clone())),
        Err("Improper number of discarded cards; with five players, one card is discarded by everyone but the dealer"),
    );
    // Tests a proper discards input
    discards[test.index_dealer as usize].clear();
    assert_eq!(
        test.process_event(super::GameEvent::DiscardSelection(discards)),
        Ok("Processed players' discards"),
    );
    // Tests that the crib and each had is the proper length
    assert_eq!(test.crib.len(), 4);
    for player in test.players {
        assert_eq!(player.hand.len(), 4);
    }
}

#[test]
fn starter_card_test() {
    // For when manual scoring or underpegging are disabled
    let mut test = return_basic_game(2, true);
    // Jack with score of 119 or higher
    test.state = super::GameState::CutStarter;
    test.players[0].change_score(119);
    test.starter_card = return_card('J', 'S');
    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("Starter card cut; nibs and win"),
    );
    assert_eq!(test.state, super::GameState::Win);

    // Jack with score less than 119
    test = return_basic_game(2, true);
    test.state = super::GameState::CutStarter;
    test.starter_card = return_card('J', 'S');
    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("Starter card cut; nib")
    );
    assert_eq!(test.players[0].front_peg_pos, 2);
    assert_eq!(test.state, super::GameState::PlayWaitForCard);

    // No jack
    test = return_basic_game(2, true);
    test.state = super::GameState::CutStarter;
    test.starter_card = return_card('A', 'S');
    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("Starter card cut; no nibs"),
    );
    assert_eq!(test.state, super::GameState::PlayWaitForCard);

    // With manual scoring and underpegging enabled
    test = return_basic_game(2, false);
    test.state = super::GameState::CutStarter;
    test.is_manual_scoring = true;
    test.is_underpegging = true;
    test.deck.reset_deck();
    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("Starter card cut"),
    );
    assert_eq!(test.state, super::GameState::NibsCheck);
}

#[test]
fn check_nibs_test() {
    // Nibs not called
    let mut test = return_basic_game(2, false);
    test.state = super::GameState::NibsCheck;
    test.starter_card = return_card('A', 'S');
    assert_eq!(
        test.process_event(super::GameEvent::Nibs(None)),
        Ok("Nibs not called"),
    );
    assert_eq!(test.state, super::GameState::PlayWaitForCard);

    // Nibs called with Jack
    // Logic for checking if the score is past 120 should work as in the previous test with
    // automatic scoring
    test.state = super::GameState::NibsCheck;
    test.starter_card = return_card('J', 'S');
    assert_eq!(
        test.process_event(super::GameEvent::Nibs(Some(super::score::ScoreEvent {
            score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Nibs),
            player_index: test.index_dealer as usize,
            point_value: 2,
        }))),
        Ok("Nibs call"),
    );
    assert_eq!(test.players[0].front_peg_pos, 2);

    // Nibs called without Jack
    // TODO With overscoring penalty
    test.state = super::GameState::NibsCheck;
    test.starter_card = return_card('A', 'S');
    assert_eq!(
        test.process_event(super::GameEvent::Nibs(Some(super::score::ScoreEvent {
            score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Nibs),
            player_index: test.index_dealer as usize,
            point_value: 2,
        }))),
        Err("Invalid nibs call"),
    );

    // Invalid ScoreEvent
    test.state = super::GameState::NibsCheck;
    assert_eq!(
        test.process_event(super::GameEvent::Nibs(Some(super::score::ScoreEvent {
            score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Pair),
            player_index: 0,
            point_value: 2,
        }))),
        Err("Invalid ScoreEvent at NibsCheck")
    );
}

#[test]
fn auto_play_score_test() {
    // Tests the actual play_score function used to determine the perfect scoring of a play

    // Checks for 15
    let mut test = super::PlayGroup {
        cards: vec![return_card('T', 'S'), return_card('5', 'S')],
        total: 15,
    };
    assert_eq!(
        super::score::play_score(0, &test),
        vec![super::score::ScoreEvent {
            player_index: 0,
            point_value: 2,
            score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Fifteen),
        }],
    );

    // Checks for 31
    test = super::PlayGroup {
        cards: vec![
            return_card('T', 'S'),
            return_card('T', 'C'),
            return_card('T', 'D'),
            return_card('A', 'S'),
        ],
        total: 31,
    };
    assert_eq!(
        super::score::play_score(0, &test),
        vec![super::score::ScoreEvent {
            player_index: 0,
            point_value: 2,
            score_type: super::score::ScoreType::Play(super::score::PlayScoreType::ThirtyOne),
        }],
    );

    // Checks for pairs
    test = super::PlayGroup {
        cards: vec![return_card('A', 'S'), return_card('A', 'C')],
        total: 2,
    };
    assert_eq!(
        super::score::play_score(0, &test),
        vec![super::score::ScoreEvent {
            player_index: 0,
            point_value: 2,
            score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Pair),
        }],
    );

    // Checks for triples
    test = super::PlayGroup {
        cards: vec![
            return_card('A', 'S'),
            return_card('A', 'C'),
            return_card('A', 'D'),
        ],
        total: 3,
    };
    assert_eq!(
        super::score::play_score(0, &test),
        vec![super::score::ScoreEvent {
            player_index: 0,
            point_value: 6,
            score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Triple),
        }],
    );

    // Checks for quadruples
    test = super::PlayGroup {
        cards: vec![
            return_card('A', 'S'),
            return_card('A', 'C'),
            return_card('A', 'D'),
            return_card('A', 'H'),
        ],
        total: 4,
    };
    assert_eq!(
        super::score::play_score(0, &test),
        vec![super::score::ScoreEvent {
            player_index: 0,
            point_value: 12,
            score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Quadruple),
        }],
    );

    // Checks for runs of lengths three through seven (the maximum)
    for length in 3..8 {
        if length == 3 {
            test = super::PlayGroup {
                cards: vec![
                    return_card('A', 'S'),
                    return_card('2', 'S'),
                    return_card('3', 'S'),
                ],
                total: 6,
            };
        } else {
            test.cards
                .push(return_card(char::from_digit(length, 10).unwrap(), 'S'));
            test.total += length as u8;
        }

        if length != 5 {
            assert_eq!(
                super::score::play_score(0, &test),
                vec![super::score::ScoreEvent {
                    player_index: 0,
                    point_value: length as u8,
                    score_type: super::score::ScoreType::Play(
                        super::score::PlayScoreType::Straight(length as u8)
                    ),
                }],
            );
        } else {
            assert_eq!(
                super::score::play_score(0, &test),
                vec![
                    super::score::ScoreEvent {
                        player_index: 0,
                        point_value: 2,
                        score_type: super::score::ScoreType::Play(
                            super::score::PlayScoreType::Fifteen
                        ),
                    },
                    super::score::ScoreEvent {
                        player_index: 0,
                        point_value: 5,
                        score_type: super::score::ScoreType::Play(
                            super::score::PlayScoreType::Straight(5)
                        ),
                    }
                ]
            );
        }
    }
}

#[test]
fn auto_hand_score_test() {
    // Tests the function to find the perfect scoring of a given hand
    // super::score::score_hand(index, hand, starter); returns a Vec<ScoreEvent>

    let mut hand: Vec<super::deck::Card> = vec![
        return_card('A', 'S'),
        return_card('2', 'S'),
        return_card('3', 'S'),
        return_card('4', 'S'),
    ];

    let mut hand_and_starter = hand.clone();
    hand_and_starter.push(return_card('5', 'S'));
    hand_and_starter.sort();

    let mut expected_score_events: Vec<super::score::ScoreEvent> = vec![
        super::score::ScoreEvent {
            player_index: 0,
            point_value: 5,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                hand_and_starter.clone(),
            )),
        },
        super::score::ScoreEvent {
            player_index: 0,
            point_value: 5,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::FiveFlush(
                hand_and_starter.clone(),
            )),
        },
        super::score::ScoreEvent {
            player_index: 0,
            point_value: 2,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                hand_and_starter.clone(),
            )),
        },
    ];

    expected_score_events.sort();

    assert_eq!(
        expected_score_events,
        super::score::score_hand(0, hand.clone(), return_card('5', 'S'))
    );

    // Example hands found on the rules of cribbage Wikipedia page
    hand = vec![
        return_card('5', 'S'),
        return_card('4', 'S'),
        return_card('2', 'S'),
        return_card('6', 'H'),
    ];

    let mut score_event_card_vectors = vec![
        vec![
            return_card('4', 'S'),
            return_card('5', 'S'),
            return_card('6', 'H'),
        ],
        vec![
            return_card('4', 'S'),
            return_card('5', 'H'),
            return_card('6', 'H'),
        ],
        vec![return_card('5', 'H'), return_card('5', 'S')],
    ];

    for event_cards in &mut score_event_card_vectors {
        event_cards.sort();
    }

    expected_score_events = vec![
        super::score::ScoreEvent {
            player_index: 0,
            point_value: 2,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                score_event_card_vectors[0].clone(),
            )),
        },
        super::score::ScoreEvent {
            player_index: 0,
            point_value: 2,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                score_event_card_vectors[1].clone(),
            )),
        },
        super::score::ScoreEvent {
            player_index: 0,
            point_value: 2,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Pair(
                score_event_card_vectors[2].clone(),
            )),
        },
        super::score::ScoreEvent {
            player_index: 0,
            point_value: 3,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                score_event_card_vectors[0].clone(),
            )),
        },
        super::score::ScoreEvent {
            player_index: 0,
            point_value: 3,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                score_event_card_vectors[1].clone(),
            )),
        },
    ];

    expected_score_events.sort();

    assert_eq!(
        expected_score_events,
        super::score::score_hand(0, hand.clone(), return_card('5', 'H'))
    );

    hand = vec![
        return_card('6', 'D'),
        return_card('J', 'H'),
        return_card('4', 'H'),
        return_card('7', 'C'),
    ];

    let mut score_event_card_vectors = vec![
        vec![return_card('J', 'H'), return_card('5', 'H')],
        vec![
            return_card('6', 'D'),
            return_card('5', 'H'),
            return_card('4', 'H'),
        ],
        vec![
            return_card('4', 'H'),
            return_card('5', 'H'),
            return_card('6', 'D'),
            return_card('7', 'C'),
        ],
        vec![return_card('5', 'H'), return_card('J', 'H')],
    ];

    for event_cards in &mut score_event_card_vectors {
        event_cards.sort();
    }

    expected_score_events = vec![
        super::score::ScoreEvent {
            player_index: 1,
            point_value: 2,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                score_event_card_vectors[0].clone(),
            )),
        },
        super::score::ScoreEvent {
            player_index: 1,
            point_value: 2,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                score_event_card_vectors[1].clone(),
            )),
        },
        super::score::ScoreEvent {
            player_index: 1,
            point_value: 4,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                score_event_card_vectors[2].clone(),
            )),
        },
        super::score::ScoreEvent {
            player_index: 1,
            point_value: 1,
            score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Nobs(
                score_event_card_vectors[3].clone(),
            )),
        },
    ];

    expected_score_events.sort();

    assert_eq!(
        expected_score_events,
        super::score::score_hand(1, hand.clone(), return_card('5', 'H'))
    );

    // Tests for triples, quadruples, and flushes of four to fill in the gaps of the above tests
    hand = vec![
        return_card('A', 'D'),
        return_card('A', 'H'),
        return_card('A', 'S'),
        return_card('A', 'C'),
    ];

    let mut score_event_card_vectors = vec![vec![
        return_card('A', 'H'),
        return_card('A', 'C'),
        return_card('A', 'S'),
        return_card('A', 'D'),
    ]];

    for event_cards in &mut score_event_card_vectors {
        event_cards.sort();
    }

    expected_score_events = vec![super::score::ScoreEvent {
        player_index: 0,
        point_value: 12,
        score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Quadruple(
            score_event_card_vectors[0].clone(),
        )),
    }];

    expected_score_events.sort();

    assert_eq!(
        expected_score_events,
        super::score::score_hand(0, hand.clone(), return_card('5', 'H'))
    );

    hand = vec![
        return_card('A', 'D'),
        return_card('A', 'H'),
        return_card('A', 'S'),
        return_card('2', 'C'),
    ];

    let mut score_event_card_vectors = vec![vec![
        return_card('A', 'H'),
        return_card('A', 'D'),
        return_card('A', 'S'),
    ]];

    for event_cards in &mut score_event_card_vectors {
        event_cards.sort();
    }

    expected_score_events = vec![super::score::ScoreEvent {
        player_index: 0,
        point_value: 6,
        score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Triple(
            score_event_card_vectors[0].clone(),
        )),
    }];

    expected_score_events.sort();

    assert_eq!(
        expected_score_events,
        super::score::score_hand(0, hand.clone(), return_card('5', 'H'))
    );

    hand = vec![
        return_card('2', 'D'),
        return_card('4', 'D'),
        return_card('6', 'D'),
        return_card('8', 'D'),
    ];

    let mut score_event_card_vectors = vec![vec![
        return_card('2', 'D'),
        return_card('4', 'D'),
        return_card('6', 'D'),
        return_card('8', 'D'),
    ]];

    for event_cards in &mut score_event_card_vectors {
        event_cards.sort();
    }

    expected_score_events = vec![super::score::ScoreEvent {
        player_index: 0,
        point_value: 4,
        score_type: super::score::ScoreType::Show(super::score::ShowScoreType::FourFlush(
            score_event_card_vectors[0].clone(),
        )),
    }];

    expected_score_events.sort();

    assert_eq!(
        expected_score_events,
        super::score::score_hand(0, hand.clone(), return_card('T', 'H'))
    );
}

#[test]
fn play_automatic_test() {
    // Tests the play and score loop of the game with automatic scoring

    // Test game setup
    let mut hands: Vec<Vec<super::deck::Card>> = Vec::new();
    hands.push(Vec::new());
    hands.push(Vec::new());
    hands.push(Vec::new());
    hands[1].push(return_card('T', 'S'));
    hands[0].push(return_card('T', 'C'));
    hands[1].push(return_card('T', 'D'));
    hands[0].push(return_card('A', 'S'));
    hands[1].push(return_card('2', 'S'));
    hands[0].push(return_card('2', 'C'));
    let mut test = game_setup(
        hands.clone(),
        return_card('A', 'C'),
        super::GameState::PlayWaitForCard,
    );

    // Player 0 is dealer when skipping cut so player 1 goes first

    // Valid play
    assert_eq!(
        test.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
            test.players[1].hand[0]
        ))),
        Ok("Player places card")
    );
    test.process_event(super::GameEvent::Confirmation);
    assert_eq!(test.players[1].front_peg_pos, 0);

    assert_eq!(
        test.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
            test.players[0].hand[0],
        ))),
        Ok("Player places card")
    );
    test.process_event(super::GameEvent::Confirmation);
    assert_eq!(test.players[0].front_peg_pos, 2);

    assert_eq!(
        test.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
            test.players[1].hand[1],
        ))),
        Ok("Player places card"),
    );
    test.process_event(super::GameEvent::Confirmation);
    assert_eq!(test.players[1].front_peg_pos, 6);

    // Invalid go
    assert_eq!(
        test.process_event(super::GameEvent::Play(super::PlayTurn::Go)),
        Err("Player must play card if possible; go invalid")
    );

    // Valid go
    test.players[0].hand[1] = return_card('2', 'C');
    assert_eq!(
        test.process_event(super::GameEvent::Play(super::PlayTurn::Go)),
        Ok("Player goes")
    );

    // Valid last card point
    assert_eq!(
        test.process_event(super::GameEvent::Play(super::PlayTurn::Go)),
        Ok("Player takes last point"),
    );
    assert_eq!(test.players[1].front_peg_pos, 7);

    // Create next PlayGroup
    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("Ready for next PlayGroup"),
    );

    // Repeated card
    assert_eq!(
        test.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
            test.players[0].hand[0]
        ))),
        Err("Last card selected has already been played"),
    );

    // Total over 31
    test.play_groups[1].total = 30;
    assert_eq!(
        test.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
            test.players[0].hand[2]
        ))),
        Err("Last card selected brings total over 31"),
    );

    // Card not in player's hand
    test.play_groups[1].total = 0;
    assert_eq!(
        test.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
            test.players[1].hand[2]
        ))),
        Err("Card played must be in the active player's hand"),
    );

    // No cards left test
    test.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
        test.players[0].hand[2],
    )));
    test.process_event(super::GameEvent::Confirmation);
    test.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
        test.players[1].hand[2],
    )));
    test.process_event(super::GameEvent::Confirmation);
    test.process_event(super::GameEvent::Play(super::PlayTurn::Go));
    test.process_event(super::GameEvent::Play(super::PlayTurn::Go));
    assert_eq!(
        test.process_event(super::GameEvent::Confirmation),
        Ok("No cards remaining, proceed to scoring"),
    );
}

#[test]
fn play_manual_test() {
    // Tests the scoring of cards when manual scoring is enabled
    // Previous test tests for all the possibilities in the actual play so this test can just focus
    // on scoring
    // Tests with/without underscoring and with/without muggins

    // Test game setup
    let hands: Vec<Vec<super::deck::Card>> = vec![
        vec![
            return_card('5', 'S'),
            return_card('A', 'S'),
            return_card('3', 'S'),
            return_card('4', 'D'),
        ],
        vec![
            return_card('4', 'S'),
            return_card('6', 'S'),
            return_card('2', 'S'),
            return_card('3', 'C'),
        ],
        Vec::new(),
    ];
    let mut game = game_setup(
        hands.clone(),
        return_card('2', 'C'),
        super::GameState::PlayWaitForCard,
    );
    game.is_manual_scoring = true;

    game.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
        return_card('4', 'S'),
    )))
    .unwrap();

    // Overscoring disabled error check
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Fifteen),
            }
        ]))),
        Err("Invalid ScoreEvent when overpegging is disabled")
    );

    // TODO Overscoring enabled

    // Test with underscoring disabled

    // Valid none with underscoring disabled
    assert_eq!(
        game.process_event(super::GameEvent::Confirmation),
        Ok("Scoring complete")
    );

    // Invalid none with underscoring disabled
    game.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
        return_card('4', 'D'),
    )))
    .unwrap();
    assert_eq!(
        game.process_event(super::GameEvent::Confirmation),
        Err("Must enter the correct ScoreEvents when underpegging is disabled")
    );

    // Valid score selection passed
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Pair)
            },
        ]))),
        Ok("Scoring complete")
    );

    // Incomplete score selection with underscoring disabled
    let mut game = game_setup(
        hands.clone(),
        return_card('2', 'C'),
        super::GameState::PlayWaitForCard,
    );
    game.is_manual_scoring = true;

    game.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
        return_card('4', 'S'),
    )))
    .unwrap();
    game.process_event(super::GameEvent::Confirmation).unwrap();
    game.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
        return_card('5', 'S'),
    )))
    .unwrap();
    game.process_event(super::GameEvent::Confirmation).unwrap();
    game.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
        return_card('6', 'S'),
    )))
    .unwrap();

    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                point_value: 2,
                player_index: 1,
                score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Fifteen),
            }
        ]))),
        Err("Incomplete score selection when underpegging is disabled")
    );

    // Complete score selection with underscoring disabled
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                point_value: 2,
                player_index: 1,
                score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Fifteen),
            },
            super::score::ScoreEvent {
                point_value: 3,
                player_index: 1,
                score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Straight(3)),
            }
        ]))),
        Ok("Scoring complete")
    );

    // Test with underscoring enabled and muggins disabled

    // Test with underscoring and muggins enabled
}

#[test]
fn show_automatic_test() {
    let hands: Vec<Vec<super::deck::Card>> = vec![
        vec![
            return_card('A', 'S'),
            return_card('2', 'S'),
            return_card('3', 'S'),
            return_card('4', 'S'),
        ],
        vec![
            return_card('4', 'C'),
            return_card('5', 'S'),
            return_card('5', 'C'),
            return_card('6', 'S'),
        ],
        vec![
            return_card('7', 'S'),
            return_card('7', 'C'),
            return_card('7', 'D'),
            return_card('7', 'H'),
        ],
    ];

    let mut game = game_setup(
        hands.clone(),
        return_card('8', 'S'),
        super::GameState::ShowScore,
    );

    // Pone scores
    assert_eq!(
        game.process_event(super::GameEvent::Confirmation),
        Ok("Scoring complete"),
    );
    assert_eq!(game.players[1].front_peg_pos, 12);

    // Dealer scores main hand
    assert_eq!(
        game.process_event(super::GameEvent::Confirmation),
        Ok("Scoring complete"),
    );
    assert_eq!(game.players[0].front_peg_pos, 13);

    // Dealer scores crib
    assert_eq!(
        game.process_event(super::GameEvent::Confirmation),
        Ok("Scoring complete"),
    );
    assert_eq!(game.players[0].front_peg_pos, 33);
}
