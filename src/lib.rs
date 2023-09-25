pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let k_1 = nums[0];

    if k_1 == target {
        return 0;
    }

    let index = binary_search(&nums[0..], target, k_1);

    match index {
        None => return -1,
        Some(v) => return v as i32,
    };

    fn binary_search(slice: &[i32], target: i32, k_1: i32) -> Option<usize> {
        if slice.len() == 0 {
            return None;
        }

        if slice.len() == 1 {
            if slice[0] == target {
                return Some(0);
            }

            return None;
        }

        let low: usize = 0;
        let high = slice.len() - 1;

        let middle = find_middle(low, high);

        let middle_value = slice[middle];

        if middle_value == target {
            return Some(middle);
        }

        if middle_value > target {
            if middle_value >= k_1 && target > k_1 {
                return binary_search(&slice[0..middle], target, k_1);
            }

            if middle_value < k_1 && target > k_1 {
                unreachable!();
            }

            if middle_value >= k_1 && target < k_1 {
                return correct_index(
                    binary_search(&slice[middle + 1..slice.len()], target, k_1),
                    middle + 1,
                );
            }

            if middle_value < k_1 && target < k_1 {
                return binary_search(&slice[0..middle], target, k_1);
            }
        }

        if middle_value < target {
            if middle_value >= k_1 && target > k_1 {
                return correct_index(
                    binary_search(&slice[middle + 1..slice.len()], target, k_1),
                    middle + 1,
                );
            }

            if middle_value < k_1 && target > k_1 {
                return binary_search(&slice[0..middle], target, k_1);
            }

            if middle_value >= k_1 && target < k_1 {
                unreachable!();
            }

            if middle_value < k_1 && target < k_1 {
                return correct_index(
                    binary_search(&slice[middle + 1..slice.len()], target, k_1),
                    middle + 1,
                );
            }
        }

        unreachable!();
    }

    fn find_middle(low: usize, high: usize) -> usize {
        if (low + high) % 2 == 0 {
            return low / high;
        }

        return (low + high + 1) / 2;
    }

    fn correct_index(index: Option<usize>, padding: usize) -> Option<usize> {
        match index {
            None => None,
            Some(i) => Some(i + padding),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::search;

    #[test]
    fn test() {
        let input = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;

        assert_eq!(search(input, target), 4);
    }
    #[test]
    fn test_2() {
        let input = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;

        assert_eq!(search(input, target), -1);
    }

    #[test]
    fn test_3() {
        let input = vec![1];
        let target = 0;

        assert_eq!(search(input, target), -1);
    }
}
