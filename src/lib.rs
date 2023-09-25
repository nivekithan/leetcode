pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let indexes = binary_search(&nums[0..], target);

    match indexes {
        (Some(l), Some(r)) => return vec![l as i32, r as i32],
        (None, None) => return vec![-1, -1],
        _ => unreachable!(),
    };

    fn binary_search(slice: &[i32], target: i32) -> (Option<usize>, Option<usize>) {
        if slice.len() == 0 {
            return (None, None);
        }

        if slice.len() == 1 {
            if slice[0] == target {
                return (Some(0), Some(0));
            }

            return (None, None);
        }

        let low: usize = 0;
        let high = slice.len() - 1;

        let middle = find_middle(low, high);
        let middle_value = slice[middle];

        if middle_value == target {
            let is_left_starting = middle == 0 || slice[middle - 1] != target;

            let is_right_starting = middle == slice.len() - 1 || slice[middle + 1] != target;

            println!(
                "is_left_starting: {is_left_starting}, is_right_starting: {is_right_starting} "
            );

            if is_right_starting && is_left_starting {
                return (Some(middle), Some(middle));
            }

            if is_left_starting && !is_right_starting {
                let (_, right_index) =
                    set_index(binary_search(&slice[middle + 1..], target), middle + 1);

                match right_index {
                    None => unreachable!(),
                    Some(r) => return (Some(middle), Some(r)),
                };
            }

            if !is_left_starting && is_right_starting {
                let (left_index, _) = binary_search(&slice[0..middle], target);

                println!("{left_index:?}");

                match left_index {
                    None => unreachable!(),
                    Some(l) => return (Some(l), Some(middle)),
                }
            }

            if !is_left_starting && !is_right_starting {
                let (left_index, _) = binary_search(&slice[0..middle], target);
                let (_, right_index) =
                    set_index(binary_search(&slice[middle + 1..], target), middle + 1);

                match (left_index, right_index) {
                    (Some(l), Some(r)) => return (Some(l), Some(r)),
                    _ => unreachable!(),
                }
            }

            unreachable!();
        }

        if middle_value > target {
            return binary_search(&slice[0..middle], target);
        }

        if middle_value < target {
            return set_index(binary_search(&slice[middle + 1..], target), middle + 1);
        }

        unreachable!();
    }

    fn find_middle(low: usize, high: usize) -> usize {
        if (low + high) % 2 == 0 {
            return (low + high) / 2;
        }

        return (low + high + 1) / 2;
    }

    fn set_index(
        indexes: (Option<usize>, Option<usize>),
        padding: usize,
    ) -> (Option<usize>, Option<usize>) {
        match indexes {
            (Some(l), Some(r)) => return (Some(l + padding), Some(r + padding)),
            (Some(l), None) => return (Some(l + padding), None),
            (None, Some(r)) => return (None, Some(r + padding)),
            (None, None) => return (None, None),
        };
    }
}

#[cfg(test)]
mod test {
    use crate::search_range;

    #[test]
    fn test_1() {
        let input = vec![5, 7, 7, 8, 8, 10];
        let target = 8;

        assert_eq!(search_range(input, target), vec![3, 4]);
    }

    #[test]
    fn test_2() {
        let input = vec![5, 7, 7, 8, 8, 10];
        let target = 6;

        assert_eq!(search_range(input, target), vec![-1, -1]);
    }

    #[test]
    fn test_3() {
        let input = vec![];
        let target = 0;

        assert_eq!(search_range(input, target), vec![-1, -1]);
    }

    #[test]
    fn test_4() {
        let input = vec![2, 2];
        let target = 2;

        assert_eq!(search_range(input, target), vec![0, 1]);
    }
}
