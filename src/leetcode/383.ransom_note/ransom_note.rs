use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_counter: HashMap<char, i32> = HashMap::new();

        magazine
            .chars()
            .for_each(|c| *magazine_counter.entry(c).or_insert(0) += 1);
        println!("{:?}", magazine);

        for c in ransom_note.chars() {
            let count = magazine_counter.entry(c).or_insert(0);
            if *count == 0 {
                return false;
            }
            *count -= 1
        }
        true
    }
}

fn main() {
    let ransom_note = "aa";
    let magazine = "aab";

    Solution::can_construct(ransom_note.to_string(), magazine.to_string());
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // Write your tests here
    #[test]
    fn test_case_1() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");

        let res = Solution::can_construct(ransom_note, magazine);

        assert_eq!(res, false)
    }

    #[test]
    fn test_case_2() {
        let ransom_note = String::from("aa");
        let magazine = String::from("ab");

        let res = Solution::can_construct(ransom_note, magazine);

        assert_eq!(res, false)
    }
    #[test]
    fn test_case_3() {
        let ransom_note = String::from("aa");
        let magazine = String::from("aab");

        let res = Solution::can_construct(ransom_note, magazine);

        assert_eq!(res, true)
    }
}
