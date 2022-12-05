use crate::{roundoutcome::RoundOutcome, shape::Shape};

pub struct Round {
    pub shape_selected: Shape,
    pub round_outcome: RoundOutcome,
}

impl Round {
    pub fn calculate_score(&self) -> usize {
        use Shape::{Paper, Rock, Scissors};
        let shape_score = match self.shape_selected {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        use RoundOutcome::{Draw, Loss, Win};
        let round_score = match self.round_outcome {
            Loss => 0,
            Draw => 3,
            Win => 6,
        };

        shape_score + round_score
    }
}
