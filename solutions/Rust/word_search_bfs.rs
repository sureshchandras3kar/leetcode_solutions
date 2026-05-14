use std::collections::{HashSet, VecDeque};

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            if bfs(&board, &word, i, j) { return true; }
        }
    }
    false
}

fn bfs(board: &Vec<Vec<char>>, word: &str, start_row: usize, start_col: usize) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col, 0, HashSet::new()));

    while let Some((row, col, index, visited)) = queue.pop_front() {
        if index == word.len() { return true; }
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dr, dc) in dirs {
            let nr = (row as i32 + dr) as usize;
            let nc = (col as i32 + dc) as usize;
            if nr < board.len() && nc < board[0].len() &&
               board[nr][nc] == word.chars().nth(index).unwrap() && !visited.contains(&(nr, nc)) {
                let mut newVis = visited.clone();
                newVis.insert((nr, nc));
                queue.push_back((nr, nc, index + 1, newVis));
            }
        }
    }
    false
}
