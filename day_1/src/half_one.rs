pub fn half_one() -> i32 {
    let input_string = std::fs::read_to_string("input/day_one.txt").expect("Couldn't read the Input File").replace("\n\n", " ");

    let input = input_string.split(" ");

    let mut max_calories = 0;

    for elf in input {
        let mut calories: i32 = 0;
        for unparsable_cal in elf.split("\n") {
            let single_cal = unparsable_cal.replace("\n", "");
            
            calories += single_cal.parse::<i32>().expect("Couldn't Parse single calories");
        }

        if calories > max_calories  {
            max_calories = calories;
        }
    }

    max_calories
}