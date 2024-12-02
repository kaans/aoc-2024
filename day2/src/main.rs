use std::fs;

fn main()
{
    part1();
}

fn get_input() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("input.txt").unwrap();

    println!("Total input elements: {}", input.lines().count());

    input.lines().map(|line| line.split_whitespace().map(|entry| entry.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect()
}

fn part1() {
    let input = get_input();

    let a: Vec<_> = input.iter()
        .map(|report| report
            .windows(2)
            .map(|e| e[0] - e[1])
            .collect::<Vec<_>>()
        )
        .filter(|report| report.iter().filter(|e| e.abs() > 3 || **e == 0).count() == 0)
        .filter(|report| report.windows(2).filter(|e| e[0].signum() != e[1].signum()).count() == 0)
        .collect();

    println!("Result part 1: {}", a.len());
}