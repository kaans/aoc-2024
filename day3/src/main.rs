use std::fs;
use regex::Regex;

fn main() {
    println!("Hello, day3!");

    let input = get_input();
    let result_part1 = part1(input.as_str());

    println!("Result part 1: {}", result_part1);

    let result_part2 = part2(input.as_str());

    println!("Result part 2: {}", result_part2);
}

fn part1(input: &str) -> u64 {
    let regex = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut result: u64 = 0;
    for (_, [mul1, mul2]) in regex.captures_iter(input).map(|c| c.extract()) {
        result = result + (mul1.parse::<u64>().unwrap() * mul2.parse::<u64>().unwrap());
    }

    result
}

fn part2(input: &str) -> u64 {
    let regex = Regex::new(r"(mul|do|don't)\((?:([0-9]{1,3}),([0-9]{1,3}))?\)").unwrap();

    let mut do_mul: bool = true;

    let result: u64 = regex.captures_iter(input).map(|c| {
        let op = c.get(1).unwrap().as_str();
        let mut result: u64 = 0;
        match op {
            "mul" => {
                if do_mul {
                    let mul1 = c.get(2).unwrap().as_str();
                    let mul2 = c.get(3).unwrap().as_str();
                    result = mul1.parse::<u64>().unwrap() * mul2.parse::<u64>().unwrap()
                }
            },
            "don't" => do_mul = false,
            "do" => do_mul = true,
            _ => eprintln!("Invalid op: {}", op)
        }

        result
    }).sum();

    result
}

fn get_input() -> String {
    fs::read_to_string("input.txt").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_PART_1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_PART_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_input_part_1() {
        let result = part1(TEST_INPUT_PART_1);

        assert_eq!(161, result);
    }

    #[test]
    fn test_input_part_2() {
        let result = part2(TEST_INPUT_PART_2);

        assert_eq!(48, result);
    }
}