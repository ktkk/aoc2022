use crate::roundoutcome::RoundOutcome;

pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn new(character: &char) -> Self {
        match character {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }

    pub fn calculate_shape(opponent_shape: &Self, round_outcome: &RoundOutcome) -> Self {
        use RoundOutcome::{Draw, Loss, Win};
        use Shape::{Paper, Rock, Scissors};

        match (round_outcome, opponent_shape) {
            (Draw, Rock) | (Win, Scissors) | (Loss, Paper) => Self::Rock,
            (Draw, Paper) | (Win, Rock) | (Loss, Scissors) => Self::Paper,
            (Draw, Scissors) | (Win, Paper) | (Loss, Rock) => Self::Scissors,
        }
    }
}
