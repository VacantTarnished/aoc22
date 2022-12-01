use std::collections::LinkedList;

pub fn half_two() -> i32 {
    let input_string = std::fs::read_to_string("input/day_one.txt").expect("Couldn't read the Input File").replace("\n\n", " ");

    let input = input_string.split(" ");

    let mut all_cals: LinkedList<i32> = LinkedList::new();

    for elf in input {
        let mut calories: i32 = 0;
        for unparsable_cal in elf.split("\n") {
            let single_cal = unparsable_cal.replace("\n", "");
            
            calories += single_cal.parse::<i32>().expect("Couldn't Parse single calories");
        }
        all_cals.push_back(calories);
    }


    let mut vec: Vec<i32> = all_cals.into_iter().collect();
    vec.sort();
    let mut sorted_cals: LinkedList<i32> = vec.into_iter().collect();

    sorted_cals.pop_back().expect("Couldn't pop back of List") + sorted_cals.pop_back().expect("Couldn't pop back of List") + sorted_cals.pop_back().expect("Couldn't pop back of List")
}