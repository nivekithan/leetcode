#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut sum = 0;

        for i in 0..nums.len() {
            let value = nums[i];
            sum += value;

            max_sum = max_sum.max(sum);

            if sum < 0 {
                sum = 0;
            }
        }

        return max_sum;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }
}
