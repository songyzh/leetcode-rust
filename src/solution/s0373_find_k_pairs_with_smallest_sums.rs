/**
 * [373] Find K Pairs with Smallest Sums
 *
 * You are given two integer arrays nums1 and nums2 sorted in ascending order and an integer k.
 *
 * Define a pair (u,v) which consists of one element from the first array and one element from the second array.
 *
 * Find the k pairs (u1,v1),(u2,v2) ...(uk,vk) with the smallest sums.
 *
 * Example 1:
 *
 *
 * Input: nums1 = <span id="example-input-1-1">[1,7,11]</span>, nums2 = <span id="example-input-1-2">[2,4,6]</span>, k = <span id="example-input-1-3">3</span>
 * Output: <span id="example-output-1">[[1,2],[1,4],[1,6]]
 * Explanation: </span>The first 3 pairs are returned from the sequence:
 *              [1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
 *
 * Example 2:
 *
 *
 * Input: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
 * Output: [1,1],[1,1]<span>
 * Explanation: </span>The first 2 pairs are returned from the sequence:
 *              [1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
 *
 * Example 3:
 *
 *
 * Input: nums1 = [1,2], nums2 = [3], k = 3
 * Output: [1,3],[2,3]<span>
 * Explanation: </span>All possible pairs are returned from the sequence: [1,3],[2,3]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-k-pairs-with-smallest-sums/
// discuss: https://leetcode.com/problems/find-k-pairs-with-smallest-sums/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        // use a max heap
        use std::collections::BinaryHeap;
        let k = k as usize;
        let mut max_heap = BinaryHeap::with_capacity(k);
        for num1 in &nums1 {
            for num2 in &nums2 {
                let sum = *num1 + *num2;
                if max_heap.len() < k {
                    max_heap.push(sum);
                } else {
                    let mut top = max_heap.peek_mut().unwrap();
                    if sum < *top {
                        *top = sum;
                    }
                }
            }
        }
        let mut ret = vec![];
        let top = max_heap.peek();
        if top.is_none() {
            return ret;
        }
        let top = *top.unwrap();
        for num1 in &nums1 {
            for num2 in &nums2 {
                let sum = *num1 + *num2;
                if sum <= top {
                    ret.push(vec![*num1, *num2]);
                }
            }
        }
        ret.sort_by_key(|v| v[0] + v[1]);
        if ret.len() > k {
            ret.split_off(k);
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_373() {}
}
