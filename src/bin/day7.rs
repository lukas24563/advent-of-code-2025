use std::{
    char,
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let text = fs::read_to_string("./inputs/day7.txt").unwrap();

    let start = position_iterator(&text, &'S').next().unwrap();
    let splitters = position_iterator(&text, &'^').collect::<HashSet<_>>();
    let size = text.lines().map(|line| line.len()).next().unwrap();

    part1(start, &splitters, size);
    part2(start, &splitters, size);
}

fn part1(start: (usize, usize), splitters: &HashSet<(usize, usize)>, size: usize) {
    let splits = (start.0..size)
        .fold(
            (0, HashSet::from([start.1])),
            |(splits, beams): (usize, HashSet<usize>), row| {
                let next_beams = beams
                    .iter()
                    .flat_map(|&column| match splitters.contains(&(row, column)) {
                        true => vec![column - 1, column + 1],
                        false => vec![column],
                    })
                    .collect::<Vec<_>>();

                (
                    splits + next_beams.len() - beams.len(),
                    HashSet::from_iter(next_beams.into_iter()),
                )
            },
        )
        .0;

    println!("{}", splits)
}

fn part2(start: (usize, usize), splitters: &HashSet<(usize, usize)>, size: usize) {
    let timelines: u64 = (start.0..size)
        .fold(HashMap::from([(start.1, 1_u64)]), |beams, row| {
            beams
                .into_iter()
                .flat_map(|(column, count)| match splitters.contains(&(row, column)) {
                    true => vec![(column - 1, count), (column + 1, count)],
                    false => vec![(column, count)],
                })
                .fold(HashMap::new(), |mut new_beams, (column, count)| {
                    *new_beams.entry(column).or_insert(0) += count;
                    new_beams
                })
        })
        .values()
        .sum();

    println!("{}", timelines);
}

fn position_iterator(text: &str, target: &char) -> impl Iterator<Item = (usize, usize)> {
    text.lines()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.char_indices()
                .map(move |(column_index, char)| (row_index, column_index, char))
        })
        .filter(|(_, _, char)| *char == *target)
        .map(|(row, column, _)| (row, column))
}
