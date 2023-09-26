#![allow(dead_code)]

use std::cmp::Ordering;

#[derive(Debug)]
struct StateMachine {
    candidates: Vec<i32>,
    pub state: Vec<i32>,
    constraint: i32,
    cursor: usize,
    sum: i32,
}

impl StateMachine {
    fn new(mut candidates: Vec<i32>) -> Self {
        candidates.sort_unstable();

        return Self {
            candidates,
            state: Vec::new(),
            constraint: 0,
            cursor: 0,
            sum: 0,
        };
    }

    fn next(&mut self) -> Option<i32> {
        while self.cursor < self.candidates.len() {
            let value_under_cursor = self.candidates[self.cursor];

            if !(value_under_cursor > self.constraint) {
                self.cursor += 1;
                continue;
            }

            self.state.push(value_under_cursor);
            self.sum += value_under_cursor;

            return Some(self.sum);
        }

        return None;
    }

    fn backtrack(&mut self) -> Result<(), ()> {
        let first_ele = self.state.pop();
        let second_ele = self.state.pop();

        match (first_ele, second_ele) {
            (Some(f), Some(s)) => {
                self.sum -= f + s;
                self.constraint = s;
                self.cursor = 0;
                return Ok(());
            }

            (None, None) => Err(()),
            (None, _) => unreachable!(),
            (Some(f), None) => {
                self.sum = 0;
                self.constraint = f;
                self.cursor = 0;
                return Ok(());
            }
        }
    }

    fn backtrack_1(&mut self) -> Result<(), ()> {
        let first_ele = self.state.pop();

        match first_ele {
            Some(f) => {
                self.sum -= f;
                self.constraint = f;
                self.cursor = 0;
                return Ok(());
            }
            None => return Err(()),
        };
    }
}

struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = vec![];
        let mut state_machine = StateMachine::new(candidates);

        loop {
            while let Some(sum) = state_machine.next() {
                println!("Statemachine: {state_machine:?}");
                match sum.cmp(&target) {
                    Ordering::Less => continue,
                    Ordering::Equal => {
                        let solution = state_machine.state.iter().map(|v| *v).collect::<Vec<i32>>();
                        output.push(solution);
                        continue;
                    }
                    Ordering::Greater => {
                        let res = state_machine.backtrack();

                        match res {
                            Ok(_) => continue,
                            Err(_) => break,
                        };
                    }
                }
            }

            match state_machine.backtrack_1() {
                Err(_) => break,
                Ok(_) => continue,
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
        let input = vec![2, 3, 6, 7];
        let target = 7;

        assert_eq!(
            Solution::combination_sum(input, target),
            vec![vec![2, 2, 3], vec![7]]
        );
    }

    #[test]
    fn test_2() {
        let input = vec![2, 3, 5];
        let target = 8;

        assert_eq!(
            Solution::combination_sum(input, target),
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        );
    }

    #[test]
    fn test_3() {
        let input = vec![2];
        let target = 1;

        assert_eq!(
            Solution::combination_sum(input, target),
            Vec::<Vec<i32>>::new()
        );
    }

    #[test]
    fn test_4() {
        let input = vec![7, 3, 2];
        let target = 18;

        Solution::combination_sum(input, target);
    }
}
