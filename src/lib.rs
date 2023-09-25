#![allow(unused)]

use std::collections::HashMap;

#[derive(Default)]
struct SudokoHashmap {
    row_hashmap: [HashMap<char, bool>; 9],
    col_hashmap: [HashMap<char, bool>; 9],
    sqaure_hashmap: [HashMap<char, bool>; 9],
}

impl SudokoHashmap {
    fn insert(&mut self, loc: (usize, usize), value: char) {
        let row = loc.0;
        let col = loc.1;

        let squaure = match (((col / 3) + 1), ((row / 3) + 1)) {
            (1, 1) => 0_usize,
            (2, 1) => 1,
            (3, 1) => 2,
            (1, 2) => 3,
            (2, 2) => 4,
            (3, 2) => 5,
            (1, 3) => 6,
            (2, 3) => 7,
            (3, 3) => 8,

            _ => unreachable!(),
        };

        self.row_hashmap[row].insert(value, true);
        self.col_hashmap[col].insert(value, true);
        self.sqaure_hashmap[squaure].insert(value, true);
    }

    fn has(&self, loc: (usize, usize), value: char) -> bool {
        let row = loc.0;
        let col = loc.1;

        let squaure = match (((col / 3) + 1), ((row / 3) + 1)) {
            (1, 1) => 0_usize,
            (2, 1) => 1,
            (3, 1) => 2,
            (1, 2) => 3,
            (2, 2) => 4,
            (3, 2) => 5,
            (1, 3) => 6,
            (2, 3) => 7,
            (3, 3) => 8,

            _ => unreachable!(),
        };

        match (
            self.row_hashmap[row].contains_key(&value),
            self.col_hashmap[col].contains_key(&value),
            self.sqaure_hashmap[squaure].contains_key(&value),
        ) {
            (false, false, false) => return false,
            _ => return true,
        };
    }
}

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut sudoko_hashmap: SudokoHashmap = Default::default();

        for i in 0..9_usize {
            for j in 0..9_usize {
                let loc = (i, j);
                let value = board[i][j];

                if value == '.' {
                    continue;
                }

                if !sudoko_hashmap.has(loc, value) {
                    sudoko_hashmap.insert(loc, value);
                } else {
                    return false;
                }
            }
        }
        return true;
    }
}
