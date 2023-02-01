use array_tool::vec::Uniq;

pub fn run() {
    let start = std::time::Instant::now();
    let input = std::fs::read("./src/datas/day6_input.txt").unwrap();
    println!("Time to read: {:?}", start.elapsed());
    let mut data = Vec::from([0, 0, 0, 0]);
    let mut idx = 0;

    let start = std::time::Instant::now();
    for (index, _item) in input.iter().enumerate() {
        if data.is_unique() {
            break;
        }

        data[0] = input[index];
        data[1] = input[index + 1];
        data[2] = input[index + 2];
        data[3] = input[index + 3];

        if index > 0 {
            idx += 1;
        }
    }
    println!("Time to run: {:?}", start.elapsed());
    println!("Input: {:?}, idx: {:?}", data, idx);

    let start = std::time::Instant::now();
    let input = std::fs::read_to_string("./src/datas/day6_input.txt")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    println!("Time to read: {:?}", start.elapsed());
    let mut data = Vec::from(['0', '0', '0', '0']);
    let mut idx = 0;

    let start = std::time::Instant::now();
    for (index, _item) in input.iter().enumerate() {
        if data.is_unique() {
            break;
        }

        data[0] = input[index];
        data[1] = input[index + 1];
        data[2] = input[index + 2];
        data[3] = input[index + 3];

        if index > 0 {
            idx += 1;
        }
    }
    println!("Time to run: {:?}", start.elapsed());
    println!("Input: {:?}, idx: {:?}", data, idx);

    // Part 2
    let start = std::time::Instant::now();
    let input = std::fs::read("./src/datas/day6_input.txt").unwrap();
    println!("Time to read: {:?}", start.elapsed());
    let mut data = Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    let mut idx = 0;

    let start = std::time::Instant::now();
    for (index, _item) in input.iter().enumerate() {
        if data.is_unique() {
            break;
        }

        data[0] = input[index];
        data[1] = input[index + 1];
        data[2] = input[index + 2];
        data[3] = input[index + 3];
        data[4] = input[index + 4];
        data[5] = input[index + 5];
        data[6] = input[index + 6];
        data[7] = input[index + 7];
        data[8] = input[index + 8];
        data[9] = input[index + 9];
        data[10] = input[index + 10];
        data[11] = input[index + 11];
        data[12] = input[index + 12];
        data[13] = input[index + 13];

        if index > 0 {
            idx += 1;
        }
    }
    println!("Time to run: {:?}", start.elapsed());
    println!("Input: {:?}, idx: {:?}", data, idx);
}
