pub fn half_one() -> i32 {
    let input_string = std::fs::read_to_string("input/input.txt").expect("Couldn't read input file");
    let input = input_string.split("\n");
    let mut total_pairs = 0;

    for pair in input {
        let mut pair_one = "";
        for split_pair in pair.split(",") {
            if pair_one.is_empty() {
                pair_one = split_pair
            } else {
                let mut split_one = pair_one.split("-");
                let mut split_two = split_pair.split("-");
                let section_oo = split_one.next().expect("Couldn't get split value").parse::<i32>().expect("Couldn't parse split value");
                let section_ot = split_one.next().expect("Couldn't get split value").parse::<i32>().expect("Couldn't parse split value");
                let section_to = split_two.next().expect("Couldn't get split value").parse::<i32>().expect("Couldn't parse split value");
                let section_tt = split_two.next().expect("Couldn't get split value").parse::<i32>().expect("Couldn't parse split value");


                if (section_oo..section_ot).contains(&section_to) {
                    total_pairs += 1;
                } else if (section_oo..section_ot).contains(&section_tt) {
                    total_pairs += 1;
                }
            }    
        }
    }

    total_pairs
}