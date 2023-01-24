use itertools::Itertools;

pub fn run_own() {
    let input = std::fs::read_to_string("src/datas/day4_input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let lines = lines
        .iter()
        .map(|s| s.split(",").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();
    let lines = lines
        .iter()
        .map(|s| {
            s.iter()
                .map(|s| {
                    s.split("-")
                        .into_iter()
                        // TODO: map is faster than flat_map
                        .map(|s| s.parse::<usize>().unwrap())
                        // .flat_map(str::parse::<usize>)
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect::<Vec<Vec<Vec<usize>>>>();

    let mut score = 0usize;

    for line in lines.clone() {
        if line[0][0] <= line[1][0] && line[0][1] >= line[1][1] {
            score += 1;
            continue;
        }

        if line[1][0] <= line[0][0] && line[1][1] >= line[0][1] {
            score += 1;
            continue;
        }
    }

    println!("Day 4 - Part 1: {:?}", score);

    let mut indexes = vec![];

    for (index, line) in lines.clone().iter().enumerate() {
        for i in line[0][0]..=line[0][1] {
            if i == line[1][0] || i == line[1][1] {
                indexes.push(index);
            }
        }

        for i in line[1][0]..=line[1][1] {
            if i == line[0][0] || i == line[0][1] {
                indexes.push(index);
            }
        }
    }

    println!("Day 4 - Part 2: {:?}", indexes.into_iter().unique().count());
}
