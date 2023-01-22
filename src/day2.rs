pub fn run_own() {
    let input = std::fs::read_to_string("src/datas/day2_input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let lines = lines
        .iter()
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut score = 0usize;

    for line in lines.clone() {
        if line[0] == "A" && line[1] == "X" {
            // It's draw and Rock
            score += 3 + 1;
        }

        if line[0] == "A" && line[1] == "Y" {
            // It's win and Paper
            score += 6 + 2;
        }

        if line[0] == "A" && line[1] == "Z" {
            // It's lose and Scissors
            score += 0 + 3;
        }

        if line[0] == "B" && line[1] == "X" {
            // It's lose and Rock
            score += 0 + 1;
        }

        if line[0] == "B" && line[1] == "Y" {
            // It's draw and Paper
            score += 3 + 2;
        }

        if line[0] == "B" && line[1] == "Z" {
            // It's win and Scissors
            score += 6 + 3;
        }

        if line[0] == "C" && line[1] == "X" {
            // It's win and Rock
            score += 6 + 1;
        }

        if line[0] == "C" && line[1] == "Y" {
            // It's lose and Paper
            score += 0 + 2;
        }

        if line[0] == "C" && line[1] == "Z" {
            // It's draw and Scissors
            score += 3 + 3;
        }
    }

    println!("Day 2 - Part 1: {:?}", score);

    let mut score = 0usize;

    for line in lines.clone() {
        if line[0] == "A" && line[1] == "X" {
            // Its a loose but need Sciccors
            score += 0 + 3;
        }

        if line[0] == "A" && line[1] == "Y" {
            // Its a draw but need Rock
            score += 3 + 1;
        }

        if line[0] == "A" && line[1] == "Z" {
            // Its a win but need Paper
            score += 6 + 2;
        }

        if line[0] == "B" && line[1] == "X" {
            // Its a loose but need Rock
            score += 0 + 1;
        }

        if line[0] == "B" && line[1] == "Y" {
            // Its a draw but need Paper
            score += 3 + 2;
        }

        if line[0] == "B" && line[1] == "Z" {
            // Its a win but need Scissors
            score += 6 + 3;
        }

        if line[0] == "C" && line[1] == "X" {
            // Its a loose but need Paper
            score += 0 + 2;
        }

        if line[0] == "C" && line[1] == "Y" {
            // Its a draw but need Scissors
            score += 3 + 3;
        }

        if line[0] == "C" && line[1] == "Z" {
            // Its a win but need Rock
            score += 6 + 1;
        }
    }

    println!("Day 2 - Part 2: {:?}", score);
}
