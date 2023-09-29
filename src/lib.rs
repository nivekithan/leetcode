#![allow(dead_code)]

struct Solution {}

impl Solution {
    fn find_subset(items: &[i32], cur: &mut Vec<i32>, output: &mut Vec<Vec<i32>>) {
        output.push(cur.clone());

        items.iter().enumerate().for_each(|(i, value)| {
            let new_items = &items[i + 1..];

            cur.push(*value);
            Self::find_subset(&new_items, cur, output);
            cur.pop();
        });
    }

    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut output = Vec::new();

        Self::find_subset(&nums, &mut vec![], &mut output);

        return output;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_1() {
        let input = vec![1, 2, 3];

        assert_eq!(
            Solution::subsets(input),
            vec![
                vec![],
                vec![1],
                vec![1, 2],
                vec![1, 2, 3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![3],
            ]
        )
    }
}
