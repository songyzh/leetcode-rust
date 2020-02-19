/**
 * [500] Keyboard Row
 *
 * Given a List of words, return the words that can be typed using letters of alphabet on only one row's of American keyboard like the image below.
 *
 *
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/10/12/keyboard.png" style="width: 100%; max-width: 600px" />
 *
 *
 * Example:
 *
 *
 * Input: ["Hello", "Alaska", "Dad", "Peace"]
 * Output: ["Alaska", "Dad"]
 *
 *
 *
 *
 * Note:
 *
 * <ol>
 * 	You may use one character in the keyboard more than once.
 * 	You may assume the input string will only contain letters of alphabet.
 * </ol>
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/keyboard-row/
// discuss: https://leetcode.com/problems/keyboard-row/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        "qwertyuiop".chars().map(|c| map.insert(c, 1)).count();
        "asdfghjkl".chars().map(|c| map.insert(c, 2)).count();
        "zxcvbnm".chars().map(|c| map.insert(c, 3)).count();
        words
            .into_iter()
            .filter(|word| Self::helper(&map, &word.to_ascii_lowercase()))
            .collect()
    }

    fn helper(map: &HashMap<char, i32>, word: &String) -> bool {
        for c in word.chars() {
            if map.get(&c).unwrap() != map.get(&word.chars().nth(0usize).unwrap()).unwrap() {
                return false;
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_500() {}
}
