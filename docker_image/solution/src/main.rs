use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead, Write};
use std::process;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();
    let mut turn_counter = 0;

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
            if let Err(_) = process_turn(
                &mut input,
                &mut buffer,
                player_symbol,
                territory_symbol,
                turn_counter,
            ) {
                respond_invalid();
                process::exit(1);
            }
            turn_counter += 1;
        }
    }
}

fn initialize_player(input: &mut impl BufRead, buffer: &mut String) -> Option<(char, char)> {
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

fn process_turn(
    input: &mut impl BufRead,
    buffer: &mut String,
    player_symbol: char,
    territory_symbol: char,
    turn_counter: usize,
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

    if let Some((y, x)) = find_optimal_placement(
        &board,
        &piece_cells,
        player_symbol,
        territory_symbol,
        turn_counter,
    ) {
        println!("{} {}", x, y);
    } else {
        println!("0 0");
    }
    io::stdout().flush()?;

    Ok(())
}

fn find_optimal_placement(
    board: &[Vec<char>],
    piece_cells: &[(usize, usize)],
    player_symbol: char,
    territory_symbol: char,
    turn_counter: usize,
) -> Option<(usize, usize)> {
    let our_territory = find_territory(board, player_symbol, territory_symbol);
    let opponent_territory = find_opponent_territory(board, player_symbol, territory_symbol);
    let opponent_frontier = find_frontier_cells(board, &opponent_territory, player_symbol);
    let our_frontier = find_frontier_cells(board, &our_territory, player_symbol);
    let opponent_center = compute_center(&opponent_territory);
    let our_center = compute_center(&our_territory);
    
    let opponent_expansion = predict_opponent_expansion(board, &opponent_territory, player_symbol);

    let map_height = board.len();
    let map_width = board[0].len();

    let mut best_score = i32::MIN;
    let mut best_pos = None;

    let direction_y = (opponent_center.0 as i32 - our_center.0 as i32).signum();
    let direction_x = (opponent_center.1 as i32 - our_center.1 as i32).signum();

    for &(ey, ex) in &opponent_expansion {
        for dy in -3..=3 {
            for dx in -3..=3 {
                let y = (ey as i32 + dy) as usize;
                let x = (ex as i32 + dx) as usize;

                if y >= map_height || x >= map_width {
                    continue;
                }

                if is_valid_placement(board, piece_cells, y, x, player_symbol, territory_symbol) {
                    let score = evaluate_placement(
                        board,
                        piece_cells,
                        y,
                        x,
                        &our_territory,
                        &opponent_territory,
                        player_symbol,
                        territory_symbol,
                        map_height,
                        map_width,
                        true,
                        turn_counter,
                        opponent_center,
                    ) + 500; 

                    if score > best_score {
                        best_score = score;
                        best_pos = Some((y, x));
                    }
                }
            }
        }
    }

    for &(ty, tx) in &opponent_frontier {
        for dy in -7..=7 {
            for dx in -7..=7 {
                let y = (ty as i32 + dy) as usize;
                let x = (tx as i32 + dx) as usize;

                if y >= map_height || x >= map_width {
                    continue;
                }

                if is_valid_placement(board, piece_cells, y, x, player_symbol, territory_symbol) {
                   let directional_bonus = if dy.signum() == direction_y && dx.signum() == direction_x {
                        300
                    } else {
                        0
                    };

                    let score = evaluate_placement(
                        board,
                        piece_cells,
                        y,
                        x,
                        &our_territory,
                        &opponent_territory,
                        player_symbol,
                        territory_symbol,
                        map_height,
                        map_width,
                        true,
                        turn_counter,
                        opponent_center,
                    ) + directional_bonus;

                    if score > best_score {
                        best_score = score;
                        best_pos = Some((y, x));
                    }
                }
            }
        }
    }

    if best_score < 1000 {
        for &(ty, tx) in &our_frontier {
            for dy in -5..=5 {
                for dx in -5..=5 {
                    let y = (ty as i32 + dy) as usize;
                    let x = (tx as i32 + dx) as usize;

                    if y >= map_height || x >= map_width {
                        continue;
                    }

                    if is_valid_placement(board, piece_cells, y, x, player_symbol, territory_symbol) {
                        let score = evaluate_placement(
                            board,
                            piece_cells,
                            y,
                            x,
                            &our_territory,
                            &opponent_territory,
                            player_symbol,
                            territory_symbol,
                            map_height,
                            map_width,
                            false,
                            turn_counter,
                            opponent_center,
                        );

                        if score > best_score {
                            best_score = score;
                            best_pos = Some((y, x));
                        }
                    }
                }
            }
        }
    }

    if best_pos.is_none() {
        for y in 0..map_height {
            for x in 0..map_width {
                if is_valid_placement(board, piece_cells, y, x, player_symbol, territory_symbol) {
                    let score = evaluate_placement(
                        board,
                        piece_cells,
                        y,
                        x,
                        &our_territory,
                        &opponent_territory,
                        player_symbol,
                        territory_symbol,
                        map_height,
                        map_width,
                        false,
                        turn_counter,
                        opponent_center,
                    );

                    if score > best_score {
                        best_score = score;
                        best_pos = Some((y, x));
                    }
                }
            }
        }
    }

    best_pos
}

fn evaluate_placement(
    board: &[Vec<char>],
    piece_cells: &[(usize, usize)],
    y: usize,
    x: usize,
    our_territory: &[(usize, usize)],
    opponent_territory: &[(usize, usize)],
    player_symbol: char,
    territory_symbol: char,
    map_height: usize,
    map_width: usize,
    aggressive_mode: bool,
    turn_counter: usize,
    opponent_center: (usize, usize),
) -> i32 {
    let mut score = 0;
    let mut new_territory = 0;
    let mut blocks_opponent = 0;
    let mut connects_territory = false;
    let mut adjacent_to_opponent = false;
    let mut cuts_opponent_path = false;
    
    for &(py, px) in piece_cells {
        let cy = y + py;
        let cx = x + px;

        if cy >= map_height || cx >= map_width {
            return i32::MIN;
        }

        if board[cy][cx] == '.' {
            new_territory += 1;

            let mut opponent_adjacent_count = 0;
            for (dy, dx) in &DIRECTIONS {
                let ny = cy as i32 + dy;
                let nx = cx as i32 + dx;
                if ny >= 0 && ny < map_height as i32 && nx >= 0 && nx < map_width as i32 {
                    let ny = ny as usize;
                    let nx = nx as usize;
                    if is_opponent_cell(board[ny][nx], player_symbol) {
                        opponent_adjacent_count += 1;
                        blocks_opponent += 1;
                        adjacent_to_opponent = true;
                    }
                }
            }
            
            if opponent_adjacent_count >= 2 {
                cuts_opponent_path = true;
            }
        }
    }

    if connects_territory_segments(board, y, x, piece_cells, our_territory) {
        connects_territory = true;
    }

    score += new_territory * 100;
    
    score += blocks_opponent * 250;
    if cuts_opponent_path {
        score += 500;
    }
    if adjacent_to_opponent {
        score += 200;
    }
    
    if connects_territory {
        score += 300;
    }

    if creates_isolated_territory(board, y, x, piece_cells, our_territory) {
        score -= 500;
    }

    if turn_counter < 10 {
        let dist_to_opponent = distance(
            (y + piece_cells[0].0, x + piece_cells[0].1),
            opponent_center,
        );
        score += (15 - dist_to_opponent as i32) * 80;
    } else if turn_counter < 30 {
        if adjacent_to_opponent {
            score += 400;
        }
    } else {
        score += new_territory * 150;
    }
    
    if aggressive_mode && adjacent_to_opponent {
        score += 300;
    }

    score
}

fn is_opponent_cell(cell: char, our_symbol: char) -> bool {
    if our_symbol == '@' {
        cell == '$' || cell == 's'
    } else {
        cell == '@' || cell == 'a'
    }
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
}

fn find_territory(
    board: &[Vec<char>],
    player_sym: char,
    territory_sym: char,
) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for (y, row) in board.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == player_sym || cell == territory_sym {
                positions.push((y, x));
            }
        }
    }
    positions
}

