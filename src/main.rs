use std::{collections::HashMap, fs};
use regex::Regex;

#[allow(dead_code)]
fn day1_part1() -> u32 {
    let mut lhs: Vec<u32> = Vec::new();
    let mut rhs: Vec<u32> = Vec::new();
    let filename = "inputs/day1.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    for part in contents.split("\n") {
        let data: Vec<&str> = part.split_whitespace().collect();
        if data.len() != 2 {
            continue;
        }
        lhs.push(data[0].trim().parse().expect("Parse error"));
        rhs.push(data[1].trim().parse().expect("Parse error"));
    }
    lhs.sort();
    rhs.sort();
    lhs.iter()
        .zip(rhs.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .reduce(|acc, x| acc+x)
        .unwrap_or(0)
}

#[allow(dead_code)]
fn day1_part2() -> u32 {
    let mut lhs: Vec<u32> = Vec::new();
    let mut rhs: HashMap<u32, u32> = HashMap::new();
    let filename = "inputs/day1.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    for part in contents.split("\n") {
        let data: Vec<&str> = part.split_whitespace().collect();
        if data.len() != 2 {
            continue;
        }
        lhs.push(data[0].trim().parse().expect("Parse error"));
        let number = data[1].trim().parse().expect("Parse error");
        *rhs.entry(number).or_insert(0) += 1;
    }
    let mut similarity = 0;
    for integer in lhs.iter() {
        match rhs.get(integer) {
            Some(count) => {
                similarity += integer*count;
            }
            None => continue,
        }
    }
    similarity
}

fn create_report(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|x| x.trim().parse::<i32>().expect("Parse error")).collect()
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return report.len() == 1;
    }
    let increasing = report[0] < report[1];
    for i in 0..(report.len()-1) {
        if increasing && report[i] >= report[i+1] {
            return false;
        }
        if !increasing && report[i] <= report[i+1] {
            return false;
        }
        if report[i].abs_diff(report[i+1]) > 3 {
            return false;
        }
    }
    true
}

fn exclude_index(report: &[i32], i: usize) -> Vec<i32> {
    let mut result = Vec::with_capacity(report.len() - 1);
    result.extend_from_slice(&report[..i]);
    result.extend_from_slice(&report[i + 1..]);
    result
}

fn is_safe_while_removing_problem(report: &[i32], i: usize) -> bool {
    let mut around = false;
    if i > 0 {
        around |= is_safe(&exclude_index(report, i-1));
    }
    if i+1 < report.len() {
        around |= is_safe(&exclude_index(report, i+1));
    }
    around || is_safe(&exclude_index(report, i))
}

fn is_safe_with_problem_dampener(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return report.len() == 1;
    }
    let increasing = report[0] < report[1];
    for i in 0..(report.len()-1) {
        if increasing && report[i] >= report[i+1] {
            return is_safe_while_removing_problem(report, i);
        }
        if !increasing && report[i] <= report[i+1] {
            return is_safe_while_removing_problem(report, i);
        }
        if report[i].abs_diff(report[i+1]) > 3 {
            return is_safe_while_removing_problem(report, i);
        }
    }
    true
}

#[allow(dead_code)]
fn day2_part1() -> usize {
    let filename = "inputs/day2.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    contents.split("\n")
        .map(create_report)
        .filter(|x| is_safe(x))
        .count()
}

#[allow(dead_code)]
fn day2_part2() -> usize {
    let filename = "inputs/day2.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    contents.split("\n")
        .map(create_report)
        .filter(|x| is_safe_with_problem_dampener(x))
        .count()
}

#[allow(dead_code)]
fn day3_part1() -> i32 {
    let filename = "inputs/day3.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    re.captures_iter(&contents)
        .map(|x| x[1].parse::<i32>().expect("Parse error") * x[2].parse::<i32>().expect("Parse error"))
        .sum()
}

fn filter_dont(contents: &str) -> String {
    let do_instruction = "do()";
    let dont_instruction = "don't()";
    let mut result = String::with_capacity(contents.len());
    let mut start: usize = 0;
    let mut add_end_of_file = true;
    loop {
        match contents[start..].find(dont_instruction) {
            None => break,
            Some(relative_dont_position) => {
                add_end_of_file = false;
                let dont_position = start + relative_dont_position;
                result.push_str(&contents[start..dont_position]);
                match contents[dont_position..].find(do_instruction) {
                    None => break,
                    Some(relative_do_position) => {
                        let do_position = dont_position + relative_do_position;
                        start = do_position + do_instruction.len() - 1;
                        add_end_of_file = true;
                    }
                }
            }
        }
    }
    if add_end_of_file && start < contents.len() {
        result.push_str(&contents[start..]);
    }
    result
}

