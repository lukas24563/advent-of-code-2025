use std::{collections::HashMap, fs};

fn main() {
    let text = fs::read_to_string("./inputs/day11.txt").unwrap();
    let outputs = text
        .lines()
        .map(|line| {
            let (device, rest) = line.split_once(": ").unwrap();
            let outputs = rest.split(" ").collect::<Vec<_>>();
            (device, outputs)
        })
        .collect::<HashMap<_, _>>();

    part1(&outputs);
}

fn part1(outputs: &HashMap<&str, Vec<&str>>) {
    let mut reachability_map: HashMap<String, Vec<String>> = HashMap::new();
    fn get_path(
        current: &str,
        outputs: &HashMap<&str, Vec<&str>>,
        reachability_map: &mut HashMap<String, Vec<String>>,
    ) -> Vec<String> {
        if reachability_map.contains_key(current) {
            return reachability_map.get(current).unwrap().clone();
        }

        let mut reachable = outputs
            .get(current)
            .iter()
            .flat_map(|outputs: &&Vec<&str>| outputs.iter())
            .flat_map(|output| get_path(output, outputs, reachability_map))
            .collect::<Vec<_>>();

        reachable.push(current.to_owned());
        reachability_map.insert(current.to_owned(), reachable.clone());

        reachable
    }

    let path = get_path("you", &outputs, &mut reachability_map);
    let paths_count = path.iter().filter(|&name| name == "out").count();

    println!("{}", paths_count)
}
