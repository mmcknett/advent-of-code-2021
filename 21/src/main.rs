use std::cmp;
use cached::proc_macro::cached;
use cached::Return;

fn main() {
    game_with_deterministic_die();
    game_deterministic_recursive();
    println!("\nPart 2...\n");
    game_with_dirac_die();
}

fn game_with_deterministic_die() {
    let mut pos_p1 = 4; // Sample
    let mut pos_p2 = 8;
    // let mut pos_p1 = 5; // Input
    // let mut pos_p2 = 9;
    let mut score_p1 = 0;
    let mut score_p2 = 0;
    let mut total_rolls = 0;
    let mut turn = Turn::First;
    let mut die_val = 0;

    while !(score_p1 >= 1000 || score_p2 >= 1000 ) {
        // println!("p1: {}, p2: {}", score_p1, score_p2);

        match turn {
            Turn::P1 => {
                pos_p1 = (pos_p1 + die_val - 1) % 10 + 1;
                score_p1 += pos_p1;
                turn = Turn::P2;
                // println!("P2 new pos: {}, new score: {}", pos_p1, score_p1);
            },
            Turn:: P2 => {
                pos_p2 = (pos_p2 + die_val - 1) % 10 + 1;
                score_p2 += pos_p2;
                turn = Turn::P1;
                // println!("P2 new pos: {}, new score: {}", pos_p2, score_p2);
            },
            Turn::First => {
                turn = Turn::P1;
            }
        }

        if score_p1 < 1000 && score_p2 < 1000 {
            die_val = 0;
            for i in 0..3 {
                let die_roll = (total_rolls + i) % 100 + 1;
                die_val += die_roll;
                // println!("die roll {}, die val {}", die_roll, die_val);
            }
            total_rolls += 3
        }
    }

    let min_score = cmp::min(score_p2, score_p1);
    let final_tally = total_rolls * min_score;
    println!("Player 1: {}", score_p1);
    println!("Player 2: {}", score_p2);
    println!("Final: total rolls {} * min_score {} = {}", total_rolls, min_score, final_tally);
}

fn game_deterministic_recursive() {
    let gamestate = Gamestate {
        score_p1: 0,
        score_p2: 0,
        pos_p1: 4, // Sample
        pos_p2: 8,
        // pos_p1: 5, // Input
        // pos_p2: 9,
        turn: Turn::First,
        die_val: 0,
        total_rolls: 0
    };

    let (p1_wins, p2_wins) = *find_wins_deterministic(gamestate);

    println!("Player 1 won {}", p1_wins);
    println!("Player 2 won {}", p2_wins);
}

