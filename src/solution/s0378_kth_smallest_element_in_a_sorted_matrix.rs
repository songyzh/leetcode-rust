/**
 * [378] Kth Smallest Element in a Sorted Matrix
 *
 * Given a n x n matrix where each of the rows and columns are sorted in ascending order, find the kth smallest element in the matrix.
 *
 *
 * Note that it is the kth smallest element in the sorted order, not the kth distinct element.
 *
 *
 * Example:
 *
 * matrix = [
 *    [ 1,  5,  9],
 *    [10, 11, 13],
 *    [12, 13, 15]
 * ],
 * k = 8,
 *
 * return 13.
 *
 *
 *
 * Note: <br>
 * You may assume k is always valid, 1 &le; k &le; n^2.
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/
// discuss: https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        // use max heap; TODO: improve
        let k = k as usize;
        let mut heap = BinaryHeap::new();
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                let curr = matrix[i][j];
                if heap.len() < k {
                    heap.push(curr);
                } else {
                    let mut top = heap.peek_mut().unwrap();
                    if curr < *top {
                        *top = curr;
                    }
                }
            }
        }
        heap.pop().unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_378() {
        println!(
            "{}",
            Solution::kth_smallest(vec![vec![1, 5, 9], vec![10, 11, 13], vec![12, 13, 15]], 8)
        )
    }
}
