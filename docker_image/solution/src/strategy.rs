use std::collections::{HashSet, VecDeque};
use crate::board::{DIRECTIONS, DIAGONALS, get_territory_metrics, predict_opponent_direction, collect_territory_cells, collect_opponent_territory, calculate_territory_center};
use crate::utils::grid_distance;

pub fn find_best_move(
    board: &[Vec<char>],
    piece_cells: &[(usize, usize)],
    player_symbol: char,
    territory_symbol: char,
    turn_counter: usize,
    opponent_pattern: &VecDeque<(usize, usize)>,
) -> Option<(usize, usize)> {
    let our_territory = collect_territory_cells(board, player_symbol, territory_symbol);
    let our_territory_set: HashSet<_> = our_territory.iter().cloned().collect();
    let opponent_territory = collect_opponent_territory(board, player_symbol, territory_symbol);
    let opponent_territory_set: HashSet<_> = opponent_territory.iter().cloned().collect();

    let (our_size, our_enclosed, our_frontier) =
        get_territory_metrics(board, player_symbol, territory_symbol);
    let (opponent_size, _, opponent_frontier) = get_territory_metrics(
        board,
        if player_symbol == '@' { '$' } else { '@' },
        if player_symbol == '@' { 's' } else { 'a' },
    );

    let opponent_center = calculate_territory_center(&opponent_territory);
    let our_center = calculate_territory_center(&our_territory);

    let total_cells = board.len() * board[0].len();
    let phase = ((our_size + opponent_size) * 100) / total_cells;

    let opponent_direction = predict_opponent_direction(opponent_pattern);

    let mut best_score = i32::MIN;
    let mut best_pos = None;

    let board_size = board.len() * board[0].len();
    let search_radius = match (phase, board_size) {
        (0..=30, size) if size > 5000 => 3,
        (0..=30, _) => 5,
        (31..=70, size) if size > 5000 => 2,
        (31..=70, _) => 4,
        (_, size) if size > 5000 => 2,
        _ => 3,
    };

    if board_size > 5000 {
        let mut candidates = HashSet::new();

        let opp_sample_rate = if opponent_frontier.len() > 100 { 3 } else { 1 };
        for (i, &(y, x)) in opponent_frontier.iter().enumerate() {
            if i % opp_sample_rate != 0 {
                continue;
            }

            for dy in -search_radius..=search_radius {
                for dx in -search_radius..=search_radius {
                    let ny = (y as i32 + dy) as usize;
                    let nx = (x as i32 + dx) as usize;
                    if ny < board.len() && nx < board[0].len() {
                        candidates.insert((ny, nx));
                    }
                }
            }
        }

        let our_sample_rate = if our_frontier.len() > 100 { 3 } else { 1 };
        for (i, &(y, x)) in our_frontier.iter().enumerate() {
            if i % our_sample_rate != 0 {
                continue;
            }

            for dy in -search_radius..=search_radius {
                for dx in -search_radius..=search_radius {
                    let ny = (y as i32 + dy) as usize;
                    let nx = (x as i32 + dx) as usize;
                    if ny < board.len() && nx < board[0].len() {
                        candidates.insert((ny, nx));
                    }
                }
            }
        }

        for &(ny, nx) in &candidates {
            if is_valid_placement(
                board,
                piece_cells,
                ny,
                nx,
                player_symbol,
                territory_symbol,
                &our_territory_set,
            ) {
                let score = evaluate_move_score(
                    board,
                    piece_cells,
                    ny,
                    nx,
                    player_symbol,
                    &opponent_territory_set,
                    &our_territory_set,
                    phase,
                    opponent_center,
                    our_center,
                    opponent_direction,
                );

                if score > best_score {
                    best_score = score;
                    best_pos = Some((ny, nx));
                }
            }
        }
    } else {
        for &(y, x) in &opponent_frontier {
            for dy in -search_radius..=search_radius {
                for dx in -search_radius..=search_radius {
                    let ny = (y as i32 + dy) as usize;
                    let nx = (x as i32 + dx) as usize;

                    if ny >= board.len() || nx >= board[0].len() {
                        continue;
                    }

                    if is_valid_placement(
                        board,
                        piece_cells,
                        ny,
                        nx,
                        player_symbol,
                        territory_symbol,
                        &our_territory_set,
                    ) {
                        let score = evaluate_move_score(
                            board,
                            piece_cells,
                            ny,
                            nx,
                            player_symbol,
                            &opponent_territory_set,
                            &our_territory_set,
                            phase,
                            opponent_center,
                            our_center,
                            opponent_direction,
                        );

                        if score > best_score {
                            best_score = score;
                            best_pos = Some((ny, nx));
                        }
                    }
                }
            }
        }

        if best_score < 1000 || phase < 50 {
            for &(y, x) in &our_frontier {
                for dy in -search_radius..=search_radius {
                    for dx in -search_radius..=search_radius {
                        let ny = (y as i32 + dy) as usize;
                        let nx = (x as i32 + dx) as usize;

                        if ny >= board.len() || nx >= board[0].len() {
                            continue;
                        }

                        if is_valid_placement(
                            board,
                            piece_cells,
                            ny,
                            nx,
                            player_symbol,
                            territory_symbol,
                            &our_territory_set,
                        ) {
                            let score = evaluate_move_score(
                                board,
                                piece_cells,
                                ny,
                                nx,
                                player_symbol,
                                &opponent_territory_set,
                                &our_territory_set,
                                phase,
                                opponent_center,
                                our_center,
                                opponent_direction,
                            );

                            if score > best_score {
                                best_score = score;
                                best_pos = Some((ny, nx));
                            }
                        }
                    }
                }
            }
        }
    }

    if best_pos.is_none() {
        let step = if board_size > 5000 { 2 } else { 1 };
        for y in (0..board.len()).step_by(step) {
            for x in (0..board[0].len()).step_by(step) {
                if is_valid_placement(
                    board,
                    piece_cells,
                    y,
                    x,
                    player_symbol,
                    territory_symbol,
                    &our_territory_set,
                ) {
                    return Some((y, x));
                }
            }
        }
    }

    best_pos
}

