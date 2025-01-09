use std::cmp;
struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        let mut max_profit = 0;
        let mut min_price = prices[0];

        for &price in prices.iter() {
            min_price = cmp::min(price, min_price);
            max_profit = cmp::max(price - min_price, max_profit)
        }
        max_profit
    }
}
fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    Solution::max_profit(prices);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // Write your tests here
    #[test]
    fn test_case_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let result = 5;
        assert_eq!(result, Solution::max_profit(prices));
    }

    #[test]
    fn test_case_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let result = 0;
        assert_eq!(result, Solution::max_profit(prices));
    }
    #[test]
    fn test_case_3_empty_prices() {
        let prices = vec![];
        let result = 0;
        assert_eq!(result, Solution::max_profit(prices));
    }
}
