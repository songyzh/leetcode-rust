/**
 * [395] Longest Substring with At Least K Repeating Characters
 *
 *
 * Find the length of the longest substring T of a given string (consists of lowercase letters only) such that every character in T appears no less than k times.
 *
 *
 * Example 1:
 *
 * Input:
 * s = "aaabb", k = 3
 *
 * Output:
 * 3
 *
 * The longest substring is "aaa", as 'a' is repeated 3 times.
 *
 *
 *
 * Example 2:
 *
 * Input:
 * s = "ababbc", k = 2
 *
 * Output:
 * 5
 *
 * The longest substring is "ababb", as 'a' is repeated 2 times and 'b' is repeated 3 times.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-with-at-least-k-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        if k < 2 {
            return s.len() as i32;
        }
        let chars: Vec<char> = s.chars().collect();
        Self::helper(&chars, 0, s.len(), k)
    }

    fn helper(chars: &Vec<char>, left: usize, right: usize, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for i in left..right {
            *map.entry(chars[i]).or_insert(0) += 1;
        }
        if map.is_empty() {
            return 0;
        }
        if *map.values().min().unwrap() >= k {
            return (right - left) as i32;
        }
        let mut start = left;
        let mut ret = 0;
        for i in left..right {
            if *map.get(&chars[i]).unwrap() < k {
                ret = ret.max(Self::helper(chars, start, i, k));
                start = i + 1;
            }
        }
        ret.max(Self::helper(chars, start, right, k))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_395() {
        println!("{}", Solution::longest_substring("weitong".to_string(), 3));
        println!("{}", Solution::longest_substring("ababbc".to_string(), 2));
    }
}
