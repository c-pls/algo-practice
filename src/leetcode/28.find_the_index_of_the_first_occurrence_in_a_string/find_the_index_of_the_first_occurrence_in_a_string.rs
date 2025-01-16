struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        match haystack.find(&needle) {
           Some(idx)  => idx as i32,
           None => -1
        }
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::Solution;

    // Write your tests here
    #[test]
    fn test_case_1() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");

        let res = Solution::str_str(haystack, needle);

        assert_eq!(res, 0);
    }

    #[test]
    fn test_case_2() {
        let haystack = String::from("leetcode");
        let needle = String::from("leeto");

        let res = Solution::str_str(haystack, needle);

        assert_eq!(res, -1);
    }

    #[test]
    fn test_case_3() {
        let haystack = String::from("adbutsad");
        let needle = String::from("sad");

        let res = Solution::str_str(haystack, needle);

        assert_eq!(res, 5);
    }

    #[test]
    fn test_case_4() {
        let haystack = String::from("a");
        let needle = String::from("a");

        let res = Solution::str_str(haystack, needle);

        assert_eq!(res, 0);
    }
}
