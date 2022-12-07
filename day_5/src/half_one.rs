use std::{collections::{LinkedList, HashMap}};

struct Move {
    amt: usize,
    from: usize,
    to: usize
}

pub fn half_one() -> String {
    let string_input = std::fs::read_to_string("input/input.txt").expect("Couldn't read Input");
    let mut top_crates = "".to_owned();

    let mut i: usize = 0;
    let crate_indexes: HashMap<usize,usize> = ([1,5,9,13,17,21,25,29,33] as [usize; 9]).into_iter().map(|index| { i+=1; (index,i) }).collect();

    let mut crates: [LinkedList<char>;9] = [
        LinkedList::new(),
        LinkedList::new(),
        LinkedList::new(),
        LinkedList::new(),
        LinkedList::new(),
        LinkedList::new(),
        LinkedList::new(),
        LinkedList::new(),
        LinkedList::new()
    ];


    // 1   5   9   13  17  21  25  29  33
 

    for line in string_input.split("\n") {
        if line.trim_start().starts_with('[') {
            let crate_line = line.char_indices();

            crate_line
                .filter(
                    |pair| 
                    crate_indexes.contains_key(&pair.0) &&  pair.1.is_alphabetic())
                .for_each(
                    |pair| 
                    crates[crate_indexes.get(&pair.0).expect("").to_owned() - 1].push_back(pair.1));
        } else if line.starts_with("move") {
            let split_line = line.split(" ");

            let moves: Vec<usize> = split_line
            .filter(|element| element.parse::<usize>().is_ok())
            .map(|element| element.parse::<usize>().expect(""))
            .collect();

            let moves: Move = Move { amt: moves.get(0).expect("Couldn't get amt").clone(), from: moves.get(1).expect("Couldn't get from").clone(), to: moves.get(2).expect("Couldn't get to").clone() };


            for _ in 0..moves.amt {
                let temp = crates[moves.from - 1].pop_front().expect("");
                crates[moves.to - 1].push_front(temp);
            }
        }
    }



    for element in crates {
        top_crates = format!("{}{}", top_crates,element.front().expect(""))
    }

    top_crates
}