#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut i, mut j, mut k) = (0_usize, 0_usize, nums.len() - 1);

        while j <= k {
            match nums[j] {
                0 => {
                    nums.swap(i, j);
                    i += 1;
                    j += 1;
                }
                2 => {
                    nums.swap(k, j);
                    if k == usize::MIN {
                        break;
                    }

                    k -= 1;
                }
                _ => {
                    j += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        let mut input = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut input);

        assert_eq!(input, vec![0, 0, 1, 1, 2, 2]);
    }
}
