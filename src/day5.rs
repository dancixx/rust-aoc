// use std::rc::Rc

pub fn run_own() {
    let input = std::fs::read_to_string("./src/datas/day5_input.txt").unwrap();
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut stacks = vec![];
    let mut instructions = vec![];

    for (index, line) in lines.clone().into_iter().enumerate() {
        let line = line.to_string() + " ";

        // line.chars().step_by(4).enumerate().for_each(|(index, _)| {
        //     let value = Rc::new(line.chars().nth(index * 4 + 1).unwrap().to_string());

        //     if value == Rc::new(" ".to_string()) {
        //         stack.push(Rc::new("0".to_string()));
        //         // stack.push(String::from("0"));
        //     } else {
        //         stack.push(Rc::clone(&value));
        //         // stack.push(Rc::clone(&value).as_str().to_string());
        //     }
        // });

        if index < 8 {
            let mut stack = vec![];

            line.chars().step_by(4).enumerate().for_each(|(index, _)| {
                let value = line.chars().nth(index * 4 + 1).unwrap().to_string();

                if value.clone() == " ".to_string() {
                    stack.push(String::from("0"));
                } else {
                    stack.push(value);
                }

                if line.chars().nth(1) == None {
                    return;
                }
            });

            stacks.push(stack);
        }

        if index > 9 {
            let mut instruction = vec![];

            line.split_whitespace().for_each(|c| {
                if let Some(value) = c.to_string().parse::<usize>().ok() {
                    instruction.push(value);
                };
            });

            instructions.push(instruction);
        }
    }

    let mut transposed_stacks: Vec<Vec<String>> =
        vec![vec!["0".to_string(); stacks.len()]; stacks[0].len()];

    for (idx, stack) in stacks.clone().iter().enumerate() {
        for (index, item) in stack.iter().enumerate() {
            transposed_stacks[index][stack.len() - 2 - idx] = item.clone();
        }
    }

    let transposed_stacks = transposed_stacks
        .iter()
        .map(|stack| {
            stack
                .iter()
                .filter(|i| i != &&"0")
                .collect::<Vec<&String>>()
        })
        .collect::<Vec<Vec<&String>>>();

    let solution = |is_reversed: bool| {
        let mut transposed_stacks = transposed_stacks.clone();

        instructions.iter().for_each(|i| {
            let quantity = i[0];
            let from = i[1] - 1;
            let to = i[2] - 1;

            let mut moved =
                transposed_stacks[from][transposed_stacks[from].len() - quantity..].to_vec();

            if is_reversed {
                moved.reverse();
            }

            transposed_stacks[to].append(&mut moved);
            transposed_stacks[from] =
                transposed_stacks[from][..transposed_stacks[from].len() - quantity].to_vec();
        });

        transposed_stacks
            .iter()
            .map(|stack| stack[stack.len() - 1])
            .collect::<Vec<&String>>()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("")
    };

    println!("Day 5 - Part 1: {:?}", solution(true));
    println!("Day 5 - Part 2: {:?}", solution(false));
}
