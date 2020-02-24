/**
 * [485] Max Consecutive Ones
 *
 * Given a binary array, find the maximum number of consecutive 1s in this array.
 *
 * Example 1:<br />
 *
 * Input: [1,1,0,1,1,1]
 * Output: 3
 * Explanation: The first two digits or the last three digits are consecutive 1s.
 *     The maximum number of consecutive 1s is 3.
 *
 *
 *
 * Note:
 *
 * The input array will only contain 0 and 1.
 * The length of input array is a positive integer and will not exceed 10,000
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-consecutive-ones/
// discuss: https://leetcode.com/problems/max-consecutive-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        let mut curr = 0;
        for num in nums {
            if num == 1 {
                curr += 1;
            } else {
                ret = ret.max(curr);
                curr = 0;
            }
        }
        ret.max(curr)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_485() {}
}
