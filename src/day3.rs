use itertools::Itertools;

pub fn run_own() {
    let alphabet_uppercase = (b'A'..=b'Z').map(|c| c as char).collect::<Vec<char>>();
    let alphabet_lowercase = (b'a'..=b'z').map(|c| c as char).collect::<Vec<char>>();
    let merged_alphabet = alphabet_lowercase
        .iter()
        .chain(alphabet_uppercase.iter())
        .collect::<Vec<&char>>();

    let input = std::fs::read_to_string("src/datas/day3_input.txt").unwrap();
    let lines = input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.split_at(s.len() / 2))
        .collect::<Vec<(&str, &str)>>();

    let mut letters = vec![];

    for line in lines.clone() {
        let l1 = line
            .0
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .unique()
            .collect::<Vec<char>>();
        let l2 = line
            .1
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .unique()
            .collect::<Vec<char>>();

        for c in l1 {
            if l2.contains(&c) {
                letters.push(c);
            }
        }
    }

    let mut score = 0usize;

    for item in letters.clone() {
        if item.is_uppercase() {
            score += merged_alphabet.iter().position(|&r| r == &item).unwrap() + 1;
        } else {
            score += merged_alphabet.iter().position(|&r| r == &item).unwrap() + 1;
        }
    }

    println!("Day 3 - Part 1: {:?}", score);

    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut grid_lines = vec![];

    for n in (0..=lines.len() - 1).step_by(3) {
        grid_lines.push(vec![lines[n], lines[n + 1], lines[n + 2]]);
    }

    let mut letters = vec![];

    for item in grid_lines.clone() {
        let l1 = item[0]
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .unique()
            .collect::<Vec<char>>();
        let l2 = item[1]
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .unique()
            .collect::<Vec<char>>();
        let l3 = item[2]
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .unique()
            .collect::<Vec<char>>();

        let mut l = ' ';

        for c in l1 {
            if l2.contains(&c) && l3.contains(&c) {
                l = c;
            }
        }

        letters.push(l);
    }

    let mut score = 0usize;

    for item in letters.clone() {
        if item.is_uppercase() {
            score += merged_alphabet.iter().position(|&r| r == &item).unwrap() + 1;
        } else {
            score += merged_alphabet.iter().position(|&r| r == &item).unwrap() + 1;
        }
    }

    println!("Day 3 - Part 2: {:?}", score);
}
