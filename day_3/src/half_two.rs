pub fn half_two() -> i32 {
    let mut total_prio = 0;
    let input_string = std::fs::read_to_string("input/input.txt").expect("Couldn't read input");
    let input = input_string.split("\n");

    let mut group_of_three = ["", "", ""];

    let mut index = 0;
    let mut num_one = "";
    let mut num_two = "";


    for backpack in input {
        if index < 2 {
            group_of_three[index] = backpack;

            match index {
                0 => num_one = backpack,
                1 => num_two = backpack,
                _ => ()
            }
            index += 1;
        } else {
            group_of_three[2] = backpack;

            let mut i = 0;

            if num_two.len() > num_one.len() && num_two.len() > group_of_three[2].len() {
                i = 1;
            } else if group_of_three[2].len() > num_one.len() && group_of_three[2].len() > num_two.len() {
                i = 2;
            }

            for char in group_of_three[i].chars() {
                match i {
                    0 => {
                        if num_two.contains(char) && group_of_three[2].contains(char) {
                            if char.is_ascii_uppercase() {
                                let added_prio = char as i32 - 'A' as i32 + 27;
                                total_prio += added_prio;
                            } else {
                                let added_prio = char as i32 - 'a' as i32 + 1;
                                total_prio += added_prio;
                            }
                            break;
                        }
                    },
                    1 => {
                        if num_one.contains(char) && group_of_three[2].contains(char) {
                            if char.is_ascii_uppercase() {
                                let added_prio = char as i32 - 'A' as i32 + 27;
                                total_prio += added_prio;
                            } else {
                                let added_prio = char as i32 - 'a' as i32 + 1;
                                total_prio += added_prio;
                            }
                            break;
                        }
                    },
                    2 => {
                        if num_one.contains(char) && num_two.contains(char) {
                            if char.is_ascii_uppercase() {
                                let added_prio = char as i32 - 'A' as i32 + 27;
                                total_prio += added_prio;
                            } else {
                                let added_prio = char as i32 - 'a' as i32 + 1;
                                total_prio += added_prio;
                            }
                            break;
                        }
                    },
                    _ => ()
                }
            }
            index = 0;
        }
    }

    total_prio
}
