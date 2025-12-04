use std::{collections::HashSet, fs};

fn main() {
    let text = fs::read_to_string("./inputs/day4.txt").unwrap();
    let roll_positions = text
        .lines()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.char_indices()
                .map(|(column_index, char)| (row_index, column_index, char))
                .collect::<Vec<_>>()
        })
        .filter(|&(_, _, char)| char == '@')
        .map(|(row, column, _)| (row as isize, column as isize))
        .collect::<HashSet<_>>();

    part1(&roll_positions);
    part2(&roll_positions);
}

fn part1(roll_positions: &HashSet<(isize, isize)>) {
    let accessible_rolls = get_accessible_rolls(roll_positions).count();

    println!("{}", accessible_rolls)
}

fn part2(roll_positions: &HashSet<(isize, isize)>) {
    let mut clone = roll_positions.clone();
    let mut sum = 0;
    loop {
        let accessible_rolls = get_accessible_rolls(&clone)
            .map(|position| *position)
            .collect::<Vec<_>>();
        if accessible_rolls.len() == 0 {
            break;
        }

        sum += accessible_rolls.len();
        for ele in accessible_rolls {
            clone.remove(&ele);
        }
    }

    println!("{}", sum)
}

fn get_accessible_rolls(
    roll_positions: &HashSet<(isize, isize)>,
) -> impl Iterator<Item = &(isize, isize)> {
    const DIRECTIONS: [(isize, isize); 8] = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ];

    roll_positions.iter().filter(|(row, column)| {
        DIRECTIONS
            .iter()
            .filter(|(row_offset, column_offset)| {
                roll_positions.contains(&(row + row_offset, column + column_offset))
            })
            .count()
            < 4
    })
}
