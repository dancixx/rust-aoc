pub fn run_own() {
    let input = std::fs::read_to_string("src/datas/day1_input.txt").unwrap();
    let lines = input.split("\n\n").collect::<Vec<&str>>();

    let numbers = lines
        .iter()
        .map(|s| 
            // TODO: here can be used s.lines()
            s.split("\n")
            // TODO: here can be used flat_map, but we dont need the unwrap in this case
            .map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    let max = numbers.iter().max().unwrap();

    println!("Day 1 - Part 1: {:?}", max);
}

pub fn run_prime_agen() {
    let input = std::fs::read_to_string("src/datas/day1_input.txt").unwrap();
    let lines = input.split("\n\n");

    let numbers = lines.map(|s| s.lines().flat_map(str::parse::<u32>).sum::<u32>());

    let max = numbers.max().unwrap();

    println!("Day 1 - Part 1: {:?}", max);
}
