pub fn half_one() -> i32 {
    let mut total_prio = 0;
    let input_string = std::fs::read_to_string("input/input.txt").expect("Couldn't read input");
    let input = input_string.split("\n");

    for backpack in input {
        let mut half_one = "".to_string();
        let mut half_two = "".to_string();
        let mut index = 0;
        let half_way = backpack.len() / 2;
        for char in backpack.chars() {
            if index < half_way {
                half_one.push(char);
            } else {
                half_two.push(char);
            }
            index += 1;
        }


        for char in half_one.chars() {
            if half_two.contains(char) {

                if char.is_ascii_uppercase() {
                    let added_prio = char as i32 - 'A' as i32 + 27;
                    total_prio += added_prio;
                } else {
                    let added_prio = char as i32 - 'a' as i32 + 1;
                    total_prio += added_prio;
                }

                break;
            }
        }
    }


    total_prio
}