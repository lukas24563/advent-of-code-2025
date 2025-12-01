use std::fs;

fn main() {
    let text = fs::read_to_string("./inputs/day1.txt").unwrap();
    let changes = text
        .lines()
        .map(|line| {
            let (direction, number) = line.split_at(1);
            let direction_parsed = if direction == "L" { -1 } else { 1 };

            number
                .parse::<i32>()
                .map(|change| change * direction_parsed)
                .unwrap()
        })
        .collect::<Vec<_>>();

    part1(&changes);
    part2(&changes);
}

fn part1(changes: &Vec<i32>) {
    let password = changes
        .iter()
        .fold((0, 50), |(count, current), change: &i32| {
            let result = (current + change).rem_euclid(100);
            (count + (result == 0) as i32, result)
        })
        .0;

    println!("{}", password)
}

fn part2(changes: &Vec<i32>) {
    let password = changes
        .iter()
        .fold((0, 50), |(count, current), change: &i32| {
            let result = (current + change).rem_euclid(100);
            let zeroes = if *change >= 0 {
                (current + change) / 100
            } else if current + change <= 0 {
                ((current + change) / 100).abs() + (current != 0) as i32
            } else {
                0
            };

            (count + zeroes, result)
        })
        .0;

    println!("{}", password)
}
