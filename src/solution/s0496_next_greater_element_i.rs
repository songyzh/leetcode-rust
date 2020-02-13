/**
 * [496] Next Greater Element I
 *
 *
 * You are given two arrays (without duplicates) nums1 and nums2 where nums1’s elements are subset of nums2. Find all the next greater numbers for nums1's elements in the corresponding places of nums2.
 *
 *
 *
 * The Next Greater Number of a number x in nums1 is the first greater number to its right in nums2. If it does not exist, output -1 for this number.
 *
 *
 * Example 1:<br />
 *
 * Input: nums1 = [4,1,2], nums2 = [1,3,4,2].
 * Output: [-1,3,-1]
 * Explanation:
 *     For number 4 in the first array, you cannot find the next greater number for it in the second array, so output -1.
 *     For number 1 in the first array, the next greater number for it in the second array is 3.
 *     For number 2 in the first array, there is no next greater number for it in the second array, so output -1.
 *
 *
 *
 * Example 2:<br />
 *
 * Input: nums1 = [2,4], nums2 = [1,2,3,4].
 * Output: [3,-1]
 * Explanation:
 *     For number 2 in the first array, the next greater number for it in the second array is 3.
 *     For number 4 in the first array, there is no next greater number for it in the second array, so output -1.
 *
 *
 *
 *
 * Note:<br>
 * <ol>
 * All elements in nums1 and nums2 are unique.
 * The length of both nums1 and nums2 would not exceed 1000.
 * </ol>
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut stack = vec![];
        let mut map = HashMap::new();
        // for every number in the pool, find the next greater number
        for num2 in nums2 {
            while stack.last().is_some() && *stack.last().unwrap() < num2 {
                map.insert(stack.pop().unwrap(), num2);
            }
            stack.push(num2);
        }
        nums1
            .iter()
            .map(|num1| *map.get(num1).unwrap_or(&-1))
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_496() {
        println!(
            "{:?}",
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
        )
    }
}
