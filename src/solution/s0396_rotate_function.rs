/**
 * [396] Rotate Function
 *
 *
 * Given an array of integers A and let n to be its length.
 *
 *
 *
 * Assume Bk to be an array obtained by rotating the array A k positions clock-wise, we define a "rotation function" F on A as follow:
 *
 *
 *
 * F(k) = 0 * Bk[0] + 1 * Bk[1] + ... + (n-1) * Bk[n-1].
 *
 * Calculate the maximum value of F(0), F(1), ..., F(n-1).
 *
 *
 * Note:<br />
 * n is guaranteed to be less than 10^5.
 *
 *
 * Example:
 *
 * A = [4, 3, 2, 6]
 *
 * F(0) = (0 * 4) + (1 * 3) + (2 * 2) + (3 * 6) = 0 + 3 + 4 + 18 = 25
 * F(1) = (0 * 6) + (1 * 4) + (2 * 3) + (3 * 2) = 0 + 4 + 6 + 6 = 16
 * F(2) = (0 * 2) + (1 * 6) + (2 * 4) + (3 * 3) = 0 + 6 + 8 + 9 = 23
 * F(3) = (0 * 3) + (1 * 2) + (2 * 6) + (3 * 4) = 0 + 2 + 12 + 12 = 26
 *
 * So the maximum value of F(0), F(1), F(2), F(3) is F(3) = 26.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotate-function/
// discuss: https://leetcode.com/problems/rotate-function/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_rotate_function(a: Vec<i32>) -> i32 {
        //        0 * a0 + 1 * a_1 + .. + (n - 1) * a_(n - 1) = t0
        //        1 * a0 + 2 * a_1 + .. + 0 * a_(n - 1) = t0 + sum - n * a_(n - 1)
        //        (n - 1) * a0 + 0 * a_1 + .. + (n - 2) * a_(n - 1) = t_p + sum - n * a_1
        let n = a.len();
        let mut sum = 0;
        let mut curr = 0;
        for (i, num) in a.iter().enumerate() {
            let i = i as i32;
            sum += *num;
            curr += i * (*num);
        }
        let mut ret = curr;
        for i in (1..n).rev() {
            curr = curr + sum - n as i32 * a[i];
            ret = ret.max(curr);
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_396() {
        println!("{}", Solution::max_rotate_function(vec![4, 3, 2, 6]));
    }
}
