use std::fs;

fn main() {
    let text = fs::read_to_string("./inputs/day9.txt").unwrap();
    let coordinates = text
        .lines()
        .filter_map(|line| line.split_once(","))
        .filter_map(|(x, y)| x.parse::<u32>().ok().zip(y.parse::<u32>().ok()))
        .collect::<Vec<_>>();

    part1(&coordinates);
}

fn part1(coordinates: &Vec<(u32, u32)>) {
    let largest = (0..coordinates.len() - 1)
        .flat_map(|i| (i + 1..coordinates.len()).map(move |j| (i, j)))
        .map(|(i, j)| (coordinates[i], coordinates[j]))
        .map(|(coordinate1, coordinate2)| get_size(coordinate1, coordinate2))
        .max()
        .unwrap();

    println!("{}", largest)
}

fn get_size(coordinate1: (u32, u32), coordinate2: (u32, u32)) -> u64 {
    (coordinate1.0.abs_diff(coordinate2.0) + 1) as u64
        * (coordinate1.1.abs_diff(coordinate2.1) + 1) as u64
}
