use std::fs;

fn main() {
    part1();
}

fn part1() {
    let content = fs::read_to_string("input.txt").expect("Something went wrong reading the file") ;

    let (mut left, mut right): (Vec<_>, Vec<_>)= content.lines()
        .map(|line| line.split("   "))
        .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
        .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        .unzip();

    left.sort();
    right.sort();

    let result = left.iter().zip(right.iter()).map(|(left, right)| left.abs_diff(*right)).sum::<u32>();

    println!("Result of part 1: {}", result);
}
