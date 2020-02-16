/**
 * [394] Decode String
 *
 * Given an encoded string, return its decoded string.
 * The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.
 * You may assume that the input string is always valid; No extra white spaces, square brackets are well-formed, etc.
 * Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there won't be input like 3a or 2[4].
 * Examples:
 *
 * s = "3[a]2[bc]", return "aaabcbc".
 * s = "3[a2[c]]", return "accaccacc".
 * s = "2[abc]3[cd]ef", return "abcabccdcdcdef".
 *
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            if c != ']' {
                stack.push(c);
            } else {
                println!("{:?}", stack);
                let mut pattern = String::new();
                while *stack.last().unwrap() != '[' {
                    pattern.push(stack.pop().unwrap());
                }
                pattern = pattern.chars().rev().collect();
                stack.pop();
                let mut count = String::new();
                while !stack.is_empty() && stack.last().unwrap().is_numeric() {
                    count.push(stack.pop().unwrap());
                }
                count = count.chars().rev().collect();
                let mut curr = String::new();
                for _ in 0..count.parse().unwrap() {
                    curr.push_str(&pattern);
                }
                stack.append(&mut curr.chars().collect());
            }
        }
        stack.iter().collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_394() {
        println!(
            "{}",
            Solution::decode_string(String::from("3[z]2[2[y]pq4[2[jk]e1[f]]]ef"))
        );
    }
}
