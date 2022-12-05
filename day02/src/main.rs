use std::env;

use helpers;

mod round;
mod roundoutcome;
mod shape;
mod strategyguide;

fn main() {
    let content = helpers::get_input(env::args().collect());

    let strategy_guide_old = strategyguide::StrategyGuide::parse_old(&content);
    let strategy_guide_new = strategyguide::StrategyGuide::parse_new(&content);

    println!("Part One: {0}", strategy_guide_old.total_score);
    println!("Part Two: {0}", strategy_guide_new.total_score);
}
