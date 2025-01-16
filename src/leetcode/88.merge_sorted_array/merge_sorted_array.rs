struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = m + n - 1;

        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1
        }

        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }

    pub fn merge_extra_space(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
        let mut combine_nums = nums1[0..m as usize].to_vec();

        combine_nums.extend(nums2.iter());

        combine_nums.sort();

        nums1.clone_from_slice(&combine_nums);

        println!("{:?}", nums1);
    }
}
fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = 3;

    Solution::merge_extra_space(&mut nums1, m, &mut nums2, n);

    println!("{:?}", nums1)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Solution;

    // Write your tests here
    #[test]
    fn test_case_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6])
    }

    #[test]
    fn test_case_2() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2: Vec<i32> = vec![];
        let n = 0;
        Solution::merge(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_case_1_extra_space() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        Solution::merge_extra_space(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6])
    }

    #[test]
    fn test_case_2_extra_space() {
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2: Vec<i32> = vec![];
        let n = 0;
        Solution::merge_extra_space(&mut nums1, m, &mut nums2, n);

        assert_eq!(nums1, vec![1]);
    }
}
