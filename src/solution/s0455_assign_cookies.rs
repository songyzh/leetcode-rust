/**
 * [455] Assign Cookies
 *
 *
 * Assume you are an awesome parent and want to give your children some cookies. But, you should give each child at most one cookie. Each child i has a greed factor gi, which is the minimum size of a cookie that the child will be content with; and each cookie j has a size sj. If sj >= gi, we can assign the cookie j to the child i, and the child i will be content. Your goal is to maximize the number of your content children and output the maximum number.
 *
 *
 * Note:<br />
 * You may assume the greed factor is always positive. <br />
 * You cannot assign more than one cookie to one child.
 *
 *
 * Example 1:<br />
 *
 * Input: [1,2,3], [1,1]
 *
 * Output: 1
 *
 * Explanation: You have 3 children and 2 cookies. The greed factors of 3 children are 1, 2, 3.
 * And even though you have 2 cookies, since their size is both 1, you could only make the child whose greed factor is 1 content.
 * You need to output 1.
 *
 *
 *
 * Example 2:<br />
 *
 * Input: [1,2], [1,2,3]
 *
 * Output: 2
 *
 * Explanation: You have 2 children and 3 cookies. The greed factors of 2 children are 1, 2.
 * You have 3 cookies and their sizes are big enough to gratify all of the children,
 * You need to output 2.
 *
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/assign-cookies/
// discuss: https://leetcode.com/problems/assign-cookies/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        g.sort();
        let mut s = s;
        s.sort();
        let mut g_index = g.len() as i32 - 1;
        let mut s_index = s.len() as i32 - 1;
        let mut ret = 0;
        while g_index >= 0 && s_index >= 0 {
            if s[s_index as usize] >= g[g_index as usize] {
                ret += 1;
                s_index -= 1;
                g_index -= 1;
            } else {
                g_index -= 1;
            }
        }
        ret
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_455() {}
}
