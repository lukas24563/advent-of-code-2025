use std::fs;

struct Region {
    size: u32,
    shapes: Vec<u8>,
}

impl Region {
    fn parse(string: &str) -> Option<Region> {
        let (raw_size, raw_shapes) = string.split_once(": ")?;
        let size = raw_size
            .split("x")
            .map(|dimension| dimension.parse::<u32>().unwrap())
            .reduce(|a, b| a * b)?;

        let shapes = raw_shapes
            .split(" ")
            .map(|shape| shape.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        Some(Region { size, shapes })
    }
}

fn main() {
    let text = fs::read_to_string("./inputs/day12.txt").unwrap();
    let blocks = text.split("\n\n").collect::<Vec<_>>();
    let sizes = blocks
        .iter()
        .take(blocks.len() - 1)
        .map(|block| block.chars().filter(|&char| char == '#').count())
        .collect::<Vec<_>>();

    let regions = blocks
        .last()
        .iter()
        .flat_map(|block| block.lines())
        .map(Region::parse)
        .collect::<Option<Vec<_>>>()
        .unwrap();

    let valid_regions = regions
        .iter()
        .filter(|region| {
            let best_case_size = region
                .shapes
                .iter()
                .enumerate()
                .map(|(index, count)| *count as u32 * sizes[index] as u32)
                .sum::<u32>();
            region.size >= best_case_size
        })
        .count();

    println!("{}", valid_regions)
}
