use std::fs;

pub fn get_input(args: Vec<String>) -> String {
    if args.len() <= 1 {
        panic!("Please provide input.");
    }

    let input_file = &args[1];
    fs::read_to_string(input_file).expect("Could not read file.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let args = vec![String::from("test"), String::from("test_input.txt")];

        assert_eq!{
            "a\nb\nc\n",
            get_input(args)
        };
    }
}
