use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum RPSMove {
    Rock,
    Paper,
    Scissors,
}

enum RPSOutcome {
    Win,
    Loss,
    Draw,
}

struct RPSGameRequirement {
    opponent: RPSMove,
    outcome: RPSOutcome,
}

struct RPSGame {
    opponent: RPSMove,
    you: RPSMove,
}

fn get_required_game_from_input(opponent: &str, outcome: &str) -> RPSGameRequirement {

    let opponent_mv = match opponent {
        "A" => RPSMove::Rock,
        "B" => RPSMove::Paper,
        "C" => RPSMove::Scissors,
        &_ => todo!(),
    };

    let req_outcome = match outcome {
        "X" => RPSOutcome::Loss,
        "Y" => RPSOutcome::Draw,
        "Z" => RPSOutcome::Win,
        &_ => todo!(),
    };  

    RPSGameRequirement{
        opponent: opponent_mv,
        outcome: req_outcome,
    }
}

fn get_score_from_game(game: &RPSGame) -> u8 {
    let mut score = 0;

    score += match game.you {
        RPSMove::Rock => 1,
        RPSMove::Paper => 2,
        RPSMove::Scissors => 3,
    };

    score += match get_outcome_from_game(game) {
        RPSOutcome::Win => 6,
        RPSOutcome::Loss => 0,
        RPSOutcome::Draw => 3,
    };

    score
}

fn make_game(req_game: RPSGameRequirement) -> RPSGame {
    let you_mv = match req_game.outcome {
        RPSOutcome::Draw => match req_game.opponent {
            RPSMove::Rock => RPSMove::Rock,
            RPSMove::Paper => RPSMove::Paper,
            RPSMove::Scissors => RPSMove::Scissors,
        },
        RPSOutcome::Win => match req_game.opponent {
            RPSMove::Rock => RPSMove::Paper,
            RPSMove::Paper => RPSMove::Scissors,
            RPSMove::Scissors => RPSMove::Rock,
        },
        RPSOutcome::Loss => match req_game.opponent {
            RPSMove::Rock => RPSMove::Scissors,
            RPSMove::Paper => RPSMove::Rock,
            RPSMove::Scissors => RPSMove::Paper,
        },
    };

    RPSGame {
        opponent: req_game.opponent,
        you: you_mv,
    }
}

fn get_outcome_from_game(game: &RPSGame) -> RPSOutcome {
    match game.you {
        RPSMove::Rock => match game.opponent {
            RPSMove::Rock => RPSOutcome::Draw,
            RPSMove::Paper => RPSOutcome::Loss,
            RPSMove::Scissors => RPSOutcome::Win,
        },
        RPSMove::Paper => match game.opponent {
            RPSMove::Rock => RPSOutcome::Win,
            RPSMove::Paper => RPSOutcome::Draw,
            RPSMove::Scissors => RPSOutcome::Loss,
        },
        RPSMove::Scissors => match game.opponent {
            RPSMove::Rock => RPSOutcome::Loss,
            RPSMove::Paper => RPSOutcome::Win,
            RPSMove::Scissors => RPSOutcome::Draw,
        },
    }
}


fn main() {
    let buffered = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(error) => panic!("File open error: {:?}", error),
    };

    let mut total_score: i32 = 0;

    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        let v = line.split(" ").collect::<Vec<_>>();
        let req_game = get_required_game_from_input(v[0], v[1]);
        let game = make_game(req_game);
        total_score += i32::from(get_score_from_game(&game));
    }

    println!("Total score: {}", total_score);
}
