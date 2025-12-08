use std::{
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

fn main() {
    let text = fs::read_to_string("./inputs/day8.txt").unwrap();
    let positions = text
        .lines()
        .map(|line| {
            line.split(",")
                .map(|part| part.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|vec| (vec[0], vec[1], vec[2]))
        .collect::<Vec<_>>();

    let mut distances = (0..positions.len() - 1)
        .flat_map(|i| (i + 1..positions.len()).map(move |j| (i, j)))
        .map(|(i, j)| (i, j, euclidean_distance(positions[i], positions[j])))
        .collect::<Vec<_>>();
    distances
        .sort_by(|(_, _, distance1), (_, _, distance2)| distance1.partial_cmp(&distance2).unwrap());

    part1(&distances);
    part2(&distances, &positions);
}

fn part1(distances: &Vec<(usize, usize, f32)>) {
    let circuits = distances.iter().take(1000).fold(
        HashMap::new(),
        |mut circuits: HashMap<usize, BTreeSet<usize>>, &(index1, index2, _)| {
            let circuit1 = circuits
                .entry(index1)
                .or_insert(BTreeSet::from([index1]))
                .clone();
            let circuit2 = circuits.entry(index2).or_insert(BTreeSet::from([index2]));
            let merged = circuit1
                .union(&circuit2)
                .map(|index| index.to_owned())
                .collect::<BTreeSet<_>>();

            for junction_box in merged.iter() {
                circuits.insert(*junction_box, merged.clone());
            }

            circuits
        },
    );

    let unique_circuits: HashSet<BTreeSet<usize>> =
        circuits.values().map(|set| set.to_owned()).collect();
    let mut lengths = unique_circuits
        .iter()
        .map(|set| set.len())
        .collect::<Vec<_>>();
    lengths.sort();

    let product = lengths[lengths.len() - 3..lengths.len()]
        .into_iter()
        .map(|length| length.to_owned())
        .reduce(|product, length| product * length)
        .unwrap();

    println!("{}", product)
}

fn part2(distances: &Vec<(usize, usize, f32)>, positions: &Vec<(u32, u32, u32)>) {
    let mut circuits: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut connected_boxes: HashSet<usize> = HashSet::new();
    let mut product = 0;

    for &(index1, index2, _) in distances {
        connected_boxes.insert(index1);
        connected_boxes.insert(index2);

        let circuit1 = circuits
            .entry(index1)
            .or_insert(HashSet::from([index1]))
            .clone();
        let circuit2 = circuits.entry(index2).or_insert(HashSet::from([index2]));
        let merged = circuit1
            .union(&circuit2)
            .map(|index| index.to_owned())
            .collect::<HashSet<_>>();

        if merged.len() == 1000 {
            product = positions[index1].0 as u64 * positions[index2].0 as u64;
            break;
        }

        for junction_box in merged.iter() {
            circuits.insert(*junction_box, merged.clone());
        }
    }

    println!("{}", product)
}

fn euclidean_distance(position1: (u32, u32, u32), position2: (u32, u32, u32)) -> f32 {
    (((position1.0 as i64 - position2.0 as i64).pow(2)
        + (position1.1 as i64 - position2.1 as i64).pow(2)
        + (position1.2 as i64 - position2.2 as i64).pow(2)) as f32)
        .sqrt()
}
