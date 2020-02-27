/**
 * [400] Nth Digit
 *
 * Find the n^th digit of the infinite integer sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...
 *
 * Note:<br />
 * n is positive and will fit within the range of a 32-bit signed integer (n < 2^31).
 *
 *
 * Example 1:
 *
 * Input:
 * 3
 *
 * Output:
 * 3
 *
 *
 *
 * Example 2:
 *
 * Input:
 * 11
 *
 * Output:
 * 0
 *
 * Explanation:
 * The 11th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/nth-digit/
// discuss: https://leetcode.com/problems/nth-digit/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        // 1-9 => 9 => 9
        // 10-99 => 90 => 180
        // 100-999 => 900 => 2700
        let mut n = n as i64 - 1;
        let mut single_width = 1;
        let mut start = 1;
        let mut end = start * 10 - 1;
        let mut range_width: i64 = (end - start + 1) * single_width;
        while n >= range_width {
            n -= range_width;
            start = end + 1;
            end = start * 10 - 1;
            single_width += 1;
            range_width = (end - start + 1) * single_width;
        }
        let nums = n / single_width;
        start += nums;
        n -= nums * single_width;
        start.to_string().chars().collect::<Vec<char>>()[n as usize]
            .to_digit(10)
            .unwrap() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_400() {
        println!("{}", Solution::find_nth_digit(1000000000));
    }
}
