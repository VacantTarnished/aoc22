
pub fn half_one() -> i32 {
    let input_string = std::fs::read_to_string("input/input.txt").expect("Couldn't read input");
    let input = input_string.split("\n");
    let points_rock = 1;
    let points_paper = 2;
    let points_scissors = 3;
    let points_loose = 0;
    let points_draw = 3;
    let points_win = 6;


    let mut total_points = 0;

    // A Rock B Paper C Scissors Enemy 
    // X Rock Y Paper Z Scissors Self

    for game in input {
        match game {
            "A X" => total_points += points_draw + points_rock,
            "A Y" => total_points += points_win + points_paper,
            "A Z" => total_points += points_loose + points_scissors,
            "B X" => total_points += points_loose + points_rock,
            "B Y" => total_points += points_draw + points_paper,
            "B Z" => total_points += points_win + points_scissors,
            "C X" => total_points += points_win + points_rock,
            "C Y" => total_points += points_loose + points_paper,
            "C Z" => total_points += points_draw + points_scissors,
            _ => {}
        }
    }

    total_points
}