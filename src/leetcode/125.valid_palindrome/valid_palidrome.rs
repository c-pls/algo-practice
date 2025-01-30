struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cleaned_str: String = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .flat_map(|c| c.to_lowercase())
            .collect();
        println!("{}", cleaned_str);

        let length = cleaned_str.len();

        let byte_str = cleaned_str.as_bytes();

        for left in 0..length / 2 {
            if byte_str[left] != byte_str[length - 1 - left] {
                return false;
            }
        }
        true
    }
}

fn main() {
    let s = String::from("A man, a plan, a canal: Panama");
    let x = Solution::is_palindrome(s);
    println!("{}", x);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // Write your tests here
    #[test]
    fn test_case_1() {
        let s = String::from("A man, a plan, a canal: Panama");

        assert_eq!(Solution::is_palindrome(s), true);
    }
    #[test]
    fn test_case_2() {
        let s = String::from("race a car");

        assert_eq!(Solution::is_palindrome(s), false);
    }
    #[test]
    fn test_case_3() {
        let s = String::from(" ");

        assert_eq!(Solution::is_palindrome(s), true);
    }
}
