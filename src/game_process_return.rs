#[derive(Debug, PartialEq)]
pub enum ConfigError {
    UnderpeggingEnabledWhenManualScoringIsDisabled,
    MugginsEnabledWhenManualScoringIsDisabled,
    MugginsEnabledWhenUnderpeggingIsDisabled,
    VDOIsNotTwoPlayersWhenVariantIsTwoPlayers,
    VDOIsTwoPlayersWhenVariantIsMoreThanTwoPlayers,
    VDOIsNotCaptainDealsWhenVariantIsThreeCaptain,
    VDOIsCaptainDealsWhenVariantIsNotThreeCaptain,
    VDOIsNotLoserDrawsForDealerWhenVariantHasPairs,
    LowballEnabledWhenVariantIsNotTwoPlayersOrPairs,
    LowballIsEnabledWhenUnderpeggingIsEnabled,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    GameSetup,
    Confirmation,
    DiscardSelection,
    Nibs,
    Play,
    ManScoreSelection,
    Muggins,
}

// u8 attatched is the player index in which the error occurs
#[derive(Debug, PartialEq)]
pub enum DiscardError {
    // Errors with number of indices given
    TwoCardsAreDiscardedWithTwoPlayers(u8),
    OneCardIsDiscardedWithThreePlayers(u8),
    OneCardIsDiscardedWithFourPlayers(u8),
    OneCardIsDiscardedForNonDealersWithFivePlayers(u8),
    NoCardsAreDiscardedForDealerWithFivePlayers(u8),
    OneCardIsDiscardedForNonDealersAndNonDealerPartnersWithSixPlayers(u8),
    NoCardsAreDiscardedForDealerAndDealerPartnerWithSixPlayers(u8),

    // Error with the indices given
    TwoCardIndicesMayNotBeRepeated(u8),
    IndicesAreBetween0And5InclusiveWithTwoStandard(u8),
    IndicesAreBetween0And6InclusiveWithTwoSevenCard(u8),
    IndicesAreBetween0And4InclusiveWithTwoFiveCard(u8),
    IndicesAreBetween0And4InclusiveWithThreeOrMorePlayers(u8),
}

// These are errors that only programmers implementing a front end to the library should see. They
// are things that will only appear from their errors, not a regular part of player input being
// handled.
#[derive(Debug, PartialEq)]
pub enum ImplError {
    // If you're using the public interface this should never occur, this error is returned when
    // there if the GameImpl's settings variable is None.
    NoConfig,
    // When creating the DiscardSelection(Vec<Vec<u8>>) event to be sent to the game, the outer
    // vector should have one Vec<u8> per player even if that inner Vec<u8> is empty such as with
    // the dealer in the five or six card variations
    ThereShouldBeOneDiscardIndicesVectorPerPlayer,
    // There will always be a starter card if you are using the public interface
    NibsCheckWhenNoStarterCard,
}

#[derive(Debug, PartialEq)]
pub enum NibsError {
    NoNibsCallWhenUnderscoringIsDisabled,
    NibsCallWhenNoCutJack,
    InvalidScoreEventToNibsCheck,
}

#[derive(Debug, PartialEq)]
pub enum PlayTurnError {
    CardHasAlreadyBeenPlayed,
    IndexIsBetween0And2InclusiveWithTwoFiveCard,
    IndexIsBetween0And4InclusiveWithTwoSevenCard,
    // This is true for all other variants
    IndexIsBetween0And3InclusiveWithThisRuleVariant,
    MustPlayCardIfAble,
    PlayGroupTotalMayNotExceed31,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ImplementationError(ImplError),
    ExpectedEvent(Vec<Event>),
    GameStartInvalidConfig(ConfigError),
    InitialCutError,
    DiscardErrors(Vec<DiscardError>),
    NibsCallError(NibsError),
    PlayWaitForCardError(PlayTurnError),
    UnimplementedState,
    UnrecognizedState,
}

#[derive(Debug, PartialEq)]
pub enum InitialCutReturn {
    CutTie,
    DealerChosen,
}

#[derive(Debug, PartialEq)]
pub enum StarterCutReturn {
    ManualScoring,
    AutoNibs,
    AutoNoNibs,
}

#[derive(Debug, PartialEq)]
pub enum NibsCheckReturn {
    Nibs,
    NoNibs,
}

#[derive(Debug, PartialEq)]
pub enum PlayWaitForCardReturn {
    AutomaticScoring(Vec<crate::score::ScoreEvent>),
    ManualScoring,
    ToResetPlay,
}

#[derive(Debug, PartialEq)]
pub enum Success {
    GameStart,
    InitialCut(InitialCutReturn),
    Deal,
    Sort,
    Discard,
    StarterCut(StarterCutReturn),
    NibsCheck(NibsCheckReturn),
    PlayWaitForCard(PlayWaitForCardReturn),
}
