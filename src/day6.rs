use std::{collections::HashSet, sync::mpsc::Receiver};

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

        idx = index;
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

        idx = index;
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

        idx = index;
    }
    println!("Time to run: {:?}", start.elapsed());
    println!("Input: {:?}, idx: {:?}", data, idx);

    // Part 2
    let start = std::time::Instant::now();
    let input = std::fs::read("./src/datas/day6_input.txt").unwrap();
    println!("Time to read: {:?}", start.elapsed());
    let mut data = Vec::new();
    let count = 14;
    let mut idx = 0;

    let start = std::time::Instant::now();
    for (index, item) in input.iter().enumerate() {
        data.push(item);

        if data.len() == count {
            let chars = data.clone().into_iter().collect::<HashSet<&u8>>();
            if chars.len() == count {
                break;
            }
            data.remove(0);
        }

        idx = index;
    }
    println!("Time to run: {:?}", start.elapsed());
    println!("Input: {:?}, idx: {:?}", data, idx);

    let start = std::time::Instant::now();
    simple(&std::fs::read("./src/datas/day6_input.txt").unwrap(), count);
    println!("Time to run: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    faster_vec(&std::fs::read("./src/datas/day6_input.txt").unwrap(), count);
    println!("Time to run: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    faster_array(&std::fs::read("./src/datas/day6_input.txt").unwrap(), count);
    println!("Time to run: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    david_a_perez(&std::fs::read("./src/datas/day6_input.txt").unwrap());
    println!("Time to run: {:?}", start.elapsed());

    let data = Box::leak(
        std::fs::read("./src/datas/day6_input.txt")
            .unwrap()
            .into_boxed_slice(),
    );
    let start = std::time::Instant::now();
    david_a_perez_async(data, 10);
    println!("Time to run: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    benny(&*std::fs::read("./src/datas/day6_input.txt").unwrap());
    println!("Time to run: {:?}", start.elapsed());
}

fn simple(input: &[u8], count: usize) -> usize {
    input
        .windows(count)
        .position(|window| window.iter().collect::<HashSet<&u8>>().len() == count)
        .map(|index| index + count)
        .unwrap()
}

fn faster_vec(input: &[u8], count: usize) -> usize {
    input
        .windows(count)
        .position(|window| {
            let mut vec = Vec::with_capacity(count);

            for item in window {
                if vec.contains(item) {
                    return false;
                }

                vec.push(*item);
            }

            return true;
        })
        .map(|index| index + count)
        .unwrap()
}

fn faster_array(input: &[u8], count: usize) -> usize {
    input
        .windows(count)
        .position(|window| {
            let mut arr = [0u8; 14];
            let mut idx = 0;

            for item in window {
                for i in 0..idx {
                    if arr[i] == *item {
                        return false;
                    }
                }

                arr[idx] = *item;
                idx += 1;
            }

            return true;
        })
        .map(|index| index + count)
        .unwrap()
}

fn join(rx: Receiver<Option<usize>>, max: usize) -> Option<usize> {
    let mut found = 0;
    while let Ok(x) = rx.recv() {
        if let Some(x) = x {
            return Some(x);
        }
        found += 1;

        if found == max {
            return None;
        }
    }

    return None;
}

fn david_a_perez_async(data: &'static [u8], cpus: usize) -> Option<usize> {
    let regions = data.len() / cpus;
    let (tx, rx) = std::sync::mpsc::channel();
    for i in 0..cpus {
        let start = if i == 0 { 0 } else { i * regions - 14 };
        let inner_tx = tx.clone();
        let mut len = regions;

        if i == regions - 1 {
            len = data.len();
        }

        std::thread::spawn(move || {
            if let Some(x) = david_a_perez(&data[start..start + len]) {
                _ = inner_tx.send(Some(x + start));
            } else {
                _ = inner_tx.send(None);
            }
        });
    }

    return join(rx, cpus);
}

pub fn david_a_perez(input: &[u8]) -> Option<usize> {
    let mut idx = 0;
    while let Some(slice) = input.get(idx..idx + 14) {
        let mut state = 0u32;

        if let Some(pos) = slice.iter().rposition(|byte| {
            let bit_idx = byte % 32;
            let ret = state & (1 << bit_idx) != 0;
            state |= 1 << bit_idx;
            ret
        }) {
            idx += pos + 1;
        } else if state.count_ones() == 14 as u32 {
            return Some(idx);
        }
    }
    return None;
}

pub fn benny(input: &[u8]) -> Option<usize> {
    let mut filter = 0u32;
    input
        .iter()
        .take(14 - 1)
        .for_each(|c| filter ^= 1 << (c % 32));

    input.windows(14).position(|w| {
        let first = w[0];
        let last = w[w.len() - 1];
        filter ^= 1 << (last % 32);
        let res = filter.count_ones() == 14 as u32;
        filter ^= 1 << (first % 32);
        res
    })
}
