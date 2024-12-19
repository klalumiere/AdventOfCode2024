use std::{collections::HashMap, fs};

fn parse_letter_to_towels(content: &str) -> HashMap<char, Vec<String>> {
    let mut letter_to_towels: HashMap<char, Vec<String>> = HashMap::new();
    for line in content.lines() {
        if line.is_empty() {
            break;
        }
        for towel in line.split(", ") {
            let first_letter = towel.chars().next().expect("a first letter");
            let letter_to_towels = letter_to_towels.entry(first_letter).or_insert_with(Vec::new);
            letter_to_towels.push(String::from(towel));
        }
    }
    letter_to_towels
}

fn parse_patterns(content: &str) -> Vec<String> {
    let mut lines = content.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
    }
    lines.map(String::from).collect()
}

fn find_towels(word: &str, letter_to_towels: &HashMap<char, Vec<String>>, cache: & mut HashMap<String, Option<Vec<String>>>) -> Option<Vec<String>> {
    if word.is_empty() {
        return Some(Vec::new());
    }
    if cache.contains_key(word) {
        return cache.get(word).unwrap().clone();
    }
    
    let next_letter = word.chars().next().expect("a first letter");
    let available_towels = letter_to_towels.get(&next_letter);
    match available_towels {
        None => {
            cache.insert(String::from(word), None);
            None
        },
        Some(towels) => {
            let mut solution = Vec::new();
            for towel in towels {
                if word.starts_with(towel) {
                    solution.push(towel.clone());
                    match find_towels(&word[towel.len()..], letter_to_towels, cache) {
                        None => continue,
                        Some(new_towels) => {
                            solution.extend(new_towels);
                            cache.insert(String::from(word), Some(solution.clone()));
                            return Some(solution);
                        }
                    }
                }
            }
            cache.insert(String::from(word), None);
            None
        }
    }
}

pub fn run() -> usize {
    let filename = "inputs/day19.txt";
    let content = fs::read_to_string(filename).expect("Can't read file '{filename}'");
    let letter_to_towels = parse_letter_to_towels(&content);
    let patterns = parse_patterns(&content);
    let mut cache: HashMap<String, Option<Vec<String>>> = HashMap::new();
    patterns.iter().map(|pattern| find_towels(pattern, &letter_to_towels, & mut cache)).filter(|x| x.is_some()).count()
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_step_forward() {
//         assert_eq!(1, 1);
//     }
// }



