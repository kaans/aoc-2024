use std::fs;

fn main() {
    let result_part1 = part1();
    part2(result_part1);
}

fn get_input() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Total input elements: {}", input.lines().count());

    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|entry| entry.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn validate(report: &Vec<i32>) -> bool {
    let diffs = report.windows(2).map(|e| e[0] - e[1]).collect::<Vec<_>>();
    let count_level = diffs.iter().filter(|e| e.abs() > 3 || **e == 0).count();
    let count_decl = diffs
        .windows(2)
        .filter(|e| e[0].signum() != e[1].signum())
        .count();

    count_decl + count_level == 0
}

fn part1() -> usize {
    let input = get_input();

    let zero = input.iter().filter(|report| {
        if validate(&report) {
            return true;
        }

        false
    }).collect::<Vec<_>>();

    println!("RESULT part 1: {}", zero.len());

    zero.len()
}

fn part2(result_part1: usize) {
    let input = get_input();

    let more = input.iter().filter(|report| {
        if validate(&report) {
            return false;
        }

        true
    }).collect::<Vec<_>>();

    let res = more.iter().filter(|report| {
        for i in 0..report.len() {
            let mut reportsub = report.to_vec();
            reportsub.remove(i);
            if validate(&reportsub) {
                return true;
            }
        }

        false
    }).collect::<Vec<_>>();

    println!("Additional safe reports: {}", res.len());

    println!("RESULT party 2: {}", res.len() + result_part1);

    return;
}
