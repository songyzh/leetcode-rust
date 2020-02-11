/**
 * [453] Minimum Moves to Equal Array Elements
 *
 * Given a non-empty integer array of size n, find the minimum number of moves required to make all array elements equal, where a move is incrementing n - 1 elements by 1.
 *
 * Example:
 *
 * Input:
 * [1,2,3]
 *
 * Output:
 * 3
 *
 * Explanation:
 * Only three moves are needed (remember each move increments two elements):
 *
 * [1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        // all about math
        // x * (n-1) + sum = n * (min + x)
        // xn - x + sum = min*n + nx
        // x = sum - min * n
        let sum: i32 = nums.iter().sum();
        sum - *nums.iter().min().unwrap() * nums.len() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_453() {}
}
