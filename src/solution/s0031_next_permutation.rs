/**
 * [31] Next Permutation
 *
 * Implement next permutation, which rearranges numbers into the lexicographically next greater permutation of numbers.
 *
 * If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
 *
 * The replacement must be <a href="http://en.wikipedia.org/wiki/In-place_algorithm" target="_blank">in-place</a> and use only constant extra memory.
 *
 * Here are some examples. Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
 *
 * 1,2,3 &rarr; 1,3,2<br />
 * 3,2,1 &rarr; 1,2,3<br />
 * 1,1,5 &rarr; 1,5,1
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/next-permutation/
// discuss: https://leetcode.com/problems/next-permutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let mut change_index = None;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] < nums[i + 1] {
                change_index = Some(i);
                break;
            }
        }
        if change_index.is_none() {
            nums.sort_unstable();
            return;
        }
        let change_index = change_index.unwrap();
        let mut successor_index = None;
        for i in (change_index + 1..nums.len()).rev() {
            if nums[i] > nums[change_index] {
                successor_index = Some(i);
                break;
            }
        }
        let successor_index = successor_index.unwrap();
        nums.swap(change_index, successor_index);
        let len = nums.len();
        nums[change_index + 1..len].sort_unstable()
    }
}

// submission codes end

/*
// a clean solution (from leetcode submissions)
impl Solution {
    pub fn next_permutation(a: &mut Vec<i32>) {
        let n = a.len();

        if let Some(i) = (1..n).rev().find(|&i| a[i - 1] < a[i]) {
            let j = (i..n).rev().find(|&j| a[i - 1] < a[j])
                .unwrap();

            a.swap(i - 1, j);
            a[i..].reverse();
        } else {
            a.reverse();
        }
    }
}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_31() {
        let mut vec1 = vec![1, 2, 3, 4, 5];
        Solution::next_permutation(&mut vec1);
        assert_eq!(vec1, vec![1, 2, 3, 5, 4]);

        let mut vec2 = vec![5, 4, 3, 2, 1];
        Solution::next_permutation(&mut vec2);
        assert_eq!(vec2, vec![1, 2, 3, 4, 5]);
    }
}
