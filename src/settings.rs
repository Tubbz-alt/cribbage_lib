#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RuleVariant {
    // Two players to 121
    TwoStandard,

    // Two players to 61
    TwoFiveCard,

    // Two players to 151
    TwoSevenCard,

    // Three players to 121
    ThreeStandard,

    // One player to 61 and one pair to 121
    ThreeCaptain,

    // Four players to 121
    FourIndividual,

    // Two pairs to 121
    FourPairs,

    // Five players to 121
    FiveStandard,

    // Three pairs to 121
    SixPairs,
}

// A setting for how the dealer for the next game in the match is chosen; with two players the first
// dealer is the player who lost the game, but with three or more players I figure you can either
// have all of the losers cut for first dealer as if it were the initial game, or you can have the
// last place loser draw; when the PlayUntilOption is ranked and the VictorDealerOption is
// LosersDrawForDealer, all victors but first place are considered losers who draw
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VictorDealerOption {
    TwoPlayers,
    LosersDrawForDealer,
    LastPlaceIsDealer,
}

#[derive(Copy, Clone, Debug)]
pub struct GameSettings {
    pub variant: RuleVariant,
    pub victor_dealer_option: VictorDealerOption,
    pub is_manual_scoring: bool,
    pub is_underpegging: bool,
    pub is_muggins: bool,
    pub is_overpegging: bool,
    pub is_lowball: bool,
}
