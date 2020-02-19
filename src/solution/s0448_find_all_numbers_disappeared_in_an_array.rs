/**
 * [448] Find All Numbers Disappeared in an Array
 *
 * Given an array of integers where 1 &le; a[i] &le; n (n = size of array), some elements appear twice and others appear once.
 *
 * Find all the elements of [1, n] inclusive that do not appear in this array.
 *
 * Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.
 *
 * Example:
 *
 * Input:
 * [4,3,2,7,8,2,3,1]
 *
 * Output:
 * [5,6]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
// discuss: https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        // not best
        let mut ret = vec![];
        let mut nums = nums;
        nums.sort();
        let mut should_be = 1;
        let mut index = 0;
        while index < nums.len() {
            if should_be == nums[index] {
                should_be += 1;
                index += 1;
            } else if should_be > nums[index] {
                index += 1;
            } else {
                ret.push(should_be);
                should_be += 1;
            }
        }
        for i in should_be..(nums.len() as i32 + 1) {
            ret.push(i);
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_448() {}
}
