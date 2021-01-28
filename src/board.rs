use rand;
use rand::Rng;

use crate::board::Direction::*;
use crate::board::MoveResult::{Loss, Ongoing, NoMove, Win};

const TWO_CHANCE: f64 = 0.75;

#[derive(Debug)]
pub struct Board {
    rows: [[usize; 4]; 4]
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        for (r1, r2) in self.rows.iter().zip(other.rows.iter()) {
            for (val1,val2) in r1.iter().zip(r2.iter()) {
                if *val1 != *val2 {
                    return false;
                }
            }
        }
        true
    }
}

impl Eq for Board {}

impl Board {
    /// Generate an empty position for 2048.
    pub fn new() -> Board {
        Board {
            rows: [[0; 4]; 4]
        }
    }

    /// Generate a start position for 2048.
    pub fn startpos() -> Board {
        let mut rng = rand::thread_rng();

        let i = rng.gen_range(0..16);
        let j = loop {
            let val = rng.gen_range(0..16);
            if i != val {
                break val;
            }
        };

        let mut rows = [[0; 4]; 4];
        let mut two_or_four = || {
            if rng.gen_bool(TWO_CHANCE) {
                2
            } else {
                4
            }
        };
        rows[i/4][i%4] = two_or_four(); // two random positions on the board are set to nonzero
        rows[j/4][j%4] = two_or_four();
        Board {
            rows
        }
    }

    /// Swipe the board in a certain direction,
    /// check for victory,
    /// add an additional tile where possible,
    /// and then check for defeat.
    pub fn mv(&mut self, dir: Direction) -> MoveResult {
        if !self.is_mergeable(dir) {
            return NoMove;
        }

        self.merge(dir);

        if self.is_win() {
            return Win;
        }

        // add new 2 or 4
        let mut rng = rand::thread_rng();
        let (r,c) = {
            let mut zero_indices: Vec<(usize, usize)> = Vec::new();
            for i in 0..self.rows.len() {
                for j in 0..self.rows[0].len() {
                    if self.rows[i][j] == 0 {
                        zero_indices.push((i, j));
                    }
                }
            }
            zero_indices[rng.gen_range(0, zero_indices.len())]
        };

        self.rows[r][c] = if rng.gen_bool(TWO_CHANCE) { 2 } else { 4 };

        if self.is_loss() {
            return Loss;
        } else {
            return Ongoing;
        }
    }

    /// Merge in a direction.
    fn merge(&mut self, dir: Direction) {
        match dir {
            Right => {
                for r_index in 0..self.rows.len() {
                    let r = self.rows[r_index];
                    let mut arr = [0; 4]; // build new array from right
                    let mut i = 3;
                    let mut row_iter = r.iter().filter(|x| **x != 0).rev();
                    let mut prev = *row_iter.next().expect("couldn't get nonzero value from array");
                    for val in row_iter {
                    // go through row, record nonzero values, calculate new row
                        if prev == *val {
                            arr[i] = 2 * prev;
                            prev = 0; // to prevent too many doublings
                        } else {
                            if prev == 0 {
                                prev = *val;
                                continue;
                            }
                            arr[i] = prev;
                            prev = *val;
                        }
                        i -= 1;
                    }
                    if prev != 0 {
                        arr[i] = prev;
                    }
                    self.rows[r_index] = arr;
                }
            },
            Left => {
                for r_index in 0..self.rows.len() {
                    let r = self.rows[r_index];
                    let mut arr = [0; 4]; // build new array from left
                    let mut i = 0;
                    let mut row_iter = r.iter().filter(|x| **x != 0);
                    let mut prev = *row_iter.next().expect("couldn't get nonzero value from array");
                    for val in row_iter {
                    // go through row, record nonzero values, calculate new row
                        if prev == *val {
                            arr[i] = 2 * prev;
                            prev = 0; // to prevent too many doublings
                        } else {
                            if prev == 0 {
                                prev = *val;
                                continue;
                            }
                            arr[i] = prev;
                            prev = *val;
                        }
                        i += 1;
                    }
                    if prev != 0 {
                        arr[i] = prev;
                    }
                    self.rows[r_index] = arr;
                }
            },
            Up => {
                let mut new_rows = [[0;4];4];
                for c_index in 0..self.rows[0].len() {
                    let mut old_col = [0;4];
                    {
                        let mut i = 0;
                        for r in self.rows.iter() {
                            old_col[i] = r[c_index];
                            i += 1;
                        }
                    }

                    let mut i = 0;
                    let mut col_iter = old_col.iter().filter(|x| **x != 0);
                    let mut prev;
                    match col_iter.next() {
                        Some(n) => prev = *n,
                        None => continue,
                    }
                    for val in col_iter {
                        // go through col, record nonzero values, calculate new row
                        if prev == *val {
                            new_rows[i][c_index] = 2 * prev;
                            prev = 0; // to prevent too many doublings
                        } else {
                            if prev == 0 {
                                prev = *val;
                                continue;
                            }
                            new_rows[i][c_index] = prev;
                            prev = *val;
                        }
                        i += 1;
                    }
                    if prev != 0 {
                        new_rows[i][c_index] = prev;
                    }
                }
                self.rows = new_rows;
            },
            Down => {
                let mut new_rows = [[0;4];4];
                for c_index in 0..self.rows[0].len() {
                    let mut old_col = [0;4];
                    {
                        let mut i = 0;
                        for r in self.rows.iter() {
                            old_col[i] = r[c_index];
                            i += 1;
                        }
                    }

                    let mut i = 3;
                    let mut col_iter = old_col.iter().filter(|x| **x != 0).rev();
                    let mut prev;
                    match col_iter.next() {
                        Some(n) => prev = *n,
                        None => continue,
                    }
                    for val in col_iter {
                        // go through col, record nonzero values, calculate new row
                        if prev == *val {
                            new_rows[i][c_index] = 2 * prev;
                            prev = 0; // to prevent too many doublings
                        } else {
                            if prev == 0 {
                                prev = *val;
                                continue;
                            }
                            new_rows[i][c_index] = prev;
                            prev = *val;
                        }
                        i -= 1;
                    }
                    if prev != 0 {
                        new_rows[i][c_index] = prev;
                    }
                }
                self.rows = new_rows;
            },
        }
    }
    
