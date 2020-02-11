/**
 * [459] Repeated Substring Pattern
 *
 * Given a non-empty string check if it can be constructed by taking a substring of it and appending multiple copies of the substring together. You may assume the given string consists of lowercase English letters only and its length will not exceed 10000.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: "abab"
 * Output: True
 * Explanation: It's the substring "ab" twice.
 *
 *
 * Example 2:
 *
 *
 * Input: "aba"
 * Output: False
 *
 *
 * Example 3:
 *
 *
 * Input: "abcabcabcabc"
 * Output: True
 * Explanation: It's the substring "abc" four times. (And the substring "abcabc" twice.)
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut pattern_len = s.len() / 2;
        while pattern_len >= 1 {
            if s.len() % pattern_len == 0 {
                let substring = &s[0..pattern_len];
                let mut start = 0;
                while start + pattern_len <= s.len() {
                    if &s[start..start + pattern_len] != substring {
                        break;
                    }
                    start += pattern_len;
                }
                if start == s.len() {
                    return true;
                }
            }
            pattern_len -= 1;
        }
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_459() {}
}
