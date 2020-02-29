/**
 * [39] Combination Sum
 *
 * Given a set of candidate numbers (candidates) (without duplicates) and a target number (target), find all unique combinations in candidates where the candidate numbers sums to target.
 *
 * The same repeated number may be chosen from candidates unlimited number of times.
 *
 * Note:
 *
 *
 * 	All numbers (including target) will be positive integers.
 * 	The solution set must not contain duplicate combinations.
 *
 *
 * Example 1:
 *
 *
 * Input: candidates = [2,3,6,7], target = 7,
 * A solution set is:
 * [
 *   [7],
 *   [2,2,3]
 * ]
 *
 *
 * Example 2:
 *
 *
 * Input: candidates = [2,3,5], target = 8,
 * A solution set is:
 * [
 *   [2,2,2,2],
 *   [2,3,3],
 *   [3,5]
 * ]
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/combination-sum/
// discuss: https://leetcode.com/problems/combination-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut seq = candidates;
        let mut res = Vec::new();
        seq.sort_unstable_by(|a, b| b.cmp(a));
        let mut vec = Vec::new();
        Solution::backtrack(&seq, target, vec, &mut res, 0);
        res
    }

    fn backtrack(
        seq: &Vec<i32>,
        target: i32,
        mut curr: Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        start_idx: usize,
    ) {
        for i in start_idx..seq.len() {
            let item = seq[i];
            if target - item < 0 {
                continue;
            }
            let mut new_vec = curr.clone();
            new_vec.push(item);
            if target == item {
                result.push(new_vec);
            } else {
                Solution::backtrack(seq, target - item, new_vec, result, i);
            }
        }
    }

    pub fn combination_sum_1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        Self::helper(&candidates, target, &mut vec![], 0, &mut ret);
        ret
    }

    fn helper(
        candidates: &Vec<i32>,
        target: i32,
        curr: &mut Vec<i32>,
        start_index: usize,
        ret: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            ret.push(curr.clone());
            return;
        }
        if target < 0 {
            return;
        }
        for i in start_index..candidates.len() {
            curr.push(candidates[i]);
            Self::helper(candidates, target - candidates[i], curr, i, ret);
            curr.pop();
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_39() {
        assert_eq!(
            Solution::combination_sum(vec![1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![7], vec![3, 2, 2],]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![5, 3], vec![3, 3, 2], vec![2, 2, 2, 2],]
        );
    }
}
