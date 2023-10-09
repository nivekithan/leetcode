#![allow(dead_code)]

struct Solution {}

struct Window {
    pub left: usize,
    pub right: usize,
}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut window = Window { left: 0, right: 0 };

        while window.right < nums.len() - 1 {
            let mut furthest = 0;
            for i in window.left..(window.right + 1) {
                furthest = furthest.max(i + nums[i] as usize);
            }

            if furthest <= window.right {
                return false;
            }

            window.left = window.right + 1;
            window.right = furthest;
        }

        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_1() {
        let input = vec![2, 3, 1, 1, 4];

        assert_eq!(Solution::can_jump(input), true);

        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
