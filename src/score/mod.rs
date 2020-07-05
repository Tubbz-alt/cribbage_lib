pub mod play;
pub mod show;

use crate::deck;
use std::cmp;

// Enum indicating the type of scoring events encountered during the play phase
// Scoring is based on the entire PlayGroup
// TODO Log Nibs and LastCard event as needed in the relevant portion of the main file
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
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
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
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
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum ScoreType {
    Play(PlayScoreType),
    Show(ShowScoreType),
}

// Instance describing a single increase in score
// Used in logs of the game, manual scoring selection/confirmation, and for automatic scoring
// Vectors of ScoreEvents are returned by the scoring functions in this file to represent the
// correct score of each hand or PlayGroup
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct ScoreEvent {
    pub score_type: ScoreType,
    pub player_index: u8,
    pub point_value: u8,
}
