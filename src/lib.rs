#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;

        for i in 0..(prices.len() - 1) {
            let cur_day_price = prices[i];
            let next_day_price = prices.get(i + 1);

            if let Some(next_day_price) = next_day_price {
                if *next_day_price > cur_day_price {
                    profit += *next_day_price - cur_day_price;
                }
            }
        }
        return profit;
    }
}
#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);

        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);

        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
