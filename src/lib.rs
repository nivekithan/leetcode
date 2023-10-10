#![allow(dead_code)]

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut total_gas = 0;
        let mut prefix_gas = 0;

        let mut start = 0;

        for i in 0..gas.len() {
            let cost = gas[i] - cost[i];
            total_gas += cost;
            prefix_gas += cost;

            if prefix_gas < 0 {
                start = i + 1;
                prefix_gas = 0;
            }
        }

        match total_gas.cmp(&0) {
            Ordering::Less => return -1,
            _ => return start as i32,
        };
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 4, 2, 3, 5], vec![3, 1, 4, 5, 2]),
            4
        );
    }
}
