use std::{collections::HashMap, fs};

fn main() {
    let text = fs::read_to_string("./inputs/day6.txt").unwrap();

    part1(&text);
    part2(&text);
}

fn part1(text: &str) {
    let instructions = text
        .lines()
        .flat_map(|line| line.split_whitespace().enumerate())
        .fold(
            HashMap::new(),
            |mut map: HashMap<usize, Vec<String>>, (index, item)| {
                map.entry(index).or_default().push(item.to_string());

                map
            },
        );

    let sum = instructions
        .values()
        .map(|column| {
            let operator = column.iter().find(|&x| x == "+" || x == "*").unwrap();
            apply_operator(column, operator)
        })
        .sum::<u64>();

    println!("{}", sum)
}

fn part2(text: &str) {
    let operators = text
        .lines()
        .last()
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect::<Vec<_>>();

    let columns = text.lines().flat_map(|line| line.char_indices()).fold(
        HashMap::new(),
        |mut map: HashMap<usize, Vec<char>>, (index, item)| {
            if item != '+' && item != '*' {
                map.entry(index).or_default().push(item);
            }

            map
        },
    );

    let sum = (0..columns.len())
        .map(|index| columns.get(&index).unwrap())
        .map(|chars| chars.iter().collect::<String>())
        .map(|string| string.trim().to_owned())
        .fold(Vec::new(), |mut blocks, string| {
            if string == "" {
                blocks.push(Vec::new());
            } else {
                if blocks.last() == None {
                    blocks.push(Vec::new());
                }
                let len = blocks.len();
                let mut last_block = blocks.last().unwrap().clone();
                last_block.push(string);
                blocks[len - 1] = last_block;
            }

            blocks
        })
        .iter()
        .zip(operators)
        .map(|(block, operator)| apply_operator(block, operator))
        .sum::<u64>();

    println!("{}", sum)
}

fn apply_operator(raw_numbers: &Vec<String>, operator: &str) -> u64 {
    raw_numbers
        .iter()
        .filter_map(|number| number.parse::<u64>().ok())
        .reduce(|a, b| match operator {
            "+" => a + b,
            "*" => a * b,
            _ => panic!("Unknown operator"),
        })
        .unwrap()
}
