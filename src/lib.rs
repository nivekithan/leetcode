#![allow(dead_code)]

struct Solution {}

struct Window {
    pub left: usize,
    pub right: usize,
}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps = 0;

        let mut window = Window { left: 0, right: 0 };

        while window.right < nums.len() - 1 {
            let mut furthest = 0;
            for i in window.left..(window.right + 1) {
                furthest = furthest.max(nums[i] + i as i32);
            }
            window.left = window.right + 1;
            window.right = furthest as usize;
            steps += 1;
        }

        return steps;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_1() {
        let input = vec![2, 3, 1, 1, 4];

        assert_eq!(Solution::jump(input), 2);

        let input = vec![2, 3, 0, 1, 4];

        assert_eq!(Solution::jump(input), 2);

        let input = vec![9, 8, 2, 2, 0, 2, 2, 0, 4, 1, 5, 7, 9, 6, 6, 0, 6, 5, 0, 5];

        assert_eq!(Solution::jump(input), 3);
    }
}
