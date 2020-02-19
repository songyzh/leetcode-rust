/**
 * [386] Lexicographical Numbers
 *
 * Given an integer n, return 1 - n in lexicographical order.
 *
 * For example, given 13, return: [1,10,11,12,13,2,3,4,5,6,7,8,9].
 *
 * Please optimize your algorithm to use less time and space. The input size may be as large as 5,000,000.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/lexicographical-numbers/
// discuss: https://leetcode.com/problems/lexicographical-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ret = Vec::with_capacity(n as usize);
        for i in 1..10 {
            Self::helper(i, &mut ret);
        }
        ret
    }

    fn helper(curr: i32, ret: &mut Vec<i32>) {
        if curr > ret.capacity() as i32 {
            return;
        }
        ret.push(curr);
        for i in 0..10 {
            Self::helper(curr * 10 + i, ret);
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_386() {
        println!("{:?}", Solution::lexical_order(13));
    }
}
