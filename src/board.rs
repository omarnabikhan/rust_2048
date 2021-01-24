use rand;
use rand::Rng;

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