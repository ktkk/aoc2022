use crate::shape::Shape;

pub enum RoundOutcome {
    Draw,
    Loss,
    Win,
}

impl RoundOutcome {
    pub fn new(character: &char) -> Self {
        match character {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }

    pub fn calculate_outcome(shape_selected: &Shape, opponent_shape: &Shape) -> Self {
        use Shape::{Paper, Rock, Scissors};
        match (shape_selected, opponent_shape) {
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Self::Win,
            (Rock, Rock) | (Scissors, Scissors) | (Paper, Paper) => Self::Draw,
            (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => Self::Loss,
        }
    }
}
