#![allow(dead_code)]

use std::cmp::Ordering;

struct Solution {}

struct Digits(usize);

impl PartialEq for Digits {
    fn eq(&self, other: &Self) -> bool {
        return self.0 == other.0;
    }
}

impl Eq for Digits {}

impl PartialOrd for Digits {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_str = format!("{}", self.0);
        let other_str = format!("{}", other.0);

        if self_str.len() == other_str.len() {
            return Some(self.0.cmp(&other.0));
        } else if self_str.len() > other_str.len() {
            let considered_self_str = &self_str[0..other_str.len()];
            let considered_value = usize::from_str_radix(&considered_self_str, 10).unwrap();

            if considered_value == other.0 {
                let considered_next_digit = usize::from_str_radix(
                    self_str.get(other_str.len()..other_str.len() + 1).unwrap(),
                    10,
                )
                .unwrap();
                let considered_first_digit =
                    usize::from_str_radix(&considered_self_str[0..1], 10).unwrap();

                return Some(considered_next_digit.cmp(&considered_first_digit));
            }

            return Some(considered_value.cmp(&other.0));
        } else {
            let considered_other_str = &other_str[0..self_str.len()];
            let considered_value = usize::from_str_radix(&considered_other_str, 10).unwrap();

            if considered_value == self.0 {
                let considered_next_digit = usize::from_str_radix(
                    other_str.get(self_str.len()..self_str.len() + 1).unwrap(),
                    10,
                )
                .unwrap();
                let considered_first_digit =
                    usize::from_str_radix(&considered_other_str[0..1], 10).unwrap();

                return Some(considered_first_digit.cmp(&considered_next_digit));
            }

            return Some(considered_value.cmp(&self.0));
        }
    }
}

impl Ord for Digits {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut digits: Vec<Digits> = nums
            .iter()
            .map(|v| {
                return Digits(*v as usize);
            })
            .collect();

        digits.sort_unstable();

        let mut output = String::new();
        digits
            .iter()
            .map(|v| v.0)
            .map(|v| format!("{}", v))
            .for_each(|s| {
                output.push_str(&s);
            });

        return output;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::largest_number(vec![10, 2]), String::from("210"));

        assert_eq!(
            Solution::largest_number(vec![34, 340]),
            String::from("34340")
        );
        assert_eq!(
            Solution::largest_number(vec![34, 344]),
            String::from("34434")
        );
    }
}
