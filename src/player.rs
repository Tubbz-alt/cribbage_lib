use crate::deck;

#[derive(Debug, Clone)]
pub struct Player {
    // The peg positions tracking score
    pub back_peg_pos: u8,
    pub front_peg_pos: u8,

    pub hand: Vec<deck::Card>,

    // The index of this player's partner if applicable
    pub partner_index: Option<u8>,
}

impl Player {
    pub fn change_score(&mut self, change: i8) {
        // Move the back peg up and then the front peg forward to simulate moving the back peg
        // forward past the front peg when the change is positive
        if change > 0 {
            self.back_peg_pos = self.front_peg_pos;
            self.front_peg_pos += change as u8;
        } else if change < 0 {
            // TODO with score penalties for overpegging; ensure check to make sure score does not
            // go below zero
        }
        // If no points are scored, the pegs should not be touched
    }
}
