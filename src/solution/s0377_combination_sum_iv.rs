/**
 * [377] Combination Sum IV
 *
 * Given an integer array with all positive numbers and no duplicates, find the number of possible combinations that add up to a positive integer target.
 *
 * Example:
 *
 *
 * nums = [1, 2, 3]
 * target = 4
 *
 * The possible combination ways are:
 * (1, 1, 1, 1)
 * (1, 1, 2)
 * (1, 2, 1)
 * (1, 3)
 * (2, 1, 1)
 * (2, 2)
 * (3, 1)
 *
 * Note that different sequences are counted as different combinations.
 *
 * Therefore the output is 7.
 *
 *
 *
 *
 * Follow up:<br />
 * What if negative numbers are allowed in the given array?<br />
 * How does it change the problem?<br />
 * What limitation we need to add to the question to allow negative numbers?
 *
 * Credits:<br />
 * Special thanks to <a href="https://leetcode.com/pbrother/">@pbrother</a> for adding this problem and creating all test cases.
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        for t in 1..target + 1 {
            for num in &nums {
                let num = *num as usize;
                if num == t {
                    dp[t] += 1;
                } else if num < t {
                    dp[t] += dp[t - num];
                }
            }
        }
        dp[target]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_377() {
        println!("{:?}", Solution::combination_sum4(vec![1, 2, 3], 4));
    }
}