fn find_opponent_territory(
    board: &[Vec<char>],
    player_symbol: char,
    _territory_symbol: char,
) -> Vec<(usize, usize)> {
    let (opponent_symbol, opponent_territory) = if player_symbol == '@' {
        ('$', 's')
    } else {
        ('@', 'a')
    };
    find_territory(board, opponent_symbol, opponent_territory)
}

fn is_valid_placement(
    board: &[Vec<char>],
    piece_cells: &[(usize, usize)],
    y: usize,
    x: usize,
    player_sym: char,
    territory_sym: char,
) -> bool {
    let mut has_overlap = false;

    for &(py, px) in piece_cells {
        let cy = y + py;
        let cx = x + px;

        if cy >= board.len() || cx >= board[0].len() {
            return false;
        }

        let cell = board[cy][cx];
        if cell == player_sym || cell == territory_sym {
            if has_overlap {
                return false;
            }
            has_overlap = true;
        } else if is_opponent_cell(cell, player_sym) {
            return false;
        }
    }

    has_overlap
}

fn compute_center(positions: &[(usize, usize)]) -> (usize, usize) {
    let sum_y: usize = positions.iter().map(|&(y, _)| y).sum();
    let sum_x: usize = positions.iter().map(|&(_, x)| x).sum();
    let count = positions.len().max(1);
    (sum_y / count, sum_x / count)
}

