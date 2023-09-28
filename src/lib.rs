#![allow(dead_code)]

struct Solution {}

impl Solution {
    fn permuate(items: &[i32], cur: &mut Vec<i32>, output: &mut Vec<Vec<i32>>) {
        if items.len() == 0 {
            output.push(cur.clone());
            return;
        }

        for i in 0..items.len() {
            let value = items[i];

            if i != 0 && value == items[i - 1] {
                continue;
            }

            let mut new_items = Vec::new();

            new_items.extend_from_slice(items);
            new_items.remove(i);

            cur.push(value);

            Self::permuate(&new_items, cur, output);
            cur.pop();
        }
    }
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut output = Vec::new();
        Self::permuate(&nums, &mut vec![], &mut output);
        return output;
    }
}
