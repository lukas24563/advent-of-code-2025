use std::{
    cmp::{max, min},
    fs, ptr,
};

#[derive(PartialEq, Clone, Copy)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn parse(text: &str) -> Option<Range> {
        let (start_raw, end_raw) = text.split_once("-")?;
        let start = start_raw.parse::<u64>().ok()?;
        let end = end_raw.parse::<u64>().ok()?;

        Some(Range { start, end })
    }

    fn check(self: &Range, id: u64) -> bool {
        self.start <= id && id <= self.end
    }
}

fn main() {
    let text = fs::read_to_string("./inputs/day5.txt").unwrap();
    let (raw_ranges, raw_ingredients) = text.split_once("\n\n").unwrap();

    let ranges = raw_ranges
        .lines()
        .filter_map(Range::parse)
        .collect::<Vec<_>>();
    let ingredients = raw_ingredients
        .lines()
        .filter_map(|id| id.parse::<u64>().ok())
        .collect::<Vec<_>>();

    part1(&ranges, &ingredients);
    part2(&ranges);
}

fn part1(ranges: &Vec<Range>, ingredients: &Vec<u64>) {
    let fresh_count = ingredients
        .iter()
        .filter(|&&id| ranges.iter().any(|range| range.check(id)))
        .count();

    println!("{}", fresh_count)
}

fn part2(ranges: &Vec<Range>) {
    let fresh_count = merge_ranges(ranges)
        .iter()
        .map(|range| range.end - range.start + 1)
        .sum::<u64>();

    println!("{}", fresh_count)
}

fn merge_ranges(ranges: &Vec<Range>) -> Vec<Range> {
    let mergeable = ranges
        .iter()
        .flat_map(|range1| {
            ranges
                .iter()
                .map(|range2| (range1, range2))
                .collect::<Vec<_>>()
        })
        .filter(|(range1, range2)| !ptr::eq(*range1, *range2))
        .find(|(range1, range2)| range1.check(range2.start) || range1.check(range2.end));

    return match mergeable {
        Some((range1, range2)) => {
            let mut result = ranges
                .iter()
                .filter(|&range| range != range1 && range != range2)
                .map(|&range| range)
                .collect::<Vec<_>>();

            result.push(Range {
                start: min(range1.start, range2.start),
                end: max(range1.end, range2.end),
            });
            merge_ranges(&result)
        }
        None => ranges.clone().to_vec(),
    };
}