fn find_frontier_cells(
    board: &[Vec<char>],
    territory: &[(usize, usize)],
    player_symbol: char,
) -> Vec<(usize, usize)> {
    let mut frontier = Vec::new();
    let mut frontier_set = HashSet::new();
    let opponent_symbol = if player_symbol == '@' { '$' } else { '@' };

    for &(y, x) in territory {
        for (dy, dx) in &DIRECTIONS {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            if ny >= 0 && ny < board.len() as i32 && nx >= 0 && nx < board[0].len() as i32 {
                let ny = ny as usize;
                let nx = nx as usize;
                if board[ny][nx] == '.' {
                    if frontier_set.insert((y, x)) {
                        frontier.push((y, x));
                    }
                    break;
                }
            }
        }
    }
    
    frontier.sort_by_key(|&(y, x)| {
        let mut opponent_adjacent = 0;
        let mut empty_adjacent = 0;
        
        for (dy, dx) in &DIRECTIONS {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            if ny >= 0 && ny < board.len() as i32 && nx >= 0 && nx < board[0].len() as i32 {
                let ny = ny as usize;
                let nx = nx as usize;
                if is_opponent_cell(board[ny][nx], player_symbol) {
                    opponent_adjacent += 1;
                } else if board[ny][nx] == '.' {
                    empty_adjacent += 1;
                }
            }
        }
        
        -(opponent_adjacent * 10 + empty_adjacent) as i32
    });
    
    frontier
}

fn connects_territory_segments(
    board: &[Vec<char>],
    y: usize,
    x: usize,
    piece_cells: &[(usize, usize)],
    our_territory: &[(usize, usize)],
) -> bool {
    if our_territory.len() < 5 {
        return false;
    }

    let mut connected_segments = 0;

    for &(py, px) in piece_cells {
        let cy = y + py;
        let cx = x + px;

        if cy >= board.len() || cx >= board[0].len() {
            continue;
        }

        let mut segment_connections = 0;
        for &(ty, tx) in our_territory {
            if distance((cy, cx), (ty, tx)) <= 3 {
                segment_connections += 1;
            }
        }

        if segment_connections >= 2 {
            connected_segments += 1;
        }
    }

    connected_segments >= 2
}

fn creates_isolated_territory(
    board: &[Vec<char>],
    y: usize,
    x: usize,
    piece_cells: &[(usize, usize)],
    our_territory: &[(usize, usize)],
) -> bool {
    let mut min_distance = usize::MAX;

    for &(py, px) in piece_cells {
        let cy = y + py;
        let cx = x + px;

        if cy >= board.len() || cx >= board[0].len() {
            continue;
        }

        for &(ty, tx) in our_territory {
            let dist = distance((cy, cx), (ty, tx));
            min_distance = min_distance.min(dist);
        }
    }

    min_distance > 5
}

fn respond_invalid() {
    println!("0 0");
    io::stdout().flush().unwrap();
}


fn predict_opponent_expansion(
    board: &[Vec<char>],
    opponent_territory: &[(usize, usize)],
    player_symbol: char,
) -> Vec<(usize, usize)> {
    let mut expansion_points = Vec::new();
    let mut visited = HashSet::new();
    let opponent_symbol = if player_symbol == '@' { '$' } else { '@' };
    let opponent_territory_symbol = if player_symbol == '@' { 's' } else { 'a' };
    
    let mut queue = VecDeque::new();
    for &pos in opponent_territory {
        queue.push_back(pos);
        visited.insert(pos);
    }
    
    while let Some((y, x)) = queue.pop_front() {
        for (dy, dx) in &DIRECTIONS {
            let ny = y as i32 + dy;
            let nx = x as i32 + dx;
            
            if ny < 0 || ny >= board.len() as i32 || nx < 0 || nx >= board[0].len() as i32 {
                continue;
            }
            
            let ny = ny as usize;
            let nx = nx as usize;
            let pos = (ny, nx);
            
            if !visited.contains(&pos) && board[ny][nx] == '.' {
                visited.insert(pos);
                
                let mut empty_neighbors = 0;
                for (ndy, ndx) in &DIRECTIONS {
                    let nny = ny as i32 + ndy;
                    let nnx = nx as i32 + ndx;
                    
                    if nny >= 0 && nny < board.len() as i32 && nnx >= 0 && nnx < board[0].len() as i32 {
                        let nny = nny as usize;
                        let nnx = nnx as usize;
                        if board[nny][nnx] == '.' {
                            empty_neighbors += 1;
                        }
                    }
                }
                
                if empty_neighbors >= 2 {
                    expansion_points.push(pos);
                }
                
                if visited.len() < 100 {
                    queue.push_back(pos);
                }
            }
        }
    }
    
    expansion_points
}