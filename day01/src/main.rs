use std::{env, fs};

#[allow(dead_code)]
#[derive(Debug)]
struct Elf {
    index: usize,
    calories: Vec<usize>,
    total_calories: usize,
}

fn get_input(args: Vec<String>) -> String {
    if args.len() <= 1 {
        panic!("Please provide input.");
    }

    let input_file = &args[1];
    fs::read_to_string(input_file).expect("Could not read file.")
}

fn parse_elves(input: String) -> Vec<Elf> {
    let mut elves = Vec::new();

    let mut index: usize = 0;
    let mut calories_per_elf = Vec::new();

    for line in input.lines() {
        calories_per_elf.push(line);

        if line.is_empty() {
            let nr_of_food_items = calories_per_elf.len() - 1;
            let calories: Vec<usize> = calories_per_elf
                .clone()
                .into_iter()
                .take(nr_of_food_items)
                .map(|s| {
                    s.parse::<usize>()
                        .expect(format!("Failed to parse calories for elf {}.", index).as_str())
                })
                .collect();
            let total_calories = calories.iter().sum();
            elves.push(Elf {
                index,
                calories,
                total_calories,
            });

            index += 1;
            calories_per_elf.clear();
        }
    }

    elves
}

fn main() {
    let content = get_input(env::args().collect());

    let mut elves = parse_elves(content);
    elves.sort_by(|a, b| b.total_calories.cmp(&a.total_calories));

    let total_calories_of_top_elf = elves.first().unwrap().total_calories;
    let total_calories_of_top_three: usize = elves
        .into_iter()
        .take(3)
        .into_iter()
        .map(|elf| elf.total_calories)
        .sum();

    println!("Part One: {total_calories_of_top_elf}");
    println!("Part Two: {total_calories_of_top_three}");
}
