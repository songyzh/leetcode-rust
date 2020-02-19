/**
 * [447] Number of Boomerangs
 *
 * Given n points in the plane that are all pairwise distinct, a "boomerang" is a tuple of points (i, j, k) such that the distance between i and j equals the distance between i and k (the order of the tuple matters).
 *
 * Find the number of boomerangs. You may assume that n will be at most 500 and coordinates of points are all in the range [-10000, 10000] (inclusive).
 *
 * Example:
 *
 *
 * Input:
 * [[0,0],[1,0],[2,0]]
 *
 * Output:
 * 2
 *
 * Explanation:
 * The two boomerangs are [[1,0],[0,0],[2,0]] and [[1,0],[2,0],[0,0]]
 *
 *
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-boomerangs/
// discuss: https://leetcode.com/problems/number-of-boomerangs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut ret = 0;
        for (i, i_point) in points.iter().enumerate() {
            let mut lookup = HashMap::new();
            for (j, j_point) in points.iter().enumerate() {
                if i == j {
                    continue;
                }
                *lookup
                    .entry(Self::calc_distance(i_point, j_point))
                    .or_insert(0) += 1;
            }
            for value in lookup.values() {
                ret += value * (value - 1);
            }
        }
        ret
    }

    fn calc_distance(i_point: &Vec<i32>, j_point: &Vec<i32>) -> i32 {
        (i_point[0] - j_point[0]).pow(2) + (i_point[1] - j_point[1]).pow(2)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_447() {}
}
