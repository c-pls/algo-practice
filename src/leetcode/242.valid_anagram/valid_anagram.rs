struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_counter: HashMap<char, i32> = HashMap::new();

        s.chars()
            .for_each(|c| *s_counter.entry(c).or_insert(0) += 1);

        for c in t.chars() {
            let count = s_counter.entry(c).or_insert(0);

            if *count == 0 {
                return false;
            }
            *count -= 1
        }

        s_counter.values().all(|&v| v == 0)
    }

    pub fn is_anagram_sort(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();

        s_chars.sort();
        t_chars.sort();

        s_chars == t_chars
    }
}

fn main() {
    let s = String::from("anagram");
    let t = String::from("nagaram");

    Solution::is_anagram_sort(s, t);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // Write your tests here
    #[test]
    fn test_case_1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");

        assert_eq!(Solution::is_anagram_sort(s, t), true);
    }
    #[test]
    fn test_case_2() {
        let s = String::from("car");
        let t = String::from("rat");

        assert_eq!(Solution::is_anagram_sort(s, t), false);
    }

    #[test]
    fn test_case_3() {
        let s = String::from("");
        let t = String::from("");
        assert_eq!(Solution::is_anagram_sort(s, t), true);
    }

    #[test]
    fn test_case_4() {
        let s = String::from("");
        let t = String::from("a");
        assert_eq!(Solution::is_anagram_sort(s, t), false);
    }

    #[test]
    fn test_case_5() {
        let s = String::from("abc");
        let t = String::from("abcd");
        assert_eq!(Solution::is_anagram_sort(s, t), false);
    }

    #[test]
    fn test_case_6() {
        let s = String::from("Anagram");
        let t = String::from("nagaram");
        assert_eq!(Solution::is_anagram_sort(s, t), false);
    }

    #[test]
    fn test_case_7() {
        let s = String::from("listen");
        let t = String::from("listen");
        assert_eq!(Solution::is_anagram_sort(s, t), true);
    }

    #[test]
    fn test_case_8() {
        let s = String::from("aabbcc");
        let t = String::from("aaabbc");
        assert_eq!(Solution::is_anagram_sort(s, t), false);
    }

    #[test]
    fn test_case_9() {
        let s = String::from("abcdef");
        let t = String::from("fedcba");
        assert_eq!(Solution::is_anagram_sort(s, t), true);
    }

    #[test]
    fn test_case_10() {
        let s = String::from("aa");
        let t = String::from("a");
        assert_eq!(Solution::is_anagram_sort(s, t), false);
    }
}
