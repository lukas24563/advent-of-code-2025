use std::fs;

fn main() {
    let text = fs::read_to_string("./inputs/day3.txt").unwrap();
    let banks = text
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|char| char.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&banks);
    part2(&banks);
}

fn part1(banks: &Vec<Vec<u32>>) {
    let joltage = banks.iter().map(|bank| get_joltage(bank, 2)).sum::<u64>();

    println!("{}", joltage)
}

fn part2(banks: &Vec<Vec<u32>>) {
    let joltage = banks.iter().map(|bank| get_joltage(bank, 12)).sum::<u64>();

    println!("{}", joltage)
}

fn get_joltage(cells: &[u32], batteries: u8) -> u64 {
    if batteries == 0 {
        return 0;
    }

    let largest = cells
        .iter()
        .enumerate()
        .filter(|(index, _)| *index < (cells.len() - batteries as usize + 1))
        .max_by(|(i_a, a), (i_b, b)| match a.cmp(b) {
            std::cmp::Ordering::Equal => i_b.cmp(i_a),
            other => other,
        })
        .unwrap();

    *largest.1 as u64 * (10 as u64).pow(batteries as u32 - 1)
        + get_joltage(cells.split_at(largest.0 + 1).1, batteries - 1)
}
