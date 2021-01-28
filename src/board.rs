use rand;
use rand::Rng;

use crate::board::Direction::*;
use std::fmt::Debug;

pub struct Board {
    rows: [[usize; 4]; 4]
}

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
            if rng.gen_bool(0.75) {
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
    /// Returns Err if the game is over.
    pub fn mv(&mut self, dir: Direction) -> Result<(),()> {
        if self.is_mergeable(dir) {
            self.merge(dir);
        } else {
            // no turn

        }

        if self.is_win() {

        }
        // add new 2 or 4
        if self.is_loss() {
            return Err(());
        }
        Ok(())
    }

    /// Merge in a direction.
    fn merge(&mut self, dir: Direction) {
        match dir {
            Right => {
                for r in self.rows.iter_mut() {
                    let mut arr = [0; 4]; // build new array from right
                    let mut i = 3;
                    let mut row_iter = r.iter().filter(|x| **x > 0).rev();
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
                }
            }
            _ => panic!(),
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
        let board = Board {
            rows: [
                [2,3,4,5],
                [1,5,6,1],
                [5,7,1,2],
                [4,5,6,2],
            ]
        };
        assert!(board.is_mergeable(Up));
        assert!(board.is_mergeable(Down));
        assert_eq!(false, board.is_mergeable(Left));
        assert_eq!(false, board.is_mergeable(Right));
    }
}
