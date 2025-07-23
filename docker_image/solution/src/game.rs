use std::collections::VecDeque;
use std::io::{self, BufRead, Write};

pub fn initialize_player(input: &mut impl BufRead, buffer: &mut String) -> Option<(char, char)> {
    for _ in 0..2 {
        buffer.clear();
        if input.read_line(buffer).is_err() {
            return None;
        }

        if buffer.contains("p1") {
            return Some(('@', 'a'));
        } else if buffer.contains("p2") {
            return Some(('$', 's'));
        }
    }
    None
}

pub fn process_game_turn(
    input: &mut impl BufRead,
    buffer: &mut String,
    player_symbol: char,
    territory_symbol: char,
    turn_counter: usize,
    opponent_pattern: &mut VecDeque<(usize, usize)>,
) -> io::Result<()> {
    let parts: Vec<&str> = buffer.split_whitespace().collect();
    let rows = parts[2].trim_end_matches(':').parse::<usize>().unwrap_or(0);

    buffer.clear();
    input.read_line(buffer)?;

    let mut board = Vec::with_capacity(rows);
    for _ in 0..rows {
        buffer.clear();
        input.read_line(buffer)?;
        let row = buffer[4..].trim_end().chars().collect();
        board.push(row);
    }

    buffer.clear();
    input.read_line(buffer)?;
    if !buffer.starts_with("Piece") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Expected Piece header",
        ));
    }

    let piece_rows = buffer
        .split_whitespace()
        .nth(2)
        .and_then(|s| s.trim_end_matches(':').parse().ok())
        .unwrap_or(0);

    let mut piece_cells = Vec::new();
    for row in 0..piece_rows {
        buffer.clear();
        input.read_line(buffer)?;
        let line = buffer.trim_end();
        for (col, c) in line.chars().enumerate() {
            if c == 'O' {
                piece_cells.push((row, col));
            }
        }
    }

    crate::board::update_opponent_movement_pattern(&board, player_symbol, territory_symbol, opponent_pattern);

    if let Some((y, x)) = crate::strategy::find_best_move(
        &board,
        &piece_cells,
        player_symbol,
        territory_symbol,
        turn_counter,
        opponent_pattern,
    ) {
        println!("{} {}", x, y);
    } else {
        println!("0 0");
    }
    io::stdout().flush()?;

    Ok(())
}

pub fn send_default_move() {
    println!("0 0");
    io::stdout().flush().unwrap();
}