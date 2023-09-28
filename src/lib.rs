struct Solution {}

impl Solution {
    fn find_perumentation(items: &[i32], output: &mut Vec<Vec<i32>>, cur: &mut Vec<i32>) {
        if items.len() == 0 {
            output.push(cur.clone());
        }

        for i in 0..items.len() {
            let v = items[i];

            let mut new_array: Vec<i32> = Vec::new();

            new_array.extend_from_slice(items);
            new_array.remove(i);

            cur.push(v);
            Self::find_perumentation(&new_array[0..], output, cur);
            cur.pop();
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output: Vec<Vec<i32>> = Vec::new();

        Self::find_perumentation(&nums[0..], &mut output, &mut vec![]);

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
            Solution::permute(input),
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        )
    }
}
