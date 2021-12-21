use std::cmp;

fn main() {
    //let mut pos_p1 = 4; // Sample
    //let mut pos_p2 = 8;
    let mut pos_p1 = 5; // Input
    let mut pos_p2 = 9;
    let mut score_p1 = 0;
    let mut score_p2 = 0;
    let mut die = 1;
    let mut total_rolls = 0;
    let mut is_p1_turn = true;

    while score_p1 < 1000 && score_p2 < 1000 {
        let mut die_roll = 0;
        for roll_for_player in 0..3 {
            die_roll += die;
            die = (die % 100) + 1;
            total_rolls += 1;
        }
        if is_p1_turn {
            pos_p1 = (pos_p1 + die_roll - 1) % 10 + 1;
            score_p1 += pos_p1;
            is_p1_turn = false;
        } else {
            pos_p2 = (pos_p2 + die_roll - 1) % 10 + 1;
            score_p2 += pos_p2;
            is_p1_turn = true;
        }
    }

    let min_score = cmp::min(score_p2, score_p1);
    let final_tally = total_rolls * min_score;
    println!("Player 1: {}", score_p1);
    println!("Player 2: {}", score_p2);
    println!("Final: total rolls {} * min_score {} = {}", total_rolls, min_score, final_tally);
}
