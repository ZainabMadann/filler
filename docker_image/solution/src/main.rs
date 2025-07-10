use std::io::{self, BufRead, Write};
use std::process;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();
    
    
    let (player_symbol, territory_symbol) = match initialize_player(&mut input, &mut buffer) {
        Some((p, t)) => (p, t),
        None => {
            respond_invalid();
            process::exit(1);
        }
    };

    loop {
        buffer.clear();
        if input.read_line(&mut buffer).is_err() {
            break;
        }

        if buffer.starts_with("Anfield") {
            if let Err(_) = process_turn(&mut input, &mut buffer, player_symbol, territory_symbol) {
                respond_invalid();
                process::exit(1);
            }
        }
    }
}

fn initialize_player(input: &mut impl BufRead, buffer: &mut String) -> Option<(char, char)> {
    for _ in 0..2 {
        buffer.clear();
        if input.read_line(buffer).is_err() {
            return None;
        }

        if buffer.contains("p1") || buffer.contains("[./filler_bot]") {
            return Some(('@', 'a'));
        } else if buffer.contains("p2") {
            return Some(('$', 's'));
        } else {
            return None;
        }
    }
    None
}


fn process_turn(
    input: &mut impl BufRead,
    buffer: &mut String,
    player_symbol: char,
    territory_symbol: char,
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
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Expected Piece header"));
    }

    let piece_rows = buffer.split_whitespace()
        .nth(2)
        .and_then(|s| s.trim_end_matches(':').parse().ok())
        .unwrap_or(0);

    let mut piece_cells = Vec::new();
    for row in 0..piece_rows {
        buffer.clear();
        input.read_line(buffer)?;
        for (col, c) in buffer.trim_end().chars().enumerate() {
            if c == 'O' {
                piece_cells.push((row, col));
            }
        }
    }

    if let Some((y, x)) = find_placement(&board, &piece_cells, player_symbol, territory_symbol) {
        println!("{} {}", x, y);
    } else {
        println!("0 0");
    }
    io::stdout().flush()?;

    Ok(())
}

fn find_placement(
    board: &[Vec<char>],
    piece_cells: &[(usize, usize)],
    player_sym: char,
    territory_sym: char,
) -> Option<(usize, usize)> {
    let center_y = board.len() / 2;
    let center_x = board[0].len() / 2;
    
    for radius in 0..=center_y.max(center_x) {
        for y in center_y.saturating_sub(radius)..=center_y.saturating_add(radius).min(board.len() - 1) {
            for x in center_x.saturating_sub(radius)..=center_x.saturating_add(radius).min(board[0].len() - 1) {
                if is_valid_placement(board, piece_cells, y, x, player_sym, territory_sym) {
                    return Some((y, x));
                }
            }
        }
    }
    None
}

fn is_valid_placement(
    board: &[Vec<char>],
    piece_cells: &[(usize, usize)],
    y: usize,
    x: usize,
    player_sym: char,
    territory_sym: char,
) -> bool {
    let mut overlaps = 0;
    
    for &(py, px) in piece_cells {
        let cy = y + py;
        let cx = x + px;
        
        if cy >= board.len() || cx >= board[0].len() {
            return false;
        }
        
        let cell = board[cy][cx];
        
        match (player_sym, cell) {
            ('@', '$') | ('@', 's') | ('$', '@') | ('$', 'a') => return false,
            _ => ()
        }
        
        if cell == player_sym || cell == territory_sym {
            overlaps += 1;
            if overlaps > 1 {
                return false;
            }
        }
    }
    
    overlaps == 1
}

fn respond_invalid() {
    println!("0 0");
    io::stdout().flush().unwrap();
}