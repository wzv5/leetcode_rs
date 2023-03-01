/*
 * @lc app=leetcode id=51 lang=rust
 *
 * [51] N-Queens
 */

// @lc code=start
#[derive(Debug, Clone)]
struct Board {
    tiles: Vec<Vec<i32>>,
    n: usize,
}
impl Board {
    fn new(n: i32) -> Self {
        let n = n as usize;
        Board {
            tiles: vec![vec![0; n]; n],
            n: n as usize,
        }
    }
    fn do_mark(&mut self, row: usize, col: usize, delta: i32) {
        if row < self.n && col < self.n {
            self.tiles[row][col] += delta;
        }
        // if the tile is outside the board, do nothing
    }
    fn mark_tile(&mut self, row: usize, col: usize) {
        self.do_mark(row, col, 1)
    }
    fn demark_tile(&mut self, row: usize, col: usize) {
        self.do_mark(row, col, -1)
    }
    fn do_place(&mut self, row: usize, col: usize, delta: i32) {
        // mark on the each tile in the same row
        for i in 0..self.n {
            self.do_mark(row, i, delta);
        }
        // same for column
        for i in 0..self.n {
            self.do_mark(i, col, delta);
        }
        // for the diagonal
        // bottom left to top right
        let row_intercept = row + col;
        for c_ in 0..self.n {
            let r_ = if row_intercept >= c_ {
                row_intercept - c_
            } else {
                continue;
            };
            self.do_mark(r_, c_, delta);
        }
        // top left to bottom right
        let row_at_last_col = row + self.n - col - 1;
        for c_ in 0..self.n {
            let col_dist = self.n - 1 - c_;
            let r_ = if row_at_last_col >= col_dist {
                row_at_last_col - col_dist
            } else {
                continue;
            };
            self.do_mark(r_, c_, delta);
        }
    }
    fn place_queen(&mut self, row: usize, col: usize) {
        self.do_place(row, col, 1);
        self.tiles[row][col] = 0;
    }
    fn remove_queen(&mut self, row: usize, col: usize) {
        self.do_place(row, col, -1);
        self.tiles[row][col] = 0;
    }
    fn check_tile(&self, row: usize, col: usize) -> bool {
        self.tiles[row][col] == 0
    }
    fn to_answer(&self) -> Vec<String> {
        let mut result = vec![String::new(); self.n];
        for (row, result_) in self.tiles.iter().zip(result.iter_mut()) {
            for tile in row {
                result_.push(if *tile == 0 { 'Q' } else { '.' });
            }
        }
        result
    }
    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.tiles {
            for tile in row {
                result.push_str(format!("{} ", tile).as_str());
            }
            result.push('\n');
        }
        return result;
    }
}
impl Solution {
    fn search(board: &mut Board, row: usize, result: &mut Vec<Board>) {
        // for each tile in the row, check the tile, if it is safe, place a queen
        // println!("row: {}, board:\n{}", row, board.to_string());
        for col in 0..board.n {
            if board.check_tile(row, col) {
                // println!("trying to place queen at ({}, {})", row, col);
                board.place_queen(row, col);
                // println!("board after place:\n{}", board.to_string());
                // if the row is the last row, push the board to the result
                if row == board.n - 1 {
                    result.push(board.clone());
                    // remove this placed queen
                    board.remove_queen(row, col);
                    // println!("removed queen at ({}, {})", row, col);
                    // println!("board:\n{}", board.to_string());
                    return;
                }
                Self::search(board, row + 1, result);
                board.remove_queen(row, col);
                // println!("removed queen at ({}, {})", row, col);
                // println!("board:\n{}", board.to_string());
            }
        }
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board = Board::new(n);
        let mut result = vec![];
        Self::search(&mut board, 0, &mut result);
        result.iter().map(|board| board.to_answer()).collect()
    }
}
// @lc code=end

struct Solution;

#[test]
fn test() {
    assert_eq!(
        Solution::solve_n_queens(4),
        vec![
            vec![".Q..", "...Q", "Q...", "..Q.",],
            vec!["..Q.", "Q...", "...Q", ".Q..",],
        ]
    );
    assert_eq!(
        Solution::solve_n_queens(1),
        vec![vec!["Q",],]
    );
    // 8
    assert_eq!(
        Solution::solve_n_queens(8).len(),
        92
    );
    assert_eq!(
        Solution::solve_n_queens(12).len(),
        14200
    )
}
