mod board;
mod game;
mod strategy;
mod utils;

use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    let (player_symbol, territory_symbol) = match game::initialize_player(&mut input, &mut buffer) {
        Some((p, t)) => (p, t),
        None => {
            game::send_default_move();
            process::exit(1);
        }
    };

    let mut turn_counter = 0;
    let mut opponent_pattern = VecDeque::new();

    loop {
        buffer.clear();
        if input.read_line(&mut buffer).is_err() {
            break;
        }

        if buffer.starts_with("Anfield") {
            if let Err(_) = game::process_game_turn(
                &mut input,
                &mut buffer,
                player_symbol,
                territory_symbol,
                turn_counter,
                &mut opponent_pattern,
            ) {
                game::send_default_move();
                process::exit(1);
            }
            turn_counter += 1;
        }
    }
}