    /// Check that board can be merged in a direction
    fn is_mergeable(&self, dir: Direction) -> bool {
        let condition = |prev: usize, curr: usize| {
            (prev == 0 && curr != 0) || (prev != 0 && prev == curr)
        };
        match dir {
            Down => {
                for col in 0..4 {
                    let mut prev = self.rows[3][col];
                    for r in self.rows.iter().rev().skip(1) {
                        if condition(prev, r[col]) {
                            return true;
                        }
                        prev = r[col];
                    }
                }
            },
            Up => {
                for col in 0..4 {
                    let mut prev = self.rows[0][col];
                    for r in self.rows.iter().skip(1) {
                        if condition(prev, r[col]) {
                            return true;
                        }
                        prev = r[col];
                    }
                }
            },
            Left => {
                for r in &self.rows {
                    let mut prev = r[0];
                    for curr in r.iter().skip(1) {
                        if condition(prev, *curr) {
                            return true;
                        }
                        prev = *curr;
                    }
                }
            },
            Right => {
                for r in &self.rows {
                    let mut prev = r[3];
                    for curr in r.iter().rev().skip(1) {
                        if condition(prev, *curr) {
                            return true;
                        }
                        prev = *curr;
                    }
                }
            },
        }
        false
    }

    fn is_win(&self) -> bool {
        for r in &self.rows {
            for i in r {
                if *i == 2048 {
                    return true;
                }
            }
        }
        false
    }

    /// Check that no merges can be done in any direction
    fn is_loss(&self) -> bool {
        !(self.is_mergeable(Up)
        || self.is_mergeable(Down)
        || self.is_mergeable(Left)
        || self.is_mergeable(Right))
    }
}

impl std::fmt::Display for Board {
    /// Prints board. For example:
    /// +----+----+----+----+
    /// |    |    |    |    |
    /// +----+----+----+----+
    /// |    |    |   2|    |
    /// +----+----+----+----+
    /// |    |    |    |    |
    /// +----+----+----+----+
    /// |    |    |    |   2|
    /// +----+----+----+----+
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let separator = "+----+----+----+----+";
        writeln!(f, "{}", separator)?;
        for r in &self.rows {
            write!(f, "|")?;
            for i in r {
                if *i != 0 {
                    write!(f, "{:4}|", i)?;
                } else {
                    write!(f, "    |")?;
                }
            }
            writeln!(f, "\n{}", separator)?;
        }
        Ok(())
    }
}

enum MoveResult {
    Ongoing, Win, Loss, NoMove
}

#[derive(Clone, Copy)]
enum Direction {
    Up, Down, Left, Right
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}", match self {
                Left  => "←",
                Up    => "↑",
                Right => "→",
                Down  => "↓",
            }
        )
    }
}

mod testing {
    use crate::board::{Board, Direction};
    use crate::board::Direction::*;

    #[test]
    fn test_merge_1() {
        let board = Board {
            rows: [
                [0;4],
                [0;4],
                [0,1,0,0],
                [0;4],
            ]
        };
        assert!(board.is_mergeable(Right));
        assert!(board.is_mergeable(Left));
        assert!(board.is_mergeable(Down));
        assert!(board.is_mergeable(Up));
    }

