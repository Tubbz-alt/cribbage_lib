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
    test.process_event(super::GameEvent::Confirmation);
    for player in &test.players {
        assert_eq!(player.hand.len(), 6);
    }

    // Confirm that program deals five cards to each player when there are three players
    test = return_basic_game(3, false);
    cut_until_dealer_chosen(&mut test);
    test.process_event(super::GameEvent::Confirmation);
    for player in &test.players {
        assert_eq!(player.hand.len(), 5);
    }

    // Confirm that program deals five cards to each player when there are four players
    test = return_basic_game(4, false);
    cut_until_dealer_chosen(&mut test);
    test.process_event(super::GameEvent::Confirmation);
    for player in &test.players {
        assert_eq!(player.hand.len(), 5);
    }

    // Confirm that program deals five cards to every player but the dealer and four card to
    // the dealer when there are five players
    test = return_basic_game(5, false);
    cut_until_dealer_chosen(&mut test);
    test.process_event(super::GameEvent::Confirmation);
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