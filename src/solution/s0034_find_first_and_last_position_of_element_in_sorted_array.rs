/**
 * [34] Find First and Last Position of Element in Sorted Array
 *
 * Given an array of integers nums sorted in ascending order, find the starting and ending position of a given target value.
 *
 * Your algorithm's runtime complexity must be in the order of O(log n).
 *
 * If the target is not found in the array, return [-1, -1].
 *
 * Example 1:
 *
 *
 * Input: nums = [5,7,7,8,8,10], target = 8
 * Output: [3,4]
 *
 * Example 2:
 *
 *
 * Input: nums = [5,7,7,8,8,10], target = 6
 * Output: [-1,-1]
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
// discuss: https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let search = nums.binary_search(&target);
        if search.is_err() {
            return vec![-1, -1];
        }
        let search = search.unwrap();
        let mut left_most = Self::search_left_most(&nums, target, 0, search - 1);
        left_most = if left_most == -1 {
            search as i32
        } else {
            left_most
        };
        let mut right_most = Self::search_right_most(&nums, target, search + 1, nums.len() - 1);
        right_most = if right_most == -1 {
            search as i32
        } else {
            right_most
        };
        vec![left_most, right_most]
    }

    fn search_left_most(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> i32 {
        if left > right {
            return -1;
        }
        let search = nums[left..right + 1].binary_search(&target);
        if search.is_err() {
            return -1;
        }
        let search = search.unwrap() + left;
        let mut left_most = Self::search_left_most(nums, target, left, search - 1);
        if left_most == -1 {
            search as i32
        } else {
            left_most
        }
    }
    fn search_right_most(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> i32 {
        if left > right {
            return -1;
        }
        let search = nums[left..right + 1].binary_search(&target);
        if search.is_err() {
            return -1;
        }
        let search = search.unwrap() + left;
        let mut right_most = Self::search_right_most(nums, target, search + 1, right);
        if right_most == -1 {
            search as i32
        } else {
            right_most
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_34() {}
}