    #[test]
    fn test_merge_2() {
        let board = Board {
            rows: [
                [0;4],
                [0;4],
                [1,0,0,0],
                [0;4],
            ]
        };
        assert!(board.is_mergeable(Right));
        assert_eq!(board.is_mergeable(Left), false);
        assert!(board.is_mergeable(Down));
        assert!(board.is_mergeable(Up));
    }
    #[test]
    fn test_merge_3() {
        let board = Board {
            rows: [
                [1;4],
                [1;4],
                [1;4],
                [1;4],
            ]
        };
        assert!(board.is_mergeable(Right));
        assert!(board.is_mergeable(Left));
        assert!(board.is_mergeable(Down));
        assert!(board.is_mergeable(Up));
    }
    #[test]
    fn test_merge_4() {
        let board = Board {
            rows: [
                [1;4],
                [2;4],
                [3;4],
                [4;4],
            ]
        };
        assert!(board.is_mergeable(Right));
        assert!(board.is_mergeable(Left));
        assert_eq!(false, board.is_mergeable(Down));
        assert_eq!(false, board.is_mergeable(Up));
    }
    #[test]
    fn test_merge_5() {
        let mut k = 1;
        let mut rows = [[0;4];4];
        for i in 0..(rows.len()) {
            for j in 0..(rows[0].len()) {
                rows[i][j] = k;
                k += 1;
            }
        }
        let board = Board {
            rows
        };
        println!("{}", board);
        assert_eq!(false, board.is_mergeable(Right));
        assert_eq!(false, board.is_mergeable(Left));
        assert_eq!(false, board.is_mergeable(Down));
        assert_eq!(false, board.is_mergeable(Up));
    }
    #[test]
    fn test_merge_6() {
        let mut k = 0;
        let mut rows = [[0;4];4];
        for i in 0..(rows.len()) {
            for j in 0..(rows[0].len()) {
                rows[i][j] = k;
                k += 1;
            }
        }
        let board = Board {
            rows
        };
        println!("{}", board);
        assert_eq!(false, board.is_mergeable(Right));
        assert_eq!(true, board.is_mergeable(Left));
        assert_eq!(false, board.is_mergeable(Down));
        assert_eq!(true, board.is_mergeable(Up));
    }
    #[test]
    fn test_merge_7() {
        let board = Board {
            rows: [
                [2,3,4,5],
                [1,5,6,1],
                [5,1,1,2],
                [4,5,6,7],
            ]
        };
        assert!(board.is_mergeable(Right));
        assert!(board.is_mergeable(Left));
        assert_eq!(false, board.is_mergeable(Down));
        assert_eq!(false, board.is_mergeable(Up));
    }
    #[test]
    fn test_merge_8() {
        let mut board = Board {
            rows: [
                [2,3,4,5],
                [1,5,6,1],
                [5,7,1,2],
                [4,5,6,2],
            ]
        };
        let merge_u = Board {
            rows: [
                [2,3,4,5],
                [1,5,6,1],
                [5,7,1,4],
                [4,5,6,0],
            ]
        };
        assert!(board.is_mergeable(Up));
        assert!(board.is_mergeable(Down));
        assert_eq!(false, board.is_mergeable(Left));
        assert_eq!(false, board.is_mergeable(Right));
        println!("{}", board);
        board.merge(Up);
        println!("{}", board);
        assert_eq!(board, merge_u)
    }
    #[test]
    fn test_merge_9() {
        let mut board = Board {
            rows: [
                [2,4,4,5],
                [1,5,6,1],
                [5,7,1,2],
                [4,5,6,1],
            ]
        };
        let mut board2 = Board {
            rows: [
                [2,4,4,5],
                [1,5,6,1],
                [5,7,1,2],
                [4,5,6,1],
            ]
        };
        let merge_r = Board {
            rows: [
                [0,2,8,5],
                [1,5,6,1],
                [5,7,1,2],
                [4,5,6,1],
            ]
        };
        let merge_l = Board {
            rows: [
                [2,8,5,0],
                [1,5,6,1],
                [5,7,1,2],
                [4,5,6,1],
            ]
        };
        assert!(board.is_mergeable(Right));
        assert!(board.is_mergeable(Left));
        assert_eq!(false, board.is_mergeable(Down));
        assert_eq!(false, board.is_mergeable(Up));
        board.merge(Right);
        assert_eq!(board, merge_r);
        board2.merge(Left);
        assert_eq!(board2, merge_l);
    }
    #[test]
    fn test_merge_10() {
        let mut board = Board {
            rows: [
                [1,2,3,4],
                [1,2,3,4],
                [1,2,3,4],
                [1,2,3,4],
            ]
        };
        let mut board2 = Board {
            rows: [
                [1,2,3,4],
                [1,2,3,4],
                [1,2,3,4],
                [1,2,3,4],
            ]
        };
        let merge_d = Board {
            rows: [
                [0;4],
                [0;4],
                [2,4,6,8],
                [2,4,6,8],
            ]
        };
        let merge_u = Board {
            rows: [
                [2,4,6,8],
                [2,4,6,8],
                [0;4],
                [0;4],
            ]
        };
        assert!(board.is_mergeable(Up));
        assert!(board.is_mergeable(Down));
        assert_eq!(false, board.is_mergeable(Left));
        assert_eq!(false, board.is_mergeable(Right));
        board.merge(Up);
        assert_eq!(board, merge_u);
        board2.merge(Down);
        assert_eq!(board2, merge_d);
    }
}
