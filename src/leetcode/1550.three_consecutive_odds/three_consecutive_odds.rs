struct Solution;
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;

        for i in arr {
            if i % 2 != 0 {
                count += 1;
                if count == 3 {
                    return true;
                }
            } else {
                count = 0;
            }
        }

        false
    }
}
fn main() {
    let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];

    let res = Solution::three_consecutive_odds(arr);
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // Write your tests here
    #[test]
    fn test_case_1() {
        let arr = vec![2,6,4,1];
        let res = Solution::three_consecutive_odds(arr);
        assert_eq!(res, false);
    }
}
