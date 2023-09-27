struct Solution {}

impl Solution {
    fn combi(candidates: &[i32], cur_path: &mut Vec<i32>, output: &mut Vec<Vec<i32>>, target: i32) {
        if target == 0 {
            output.push(cur_path.clone());
            return;
        }

        for i in 0..candidates.len() {
            let value = candidates[i];

            if value > target {
                break;
            }

            if i != 0 && value == candidates[i - 1] {
                continue;
            }

            cur_path.push(value);
            Self::combi(
                &candidates[i + 1..],
                cur_path,
                output,
                target - candidates[i],
            );
            cur_path.pop();
        }
    }

    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();

        let mut output: Vec<Vec<i32>> = Vec::new();

        Self::combi(&candidates[0..], &mut Vec::new(), &mut output, target);

        return output;
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_1() {
        let input = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;

        assert_eq!(
            Solution::combination_sum2(input, target),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }
}
