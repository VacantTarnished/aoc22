use std::collections::HashSet;

pub fn half_one() -> usize {
    let string_input =  std::fs::read_to_string("input/input.txt").expect("Couldn't read Input");
    let mut four_diff_after: usize = 0;
    let mut recent_four: Vec<char> = Vec::new();


    for char in string_input.chars() {
        four_diff_after+=1;

        if recent_four.len() < 3 {
            recent_four.push(char);
        }
        else {
            recent_four.push(char);
            let diff = recent_four.iter().collect::<HashSet<&char>>().len() == 4;
            if diff {
                return four_diff_after;
            }

            recent_four = recent_four.iter().skip(1).collect::<String>().chars().collect();
        }
    }

    panic!("No four different characters in a row found")
}