// Made the deterministic version to help troubleshoot the dirac version
#[cached(with_cached_flag = true)]
fn find_wins_deterministic(gamestate: Gamestate) -> Return<(u64, u64)> {
    if gamestate.score_p1 >= 1000 || gamestate.score_p2 >= 1000 {
        let min_score = cmp::min(gamestate.score_p2, gamestate.score_p1);
        println!("\nRecursive version...");
        println!("Total rolls: {}, min score: {}", gamestate.total_rolls, min_score);
        let final_tally = gamestate.total_rolls * min_score;
        println!("Player 1: {}", gamestate.score_p1);
        println!("Player 2: {}", gamestate.score_p2);
        println!("Final: {}", final_tally);

        return match gamestate.score_p1 > gamestate.score_p2 {
            true => Return::new((1, 0)),
            false => match gamestate.score_p2 > gamestate.score_p1 {
                true => Return::new((0, 1)),
                false => panic!("A tie? Shouldn't happen.")
            }
        }
    }

    // println!("p1: {}, p2: {}", gamestate.score_p1, gamestate.score_p2);

    let mut next = gamestate.clone();
    match gamestate.turn {
        Turn::P1 => {
            next.pos_p1 = (gamestate.pos_p1 + gamestate.die_val - 1) % 10 + 1;
            next.score_p1 += next.pos_p1;
            next.turn = Turn::P2;
            // println!("P1 new pos: {}, new score: {}", next.pos_p1, next.score_p1);
        },
        Turn:: P2 => {
            next.pos_p2 = (gamestate.pos_p2 + gamestate.die_val - 1) % 10 + 1;
            next.score_p2 += next.pos_p2;
            next.turn = Turn::P1;
            // println!("P2 new pos: {}, new score: {}", next.pos_p2, next.score_p2);
        },
        Turn::First => {
            next.turn = Turn::P1;
        }
    }

    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    if next.score_p1 < 1000 && next.score_p2 < 1000 {
        next.die_val = 0;
        for i in 0..3 {
            let die_roll = (gamestate.total_rolls + i) % 100 + 1;
            next.die_val += die_roll;
        }
        next.total_rolls += 3;
    }

    let res = find_wins_deterministic(next);

    let (next_p1_wins, next_p2_wins) = *res;

    p1_wins += next_p1_wins;
    p2_wins += next_p2_wins;

    return Return::new((p1_wins, p2_wins));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Gamestate {
    pos_p1: u32,
    pos_p2: u32,
    turn: Turn,
    score_p1: u32,
    score_p2: u32,
    die_val: u32,
    total_rolls: u32
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Turn {
    P1,
    P2,
    First
}

fn game_with_dirac_die() {
    let gamestate = Gamestate {
        score_p1: 0,
        score_p2: 0,
        // pos_p1: 4, // Sample
        // pos_p2: 8,
        pos_p1: 5, // Input
        pos_p2: 9,
        turn: Turn::First,
        die_val: 0,
        total_rolls: 0
    };

    let (p1_wins, p2_wins) = find_wins_dirac(gamestate);

    println!("Player 1 won {}, Player 2 won {} -- (biggest is {} )", p1_wins, p2_wins, cmp::max(p1_wins, p2_wins));
    println!("Should be    {},              {} for the sample", 444356092776315u64, 341960390180808u64);
}

#[cached]
fn find_wins_dirac(gamestate: Gamestate) -> (u64, u64) {
    if gamestate.score_p1 >= 21 || gamestate.score_p2 >= 21 {
        return match gamestate.score_p1 > gamestate.score_p2 {
            true => (1, 0),
            false => match gamestate.score_p2 > gamestate.score_p1 {
                true => (0, 1),
                false => panic!("A tie? Shouldn't happen.")
            }
        }
    }

    let mut next = gamestate.clone();
    match gamestate.turn {
        Turn::P1 => {
            next.pos_p1 = (gamestate.pos_p1 + gamestate.die_val - 1) % 10 + 1;
            next.score_p1 += next.pos_p1;
            next.turn = Turn::P2;
            // println!("P1 new pos: {}, new score: {}", next.pos_p1, next.score_p1);
        },
        Turn:: P2 => {
            next.pos_p2 = (gamestate.pos_p2 + gamestate.die_val - 1) % 10 + 1;
            next.score_p2 += next.pos_p2;
            next.turn = Turn::P1;
            // println!("P2 new pos: {}, new score: {}", next.pos_p2, next.score_p2);
        },
        Turn::First => {
            next.turn = Turn::P1;
        }
    }

    let mut p1_wins: u64 = 0;
    let mut p2_wins: u64 = 0;

    if next.score_p1 >= 21 || next.score_p2 >= 21 {
        return match next.score_p1 > next.score_p2 {
            true => (1, 0),
            false => match next.score_p2 > next.score_p1 {
                true => (0, 1),
                false => panic!("A tie? Shouldn't happen.")
            }
        }
    }

    // 27 universes fork here, one for each combination of 3-sided dice.
    for roll1 in 1..=3 {
        for roll2 in 1..=3 {
            for roll3 in 1..=3 {
                let mut next_with_roll = next.clone();
                next_with_roll.die_val = roll1 + roll2 + roll3;

                let res = find_wins_dirac(next_with_roll);
                let (next_p1_wins, next_p2_wins) = res;

                p1_wins += next_p1_wins;
                p2_wins += next_p2_wins;
                // println!("p1 wins: {}; p2 wins: {}", p1_wins, p2_wins);
            }
        }
    }

    return (p1_wins, p2_wins);
}