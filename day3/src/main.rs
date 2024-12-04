use std::fs;
use regex::Regex;

fn main() {
    println!("Hello, day3!");

    let input = get_input();
    let result_part1 = part1(input.as_str());

    println!("Result part 1: {}", result_part1);
}

fn part1(input: &str) -> u64 {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut result: u64 = 0;
    for (a, [mul1, mul2]) in regex.captures_iter(input).map(|c| c.extract()) {
        println!("{} - {}*{}", a, mul1, mul2);

        result = result + (mul1.parse::<u64>().unwrap() * mul2.parse::<u64>().unwrap());
    }

    result
}

fn get_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PART_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    
    #[test]
    fn test_input_part_1() {
        let result = part1(TEST_INPUT_PART_1);

        assert_eq!(161, result);
    }
}