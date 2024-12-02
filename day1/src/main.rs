use std::fs;

fn main() {
    part1();
    part2();
}

fn get_input() -> (Vec<u32>, Vec<u32>) {
    let content = fs::read_to_string("input.txt").expect("Something went wrong reading the file") ;

    let (left, right): (Vec<_>, Vec<_>)= content.lines()
        .map(|line| line.split_whitespace())
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .unzip();

    (left, right)
}

fn part1() {
    let (mut left, mut right) = get_input();

    left.sort();
    right.sort();

    let result = left.iter().zip(right.iter()).map(|(left, right)| left.abs_diff(*right)).sum::<u32>();

    println!("Result of part 1: {}", result);
}

fn part2() {
    let (left, right) = get_input();

    let result: u32  = left.iter().map(|l| right.iter().filter(|r| l == *r).count() as u32 * *l).sum();

    println!("Result of part 2: {}", result);
}