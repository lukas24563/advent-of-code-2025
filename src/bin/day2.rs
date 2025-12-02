use std::fs;

fn main() {
    let text = fs::read_to_string("./inputs/day2.txt").unwrap();

    part1(&text);
    part2(&text);
}

fn part1(text: &str) {
    let sum = create_iterator(text)
        .filter(|id| id.len() % 2 == 0)
        .filter(|id| {
            let (half1, half2) = id.split_at(id.len() / 2);
            half1 == half2
        })
        .filter_map(|id| id.parse::<usize>().ok())
        .sum::<usize>();

    println!("{}", sum)
}

fn part2(text: &str) {
    let sum = create_iterator(text)
        .filter(|id| {
            (1..id.len() / 2 + 1)
                .filter(|sequence_length| id.len() % sequence_length == 0)
                .filter(|sequence_length| {
                    let sequence_repeated = id
                        .split_at(*sequence_length)
                        .0
                        .repeat(id.len() / sequence_length);

                    sequence_repeated == *id
                })
                .next()
                .is_some()
        })
        .filter_map(|id| id.parse::<usize>().ok())
        .sum::<usize>();

    println!("{}", sum)
}

fn create_iterator(text: &str) -> impl Iterator<Item = String> {
    text.split(",")
        .filter_map(|range| range.split_once("-"))
        .filter_map(|(start, end)| start.parse::<usize>().ok().zip(end.parse::<usize>().ok()))
        .flat_map(|(start, end)| start..end + 1)
        .map(|id| id.to_string())
}
