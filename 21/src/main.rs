use std::cmp;
use cached::proc_macro::cached;
use cached::Return;

fn main() {
    game_with_deterministic_die();
    game_with_dirac_die();
}

fn game_with_deterministic_die() {
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
        for _ in 0..3 {
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct DiracBundle {
    pos_p1: u16,
    pos_p2: u16,
    turn: Turn,
    score_p1: u16,
    score_p2: u16,
    die_val: u16
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Turn {
    P1,
    P2,
    First
}

fn game_with_dirac_die() {
    let gamestate = DiracBundle {
        score_p1: 0,
        score_p2: 0,
        pos_p1: 4, // Sample
        pos_p2: 8,
        // pos_p1: 5, // Input
        // pos_p2: 9,
        turn: Turn::First,
        die_val: 0
    };

    let (p1_wins, p2_wins) = *find_wins(gamestate);

    println!("Player 1 won {}", p1_wins);
    println!("Player 2 won {}", p2_wins);
}

#[cached(with_cached_flag = true)]
fn find_wins(gamestate: DiracBundle) -> Return<(u64, u64)> {
    if gamestate.score_p1 >= 1000 || gamestate.score_p1 >= 1000 {
        return match gamestate.score_p1 > gamestate.score_p2 {
            true => Return::new((1, 0)),
            false => match gamestate.score_p2 > gamestate.score_p1 {
                true => Return::new((0, 1)),
                false => Return::new((0, 0))
            }
        }
    }

    let mut next = gamestate.clone();
    match gamestate.turn {
        Turn::P1 => {
            next.pos_p1 = (gamestate.pos_p1 + gamestate.die_val - 1) % 10 + 1;
            next.score_p1 += gamestate.pos_p1;
            next.turn = Turn::P2;
        },
        Turn:: P2 => {
            next.pos_p2 = (gamestate.pos_p2 + gamestate.die_val - 1) % 10 + 1;
            next.score_p2 += gamestate.pos_p2;
            next.turn = Turn::P1;
        },
        Turn::First => {
            next.turn = Turn::P1;
        }
    }

    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    // 27 universes fork here, one for each combination of 3-sided dice.
    for roll1 in 1..=3 {
        for roll2 in 1..=3 {
            for roll3 in 1..=3 {
                let mut next_with_roll = next.clone();
                next_with_roll.die_val = roll1 + roll2 + roll3;

                let res = find_wins(next_with_roll);

                let (next_p1_wins, next_p2_wins) = *res;
                // if !res.was_cached {
                //     println!("Running tally... p1 wins {} + {} times, p2 wins {} + {} times", p1_wins, next_p1_wins, p2_wins, next_p2_wins);
                // }
                if p1_wins.checked_add(next_p1_wins).is_none() || p2_wins.checked_add(next_p2_wins).is_none() {
                    println!("Won't be able to add\n{} + {} or\n{} + {}", p1_wins, next_p1_wins, p2_wins, next_p2_wins);
                    println!("It should be only\n{} vs.\n{}", 444356092776315u64, 341960390180808u64);
                }
                p1_wins += next_p1_wins;
                p2_wins += next_p2_wins;
            }
        }
    }

    return Return::new((p1_wins, p2_wins));
}