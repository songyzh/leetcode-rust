/**
 * [476] Number Complement
 *
 * Given a positive integer, output its complement number. The complement strategy is to flip the bits of its binary representation.
 *
 * Note:<br>
 * <ol>
 * The given integer is guaranteed to fit within the range of a 32-bit signed integer.
 * You could assume no leading zero bit in the integer’s binary representation.
 * </ol>
 *
 *
 * Example 1:<br />
 *
 * Input: 5
 * Output: 2
 * Explanation: The binary representation of 5 is 101 (no leading zero bits), and its complement is 010. So you need to output 2.
 *
 *
 *
 * Example 2:<br />
 *
 * Input: 1
 * Output: 0
 * Explanation: The binary representation of 1 is 1 (no leading zero bits), and its complement is 0. So you need to output 0.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-complement/
// discuss: https://leetcode.com/problems/number-complement/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut num = num;
        let mut ret = 0;
        let mut offset = 0;
        while num != 0 {
            ret |= (1 - (num & 1)) << offset;
            num >>= 1;
            offset += 1;
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_476() {}
}
