use good_lp::{Expression, Solution, SolverModel, constraint, scip, variable, variables};
use std::fs;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<u8>>,
    joltages: Vec<u8>,
}

impl Machine {
    fn parse(string: &str) -> Option<Machine> {
        let (lights_raw, rest) = string.split_once("] ")?;
        let lights = lights_raw
            .strip_prefix("[")
            .iter()
            .flat_map(|string| string.chars())
            .map(|symbol| if symbol == '#' { true } else { false })
            .collect::<Vec<_>>();

        let (buttons_raw, joltages_raw) = rest.split_once(" {")?;
        let buttons = buttons_raw
            .split(" ")
            .filter_map(|part| {
                part.strip_prefix("(")
                    .and_then(|rest| rest.strip_suffix(")"))
            })
            .map(|string| {
                string
                    .split(",")
                    .map(|part| part.parse::<u8>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let joltages = joltages_raw
            .strip_suffix("}")
            .iter()
            .flat_map(|string| string.split(","))
            .map(|part| part.parse::<u8>().unwrap())
            .collect::<Vec<_>>();

        Some(Machine {
            lights,
            buttons,
            joltages,
        })
    }
}

fn main() {
    let text = fs::read_to_string("./inputs/day10.txt").unwrap();
    let machines = text
        .lines()
        .map(Machine::parse)
        .map(|machine| machine.unwrap())
        .collect::<Vec<_>>();

    part1(&machines);
    part2(&machines);
}

fn part1(machines: &Vec<Machine>) {
    let sum = machines
        .iter()
        .map(|machine| {
            let mut variables = variables!();
            let buttons = machine
                .buttons
                .iter()
                .map(|_| variables.add(variable().min(0).integer()))
                .collect::<Vec<_>>();

            let is_even = machine
                .lights
                .iter()
                .map(|_| variables.add(variable().min(0).integer()))
                .collect::<Vec<_>>();

            let objective = buttons.iter().sum::<Expression>();
            let mut problem = variables.minimise(objective.clone()).using(scip);
            for (index, light) in machine.lights.iter().enumerate() {
                let button_sum = machine
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|(_, lights)| lights.contains(&(index as u8)))
                    .map(|(button_index, _)| buttons[button_index])
                    .sum::<Expression>();

                problem
                    .add_constraint(constraint!(button_sum == 2 * is_even[index] + *light as u8));
            }

            match problem.solve() {
                Ok(solution) => solution.eval(objective).round() as u32,
                Err(_) => panic!("Unsolvable"),
            }
        })
        .sum::<u32>();

    println!("{}", sum);
}

fn part2(machines: &Vec<Machine>) {
    let sum = machines
        .iter()
        .map(|machine| {
            let mut variables = variables!();
            let buttons = machine
                .buttons
                .iter()
                .map(|_| variables.add(variable().min(0).integer()))
                .collect::<Vec<_>>();

            let objective = buttons.iter().sum::<Expression>();
            let mut problem = variables.minimise(objective.clone()).using(scip);
            for (index, joltage) in machine.joltages.iter().enumerate() {
                let button_sum = machine
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|(_, joltages)| joltages.contains(&(index as u8)))
                    .map(|(button_index, _)| buttons[button_index])
                    .sum::<Expression>();

                problem.add_constraint(constraint!(button_sum == *joltage));
            }

            match problem.solve() {
                Ok(solution) => solution.eval(objective).round() as u32,
                Err(_) => panic!("Unsolvable"),
            }
        })
        .sum::<u32>();

    println!("{}", sum);
}
