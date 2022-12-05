use crate::{round::Round, roundoutcome::RoundOutcome, shape::Shape};

pub struct StrategyGuide {
    rounds: Vec<Round>,
    pub total_score: usize,
}

impl StrategyGuide {
    pub fn parse_old(input: &String) -> Self {
        let mut strategy_guide = StrategyGuide {
            rounds: Vec::new(),
            total_score: 0,
        };
        for line in input.lines() {
            let mut chars = line.chars();
            let opponent_shape = Shape::new(&chars.nth(0).expect("Could not get shape."));
            chars.next();
            let shape_selected = Shape::new(&chars.nth(0).expect("Could not get shape."));

            let round_outcome = RoundOutcome::calculate_outcome(&shape_selected, &opponent_shape);

            let round = Round {
                shape_selected,
                round_outcome,
            };
            let score = round.calculate_score();

            strategy_guide.total_score += score;
            strategy_guide.rounds.push(round);
        }

        strategy_guide
    }

    pub fn parse_new(input: &String) -> Self {
        let mut strategy_guide = StrategyGuide {
            rounds: Vec::new(),
            total_score: 0,
        };
        for line in input.lines() {
            let mut chars = line.chars();
            let opponent_shape = Shape::new(&chars.nth(0).expect("Could not get shape."));
            chars.next();
            let round_outcome =
                RoundOutcome::new(&chars.nth(0).expect("Cound not get round outcome."));

            let shape_selected = Shape::calculate_shape(&opponent_shape, &round_outcome);

            let round = Round {
                shape_selected,
                round_outcome,
            };
            let score = round.calculate_score();

            strategy_guide.total_score += score;
            strategy_guide.rounds.push(round);
        }

        strategy_guide
    }
}
