/**
 * [29] Divide Two Integers
 *
 * Given two integers dividend and divisor, divide two integers without using multiplication, division and mod operator.
 *
 * Return the quotient after dividing dividend by divisor.
 *
 * The integer division should truncate toward zero.
 *
 * Example 1:
 *
 *
 * Input: dividend = 10, divisor = 3
 * Output: 3
 *
 * Example 2:
 *
 *
 * Input: dividend = 7, divisor = -3
 * Output: -2
 *
 * Note:
 *
 *
 * 	Both dividend and divisor will be 32-bit signed integers.
 * 	The divisor will never be 0.
 * 	Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [-2^31,  2^31 - 1]. For the purpose of this problem, assume that your function returns 2^31 - 1 when the division result overflows.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-two-integers/
// discuss: https://leetcode.com/problems/divide-two-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

use std::i32::MAX;
use std::i32::MIN;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut dividend = dividend;
        let mut divisor = divisor;
        if divisor == 1 {
            return dividend;
        }
        // in case divisor == MIN
        if divisor == MIN {
            if dividend == MIN {
                return 1;
            }
            return 0;
        }
        // in case dividend == MIN
        if dividend == MIN && divisor == -1 {
            return MAX;
        }
        let mut borrow = 0;
        if dividend == MIN {
            if divisor > 0 {
                borrow = -1;
                dividend += divisor;
            } else {
                borrow = 1;
                dividend -= divisor;
            }
        }
        let mut ret = 0;
        let mut flag = 1;
        if dividend > 0 && divisor < 0 || dividend < 0 && divisor > 0 {
            flag = -1;
        }
        dividend = dividend.abs();
        divisor = divisor.abs();
        while dividend >= divisor {
            let mut curr_divisor = divisor;
            let mut offset = 0;
            while curr_divisor << offset > 0 && dividend >= curr_divisor << offset {
                ret += 1 << offset;
                dividend -= curr_divisor << offset;
                offset += 1;
            }
        }
        ret * flag + borrow
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_29() {}
}
