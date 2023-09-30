#![allow(dead_code)]

struct Solution {}

impl Solution {
    fn continous_checker(
        board: &[Vec<char>],
        exists_checker: &mut [Vec<bool>],
        pos: (usize, usize),
        target: &str,
    ) -> bool {
        let i = pos.0;
        let j = pos.1;

        let max_i = board.len();
        let max_j = board[0].len();

        let cur_char = board[i][j];

        if cur_char == target.chars().next().unwrap() {
            if target.len() == 1 {
                return true;
            }

            exists_checker[i][j] = true;

            for pos in Self::get_next_postions((i, j), (max_i, max_j), &exists_checker).iter() {
                let i = pos.0;
                let j = pos.1;

                let is_exists =
                    Self::continous_checker(board, exists_checker, (i, j), &target[1..]);

                if is_exists {
                    return true;
                }
            }

            exists_checker[i][j] = false;
        }
        return false;
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut exists_checker = vec![vec![false; board[0].len()]; board.len()];

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let is_exists = Self::continous_checker(&board, &mut exists_checker, (i, j), &word);

                if is_exists {
                    return true;
                }
            }
        }

        return false;
    }

    fn get_next_postions(
        cur_pos: (usize, usize),
        max: (usize, usize),
        exists_checker: &[Vec<bool>],
    ) -> Vec<(usize, usize)> {
        let mut output: Vec<(usize, usize)> = vec![];

        if cur_pos.1 != 0 {
            let new_pos = (cur_pos.0, cur_pos.1 - 1);

            if !exists_checker[new_pos.0][new_pos.1] {
                output.push(new_pos);
            }
        }

        if cur_pos.0 + 1 < max.0 {
            let new_pos = (cur_pos.0 + 1, cur_pos.1);

            if !exists_checker[new_pos.0][new_pos.1] {
                output.push(new_pos);
            }
        }

        if cur_pos.1 + 1 < max.1 {
            let new_pos = (cur_pos.0, cur_pos.1 + 1);

            if !exists_checker[new_pos.0][new_pos.1] {
                output.push(new_pos);
            }
        }

        if cur_pos.0 != 0 {
            let new_pos = (cur_pos.0 - 1, cur_pos.1);

            if !exists_checker[new_pos.0][new_pos.1] {
                output.push(new_pos);
            }
        }

        return output;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_1() {
        let input = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        let target = "ABCCED";

        assert_eq!(Solution::exist(input, target.to_string()), true);
    }

    #[test]
    fn test_2() {
        let input = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        let target = "SEE";

        assert_eq!(Solution::exist(input, target.to_string()), true);
    }

    #[test]
    fn test_3() {
        let input = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        let target = "ABCB";

        assert_eq!(Solution::exist(input, target.to_string()), false);
    }
}
