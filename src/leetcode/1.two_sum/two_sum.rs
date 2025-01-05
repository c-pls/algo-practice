use std::{collections::HashMap, vec};

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m: HashMap<i32, usize> = HashMap::new();
        for (idx, &num) in nums.iter().enumerate() {
            match m.get(&(target - num)) {
                Some(&i) => {
                    let mut res = vec![idx as i32, i as i32];
                    res.sort();
                    return res;
                }
                _ => {
                    m.insert(num, idx);
                }
            }
        }

        Vec::new()
    }

    pub fn two_sum_for_loop(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let length = nums.len();

        for i in 0..length {
            let curr = nums[i];
            for j in i + 1..length {
                if curr + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution; 

    #[test]
    fn test_two_sum_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]); // nums[0] + nums[1] = 9
    }

    #[test]
    fn test_two_sum_case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]); // nums[1] + nums[2] = 6
    }

    #[test]
    fn test_two_sum_case_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]); // nums[0] + nums[1] = 6
    }

    #[test]
    fn test_two_sum_no_solution() {
        let nums = vec![1, 2, 3];
        let target = 7;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![]); // No solution exists
    }
}
