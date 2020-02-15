/**
 * [368] Largest Divisible Subset
 *
 * Given a set of distinct positive integers, find the largest subset such that every pair (Si, Sj) of elements in this subset satisfies:
 *
 * Si % Sj = 0 or Sj % Si = 0.
 *
 * If there are multiple solutions, return any subset is fine.
 *
 * Example 1:
 *
 * <div>
 *
 * Input: <span id="example-input-1-1">[1,2,3]</span>
 * Output: <span id="example-output-1">[1,2] </span>(of course, [1,3] will also be ok)
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: <span id="example-input-2-1">[1,2,4,8]</span>
 * Output: <span id="example-output-2">[1,2,4,8]</span>
 *
 * </div>
 * </div>
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut prev_indices: Vec<i32> = vec![-1; nums.len()];
        let mut dp = vec![1; nums.len()];
        let mut max_count = 0;
        let mut max_index: i32 = -1;
        for i in 0..nums.len() {
            for j in (0..i).rev() {
                if nums[i] % nums[j] == 0 && dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    prev_indices[i] = j as i32;
                }
            }
            if dp[i] > max_count {
                max_count = dp[i];
                max_index = i as i32;
            }
        }
        let mut ret = vec![];
        while max_index != -1 {
            ret.push(nums[max_index as usize]);
            max_index = prev_indices[max_index as usize];
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_368() {
        println!("{:?}", Solution::largest_divisible_subset(vec![1, 2, 3]));
    }
}
