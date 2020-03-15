#[derive(Debug, PartialEq)]
pub enum ConfigError {
    UnderpeggingEnabledWhenManualScoringIsDisabled,
    MugginsEnabledWhenManualScoringIsDisabled,
    MugginsEnabledWhenUnderpeggingIsDisabled,
    OverpeggingEnabledWhenManualScoringIsDisabled,
    VDOIsNotTwoPlayersWhenVariantIsTwoPlayers,
    VDOIsTwoPlayersWhenVariantIsMoreThanTwoPlayers,
    VDOIsNotLoserDrawsForDealerWhenVariantIsThreeCaptain,
    LowballEnabledWhenVariantIsThreeCaptain,
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
    IndicesAreBetween0And4InclusiveWithTwoFiveCard(u8),
    IndicesAreBetween0And4InclusiveWithThreeOrMoreCards(u8),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedEvent(Vec<Event>),
    GameStartInvalidConfig(ConfigError),
    InitialCutError,
    DiscardErrors(Vec<DiscardError>),
    UnimplementedState,
    UnrecognizedState,
}

#[derive(Debug, PartialEq)]
pub enum InitialCutReturn {
    CutTie,
    DealerChosen,
}

#[derive(Debug, PartialEq)]
pub enum Success {
    GameStart,
    InitialCut(InitialCutReturn),
    Deal,
    Sort,
    Discard,
    StarterCut,
    NibsCheck,
}
