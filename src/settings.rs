#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RuleVariant {
    // Two players to 121
    TwoStandard,

    // Two players to 61
    TwoFiveCard,

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
