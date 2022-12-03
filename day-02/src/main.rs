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

struct RPSGame {
    opponent: RPSMove,
    you: RPSMove,
}

fn get_game_from_input(opponent: &str, you: &str) -> RPSGame {

    let opponent_mv = match opponent {
        "A" => RPSMove::Rock,
        "B" => RPSMove::Paper,
        "C" => RPSMove::Scissors,
        &_ => todo!(),
    };

    let you_mv = match you {
        "X" => RPSMove::Rock,
        "Y" => RPSMove::Paper,
        "Z" => RPSMove::Scissors,
        &_ => todo!(),
    };  

    RPSGame{
        opponent: opponent_mv,
        you: you_mv,
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
        let game = get_game_from_input(v[0], v[1]);
        total_score += i32::from(get_score_from_game(&game));
    }

    println!("Total score: {}", total_score);
}
