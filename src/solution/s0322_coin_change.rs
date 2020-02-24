/**
 * [322] Coin Change
 *
 * You are given coins of different denominations and a total amount of money amount. Write a function to compute the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
 *
 * Example 1:
 *
 *
 * Input: coins = [1, 2, 5], amount = 11
 * Output: 3
 * Explanation: 11 = 5 + 5 + 1
 *
 * Example 2:
 *
 *
 * Input: coins = [2], amount = 3
 * Output: -1
 *
 *
 * Note:<br />
 * You may assume that you have an infinite number of each kind of coin.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/coin-change/
// discuss: https://leetcode.com/problems/coin-change/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;
        for a in 1..(amount + 1) {
            for coin in &coins {
                if a >= *coin {
                    dp[a as usize] = dp[a as usize].min(dp[(a - *coin) as usize] + 1);
                }
            }
        }
        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_322() {}
}