pub fn evaluate_move_score(
    board: &[Vec<char>],
    piece_cells: &[(usize, usize)],
    y: usize,
    x: usize,
    player_symbol: char,
    opponent_territory: &HashSet<(usize, usize)>,
    our_territory: &HashSet<(usize, usize)>,
    phase: usize,
    opponent_center: (usize, usize),
    our_center: (usize, usize),
    opponent_direction: (i32, i32),
) -> i32 {
    let mut score = 0;
    let mut new_territory = 0;
    let mut blocks_opponent = 0;
    let mut creates_enclosure = 0;
    let mut potential_growth = 0;
    let mut direction_bonus = 0;

    let opponent_symbol = if player_symbol == '@' { '$' } else { '@' };

    for &(py, px) in piece_cells {
        let cy = y + py;
        let cx = x + px;

        if board[cy][cx] == '.' {
            new_territory += 1;

            for &(dy, dx) in &DIRECTIONS {
                let ny = (cy as i32 + dy) as usize;
                let nx = (cx as i32 + dx) as usize;
                if ny < board.len() && nx < board[0].len() {
                    if opponent_territory.contains(&(ny, nx)) {
                        blocks_opponent += 1;
                    }

                    if board[ny][nx] == '.' {
                        let mut surrounded = true;
                        for &(ddy, ddx) in &DIAGONALS {
                            let nny = (ny as i32 + ddy) as usize;
                            let nnx = (nx as i32 + ddx) as usize;
                            if nny < board.len() && nnx < board[0].len() {
                                if !our_territory.contains(&(nny, nnx))
                                    && board[nny][nnx] != opponent_symbol
                                {
                                    surrounded = false;
                                    break;
                                }
                            }
                        }
                        if surrounded {
                            creates_enclosure += 1;
                        }
                    }
                }
            }

            for &(dy, dx) in &DIRECTIONS {
                let ny = (cy as i32 + dy) as usize;
                let nx = (cx as i32 + dx) as usize;
                if ny < board.len() && nx < board[0].len() && board[ny][nx] == '.' {
                    potential_growth += 1;
                }
            }

            let move_dy = (cy as i32 - our_center.0 as i32).signum();
            let move_dx = (cx as i32 - our_center.1 as i32).signum();

            if phase < 50 {
                if (move_dy == opponent_direction.0 && move_dx == opponent_direction.1)
                    || (move_dy == -opponent_direction.0 && move_dx == -opponent_direction.1)
                {
                    direction_bonus += 2;
                }
            } else {
                if board[cy][cx] == '.' {
                    direction_bonus += 1;
                }
            }
        }
    }

    let territory_weight = if phase < 30 {
        80
    } else if phase < 70 {
        120
    } else {
        150
    };
    let blocking_weight = if phase < 30 {
        200
    } else if phase < 70 {
        150
    } else {
        80
    };
    let enclosure_weight = if phase < 30 {
        50
    } else if phase < 70 {
        100
    } else {
        200
    };

    score += new_territory as i32 * territory_weight;
    score += blocks_opponent as i32 * blocking_weight;
    score += creates_enclosure as i32 * enclosure_weight;
    score += potential_growth as i32 * 30;
    score += direction_bonus * 50;

    let dist_to_opponent = grid_distance((y, x), opponent_center);
    let dist_to_us = grid_distance((y, x), our_center);
    score += (100 - dist_to_opponent as i32) * 5;
    score += (50 - dist_to_us as i32) * 2;

    score
}

pub fn is_valid_placement(
    board: &[Vec<char>],
    piece_cells: &[(usize, usize)],
    y: usize,
    x: usize,
    player_sym: char,
    territory_sym: char,
    our_territory: &HashSet<(usize, usize)>,
) -> bool {
    let mut has_overlap = false;
    let opponent_sym = if player_sym == '@' { '$' } else { '@' };
    let opponent_territory_sym = if player_sym == '@' { 's' } else { 'a' };

    for &(py, px) in piece_cells {
        let cy = y + py;
        let cx = x + px;

        if cy >= board.len() || cx >= board[0].len() {
            return false;
        }

        let cell = board[cy][cx];

        if cell == opponent_sym || cell == opponent_territory_sym {
            return false;
        }

        if cell == player_sym || cell == territory_sym {
            if has_overlap {
                return false;
            }
            has_overlap = true;
        }
    }

    has_overlap
}