#[allow(dead_code)]
fn day3_part2() -> i32 {
    let filename = "inputs/day3.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    re.captures_iter(filter_dont(&contents).as_str())
        .map(|x| x[1].parse::<i32>().expect("Parse error") * x[2].parse::<i32>().expect("Parse error"))
        .sum()
}

struct XmasAutomata {
    state: char,
}

const XMAS_AUTOMATA_BEGIN_STATE: char = 'X';
const XMAS_AUTOMATA_END_STATE: char = '\0';

impl Default for XmasAutomata {
    fn default() -> Self {
        XmasAutomata { state: XMAS_AUTOMATA_BEGIN_STATE }
    }
}

impl XmasAutomata {
    fn transit(&self, character: char) -> (bool, XmasAutomata) {
        let transitions = HashMap::from([
            (XMAS_AUTOMATA_BEGIN_STATE, 'M'),
            ('M', 'A'),
            ('A', 'S'),
            ('S', XMAS_AUTOMATA_END_STATE),
        ]);
        if self.state == character {
            (true, XmasAutomata { state: transitions[&self.state] })
        } else {
            (false, XmasAutomata { state: self.state })
        }
    }
}

fn get_indexes_around (i: usize, j: usize) -> Vec<(i32,i32)> {
    let i = i32::try_from(i).ok().unwrap();
    let j = i32::try_from(j).ok().unwrap();
    vec![
        (i-1, j-1),
        (i-1, j),
        (i-1, j+1),
        (i, j-1),
        (i, j+1),
        (i+1, j-1),
        (i+1, j),
        (i+1, j+1),
    ]
}

fn calculate_delta(initial: (usize, usize), actual: (usize, usize)) -> (i32, i32) {
    let (i, j) = initial;
    let (k, l) = actual;
    let i = i32::try_from(i).ok().unwrap();
    let j = i32::try_from(j).ok().unwrap();
    let k = i32::try_from(k).ok().unwrap();
    let l = i32::try_from(l).ok().unwrap();
    (k - i, l - j)
}

fn advance(initial: (usize, usize), direction: (i32, i32)) -> (i32, i32) {
    let (i, j) = initial;
    let (delta_i, delta_j) = direction;
    let i = i32::try_from(i).ok().unwrap();
    let j = i32::try_from(j).ok().unwrap();
    (i + delta_i, j + delta_j)
}

fn is_bounded(indices: (i32, i32), char_matrix: &Vec<Vec<char>>) -> bool {
    let (i, j) = indices;
    let len_i = i32::try_from(char_matrix.len()).ok().unwrap();
    let len_j = i32::try_from(char_matrix[0].len()).ok().unwrap();
    i >= 0 && i < len_i && j >= 0 && j < len_j
}

fn bound_indices(indices: (i32, i32), char_matrix: &Vec<Vec<char>>) -> (usize, usize) {
    let (i, j) = indices;
    let len_i = i32::try_from(char_matrix.len()).ok().unwrap();
    let len_j = i32::try_from(char_matrix[0].len()).ok().unwrap();
    let (i, j) = (i.max(0).min(len_i - 1), j.max(0).min(len_j-1));
    (usize::try_from(i).ok().unwrap(), usize::try_from(j).ok().unwrap())
}

fn explore_char_matrix(char_matrix: &Vec<Vec<char>>, i: usize, j: usize, xmas: XmasAutomata) -> usize {
    let mut total: usize = 0;
    for (k, l) in get_indexes_around(i, j) {
        if ! is_bounded((k, l), char_matrix) {
            continue;
        }
        let (k, l) = bound_indices((k, l), char_matrix);
        let (transitioned, new_xmas) = xmas.transit(char_matrix[k][l]);
        if transitioned {
            let direction = calculate_delta((i, j), (k, l));
            let mut m = k;
            let mut n = l;
            let mut transitioned;
            let mut new_xmas = new_xmas;
            loop {
                if new_xmas.state == XMAS_AUTOMATA_END_STATE {
                    total += 1;
                    break;
                }
                if ! is_bounded(advance((m, n), direction), char_matrix) {
                    break;
                }
                (m, n) = bound_indices(advance((m, n), direction), char_matrix);
                (transitioned, new_xmas) = new_xmas.transit(char_matrix[m][n]);
                if ! transitioned {
                    break;
                }
            }
        }
    }
    total
}

#[allow(dead_code)]
fn day4_part1() -> usize {
    let filename = "inputs/day4.txt";
    let contents = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let char_matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut total: usize = 0;
    for i in 0..char_matrix.len() {
        for j in 0..char_matrix[i].len() {
            if char_matrix[i][j] != XMAS_AUTOMATA_BEGIN_STATE {
                continue;
            }
            let (_, xmas) = XmasAutomata::default().transit(char_matrix[i][j]);
            total += explore_char_matrix(&char_matrix, i, j, xmas);
        }
    }
    total
}



fn main() {
    let result = day4_part1();
    println!("result={result}");
}
