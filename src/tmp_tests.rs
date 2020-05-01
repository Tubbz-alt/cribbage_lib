/*

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
    let mut game = game_setup(
        hands.clone(),
        return_card('2', 'C'),
        super::GameState::PlayWaitForCard,
    );
    game.is_manual_scoring = true;
    game.is_underpegging = true;

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
        Ok("Scoring complete")
    );
    assert_eq!(game.players[1].front_peg_pos, 2);

    // Test with underscoring and muggins enabled
    let mut game = game_setup(
        hands.clone(),
        return_card('2', 'C'),
        super::GameState::PlayWaitForCard,
    );
    game.is_manual_scoring = true;
    game.is_underpegging = true;
    game.is_muggins = true;

    game.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
        return_card('4', 'S'),
    )))
    .unwrap();
    game.process_event(super::GameEvent::Confirmation).unwrap();
    game.process_event(super::GameEvent::Confirmation).unwrap();
    game.process_event(super::GameEvent::Play(super::PlayTurn::CardSelected(
        return_card('5', 'S'),
    )))
    .unwrap();
    game.process_event(super::GameEvent::Confirmation).unwrap();
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
        Ok("Scoring complete")
    );

    assert_eq!(
        game.process_event(super::GameEvent::Muggins(Some(vec![
            super::score::ScoreEvent {
                point_value: 2,
                player_index: 0,
                score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Fifteen),
            },
            super::score::ScoreEvent {
                point_value: 3,
                player_index: 0,
                score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Straight(3)),
            }
        ]))),
        Err("Invalid muggins selection")
    );

    assert_eq!(
        game.process_event(super::GameEvent::Muggins(Some(vec![
            super::score::ScoreEvent {
                point_value: 3,
                player_index: 0,
                score_type: super::score::ScoreType::Play(super::score::PlayScoreType::Straight(3)),
            }
        ]))),
        Ok("Muggins complete")
    );
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

#[test]
fn show_manual_test() {
    // Tests the manual scoring of the hands and crib

    // Overpegging disabled invalid score check
    // Game setup
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

    let mut selections: Vec<Vec<Vec<super::deck::Card>>> = vec![
        // Player one's hand
        vec![
            // Fifteen two & run of three
            vec![
                return_card('4', 'C'),
                return_card('5', 'S'),
                return_card('6', 'S'),
            ],
            // Fifteen four & run of three
            vec![
                return_card('4', 'C'),
                return_card('5', 'C'),
                return_card('6', 'S'),
            ],
            // Pair
            vec![return_card('5', 'S'), return_card('5', 'C')],
        ],
        // Player zero's hand
        vec![
            // Fifteen two
            vec![
                return_card('3', 'S'),
                return_card('4', 'S'),
                return_card('8', 'S'),
            ],
            // Fifteen four
            vec![
                return_card('A', 'S'),
                return_card('2', 'S'),
                return_card('4', 'S'),
                return_card('8', 'S'),
            ],
            // Run of four
            vec![
                return_card('A', 'S'),
                return_card('2', 'S'),
                return_card('3', 'S'),
                return_card('4', 'S'),
            ],
            // Flush of five
            vec![
                return_card('A', 'S'),
                return_card('2', 'S'),
                return_card('3', 'S'),
                return_card('4', 'S'),
                return_card('8', 'S'),
            ],
        ],
        // Player zero's crib
        vec![
            // Fifteen two
            vec![return_card('7', 'S'), return_card('8', 'S')],
            // Fifteen four
            vec![return_card('7', 'C'), return_card('8', 'S')],
            // Fifteen six
            vec![return_card('7', 'D'), return_card('8', 'S')],
            // Fifteen eight
            vec![return_card('7', 'H'), return_card('8', 'S')],
            // Quadruple
            vec![
                return_card('7', 'S'),
                return_card('7', 'C'),
                return_card('7', 'D'),
                return_card('7', 'H'),
            ],
        ],
    ];

    // Sorts the selections such that ScoreEvents made with each selection are valid
    for hand in &mut selections {
        for selection in hand {
            selection.sort();
        }
    }

    let mut game = game_setup(
        hands.clone(),
        return_card('8', 'S'),
        super::GameState::ShowScore,
    );
    game.is_manual_scoring = true;
    // Hands
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                point_value: 4,
                player_index: 1,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::FourFlush(
                    hands[1].clone()
                )),
            }
        ]))),
        Err("Invalid ScoreEvent when overpegging is disabled")
    );
    // Crib
    game.state = super::GameState::CribScore;
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                point_value: 4,
                player_index: 1,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::FourFlush(
                    hands[1].clone()
                )),
            }
        ]))),
        Err("Invalid ScoreEvent when overpegging is disabled")
    );

    // Underpegging disabled invalid None
    // Game Setup
    game = game_setup(
        hands.clone(),
        return_card('8', 'S'),
        super::GameState::ShowScore,
    );
    game.is_manual_scoring = true;
    // Hands
    assert_eq!(
        game.process_event(super::GameEvent::Confirmation),
        Err("Must enter the correct ScoreEvents when underpegging is disabled")
    );
    // Crib
    game.state = super::GameState::CribScore;
    assert_eq!(
        game.process_event(super::GameEvent::Confirmation),
        Err("Must enter the correct ScoreEvents when underpegging is disabled")
    );

    // Underpegging disabled incomplete ScoreEvent vector
    // Game Setup
    game = game_setup(
        hands.clone(),
        return_card('8', 'S'),
        super::GameState::ShowScore,
    );
    game.is_manual_scoring = true;
    // Hands
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Pair(
                    selections[0][2].clone()
                )),
            }
        ]))),
        Err("Incomplete score selection when underpegging is disabled")
    );
    // Crib
    game.state = super::GameState::CribScore;
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][0].clone()
                )),
            }
        ]))),
        Err("Incomplete score selection when underpegging is disabled")
    );

    // Underpegging disabled valid ScoreEvent vector
    // Game Setup
    game = game_setup(
        hands.clone(),
        return_card('8', 'S'),
        super::GameState::ShowScore,
    );
    game.is_manual_scoring = true;
    // Hands
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[0][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 3,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                    selections[0][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[0][1].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 3,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                    selections[0][1].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Pair(
                    selections[0][2].clone()
                )),
            },
        ]))),
        Ok("Scoring complete")
    );

    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[1][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[1][1].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 4,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                    selections[1][2].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 5,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::FiveFlush(
                    selections[1][3].clone()
                )),
            },
        ]))),
        Ok("Scoring complete")
    );
    // Crib
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][1].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][2].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][3].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 12,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Quadruple(
                    selections[2][4].clone()
                )),
            },
        ]))),
        Ok("Scoring complete")
    );

    assert_eq!(game.players[1].front_peg_pos, 12);
    assert_eq!(game.players[0].front_peg_pos, 33);

    // Underpegging enabled incomplete ScoreEvent vector
    // Game Setup
    game = game_setup(
        hands.clone(),
        return_card('8', 'S'),
        super::GameState::ShowScore,
    );
    game.is_manual_scoring = true;
    game.is_underpegging = true;
    // Hands
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[0][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 3,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                    selections[0][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[0][1].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 3,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                    selections[0][1].clone()
                )),
            },
        ]))),
        Ok("Scoring complete")
    );

    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[1][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 4,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                    selections[1][2].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 5,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::FiveFlush(
                    selections[1][3].clone()
                )),
            },
        ]))),
        Ok("Scoring complete")
    );
    // Crib
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][1].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][2].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][3].clone()
                )),
            },
        ]))),
        Ok("Scoring complete")
    );

    assert_eq!(game.players[1].front_peg_pos, 10);
    assert_eq!(game.players[0].front_peg_pos, 19);

    // Muggins enabled incomplete ScoreEvent vector
    // Game Setup
    game = game_setup(
        hands.clone(),
        return_card('8', 'S'),
        super::GameState::ShowScore,
    );
    game.is_manual_scoring = true;
    game.is_underpegging = true;
    game.is_muggins = true;
    // Hands
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[0][1].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 3,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                    selections[0][1].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Pair(
                    selections[0][2].clone()
                )),
            },
        ]))),
        Ok("Scoring complete")
    );

    assert_eq!(
        game.process_event(super::GameEvent::Muggins(Some(vec![
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[0][0].clone()
                )),
            }
        ]))),
        Ok("Muggins complete")
    );

    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 5,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::FiveFlush(
                    selections[1][3].clone()
                )),
            },
        ]))),
        Ok("Scoring complete")
    );

    assert_eq!(
        game.process_event(super::GameEvent::Muggins(Some(vec![
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 5,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::FiveFlush(
                    selections[1][3].clone()
                )),
            },
        ]))),
        Err("Invalid muggins selection")
    );

    assert_eq!(
        game.process_event(super::GameEvent::Muggins(Some(vec![
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[1][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 1,
                point_value: 4,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Straight(
                    selections[1][2].clone()
                )),
            },
        ]))),
        Ok("Muggins complete")
    );
    // Crib
    assert_eq!(
        game.process_event(super::GameEvent::ManScoreSelection(Some(vec![
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][0].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][1].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][2].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 2,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Fifteen(
                    selections[2][3].clone()
                )),
            },
            super::score::ScoreEvent {
                player_index: 0,
                point_value: 12,
                score_type: super::score::ScoreType::Show(super::score::ShowScoreType::Quadruple(
                    selections[2][4].clone()
                )),
            },
        ]))),
        Ok("Scoring complete")
    );

    assert_eq!(
        game.process_event(super::GameEvent::Confirmation),
        Ok("No muggins selection")
    );

    assert_eq!(game.players[1].front_peg_pos, 13);
    assert_eq!(game.players[0].front_peg_pos, 27);
}
*/
