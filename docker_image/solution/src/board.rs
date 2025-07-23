use std::collections::{HashSet, VecDeque};

pub const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
pub const DIAGONALS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn update_opponent_movement_pattern(
    board: &[Vec<char>],
    player_symbol: char,
    territory_symbol: char,
    pattern: &mut VecDeque<(usize, usize)>,
) {
    let opponent_symbol = if player_symbol == '@' { '$' } else { '@' };
    let opponent_territory_symbol = if player_symbol == '@' { 's' } else { 'a' };

    for (y, row) in board.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == opponent_territory_symbol {
                pattern.push_front((y, x));
                if pattern.len() > 5 {
                    pattern.pop_back();
                }
                return;
            }
        }
    }
}

pub fn get_territory_metrics(
    board: &[Vec<char>],
    player_symbol: char,
    territory_symbol: char,
) -> (usize, usize, Vec<(usize, usize)>) {
    let mut size = 0;
    let mut enclosed = 0;
    let mut frontier = Vec::new();

    for y in 0..board.len() {
        for x in 0..board[0].len() {
            if board[y][x] == player_symbol || board[y][x] == territory_symbol {
                size += 1;
                let mut is_frontier = false;

                for &(dy, dx) in &DIRECTIONS {
                    let ny = y as i32 + dy;
                    let nx = x as i32 + dx;
                    if ny >= 0 && nx >= 0 && ny < board.len() as i32 && nx < board[0].len() as i32 {
                        if board[ny as usize][nx as usize] == '.' {
                            is_frontier = true;
                            break;
                        }
                    }
                }

                if is_frontier {
                    frontier.push((y, x));
                } else {
                    enclosed += 1;
                }
            }
        }
    }

    (size, enclosed, frontier)
}

pub fn predict_opponent_direction(pattern: &VecDeque<(usize, usize)>) -> (i32, i32) {
    if pattern.len() < 2 {
        return (0, 0);
    }

    let mut dx = 0;
    let mut dy = 0;
    let mut count = 0;

    for i in 1..pattern.len() {
        let (y1, x1) = pattern[i - 1];
        let (y2, x2) = pattern[i];
        dy += (y2 as i32 - y1 as i32).signum();
        dx += (x2 as i32 - x1 as i32).signum();
        count += 1;
    }

    if count > 0 {
        (dy / count as i32, dx / count as i32)
    } else {
        (0, 0)
    }
}

pub fn collect_territory_cells(
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

pub fn collect_opponent_territory(
    board: &[Vec<char>],
    player_symbol: char,
    territory_symbol: char,
) -> Vec<(usize, usize)> {
    let (opponent_symbol, opponent_territory) = if player_symbol == '@' {
        ('$', 's')
    } else {
        ('@', 'a')
    };
    collect_territory_cells(board, opponent_symbol, opponent_territory)
}

pub fn calculate_territory_center(positions: &[(usize, usize)]) -> (usize, usize) {
    if positions.is_empty() {
        return (0, 0);
    }
    let sum_y: usize = positions.iter().map(|&(y, _)| y).sum();
    let sum_x: usize = positions.iter().map(|&(_, x)| x).sum();
    let count = positions.len();
    (sum_y / count, sum_x / count)
}