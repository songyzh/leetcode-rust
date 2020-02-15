/**
 * [347] Top K Frequent Elements
 *
 * Given a non-empty array of integers, return the k most frequent elements.
 *
 * Example 1:
 *
 *
 * Input: nums = <span id="example-input-1-1">[1,1,1,2,2,3]</span>, k = <span id="example-input-1-2">2</span>
 * Output: <span id="example-output-1">[1,2]</span>
 *
 *
 * <div>
 * Example 2:
 *
 *
 * Input: nums = <span id="example-input-2-1">[1]</span>, k = <span id="example-input-2-2">1</span>
 * Output: <span id="example-output-2">[1]</span>
 * </div>
 *
 * Note:
 *
 *
 * 	You may assume k is always valid, 1 &le; k &le; number of unique elements.
 * 	Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
 *
 *
 */
pub struct Solution {}

// submission codes start here

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        nums.iter()
            .map(|num| *map.entry(*num).or_insert(0) += 1)
            .count();
        let mut nums: Vec<i32> = map.keys().map(|num| *num).collect();
        nums.sort_by_key(|num| -*map.get(num).unwrap());
        nums[0..k as usize].to_vec()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_347() {}
}
