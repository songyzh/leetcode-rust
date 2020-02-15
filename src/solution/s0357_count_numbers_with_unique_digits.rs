/**
 * [357] Count Numbers with Unique Digits
 *
 * Given a non-negative integer n, count all numbers with unique digits, x, where 0 &le; x < 10^n.
 *
 * <div>
 * Example:
 *
 *
 * Input: <span id="example-input-1-1">2</span>
 * Output: <span id="example-output-1">91
 * Explanation: </span>The answer should be the total numbers in the range of 0 &le; x < 100,
 *              excluding 11,22,33,44,55,66,77,88,99
 *
 * </div>
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 10;
        }
        if n == 2 {
            return 91;
        }
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 10;
        dp[2] = 91;
        for i in 3..(n + 1) {
            let mut curr = 9;
            let mut remain = 9;
            for _ in 0..(i - 1) {
                curr *= remain;
                remain -= 1;
            }
            dp[i as usize] = dp[i as usize - 1] + curr;
        }
        dp[n as usize]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_357() {}
}
