/**
 * [337] House Robber III
 *
 * The thief has found himself a new place for his thievery again. There is only one entrance to this area, called the "root." Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that "all houses in this place forms a binary tree". It will automatically contact the police if two directly-linked houses were broken into on the same night.
 *
 * Determine the maximum amount of money the thief can rob tonight without alerting the police.
 *
 * Example 1:
 *
 *
 * Input: [3,2,3,null,3,null,1]
 *
 *      <font color="red">3</font>
 *     / \
 *    2   3
 *     \   \
 *      <font color="red">3   1
 * </font>
 * Output: 7
 * Explanation: Maximum amount of money the thief can rob = <font color="red" style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">3</font><span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";"> + </span><font color="red" style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">3</font><span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";"> + </span><font color="red" style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">1</font><span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";"> = </span><b style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">7<span style="font-family: sans-serif, Arial, Verdana, "Trebuchet MS";">.</span>
 *
 * Example 2:
 *
 *
 * Input: [3,4,5,1,3,null,1]
 *
 *      3
 *     / \
 *    <font color="red">4</font>   <font color="red">5</font>
 *   / \   \
 *  1   3   1
 *
 * Output: 9
 * Explanation: Maximum amount of money the thief can rob = <font color="red">4</font> + <font color="red">5</font> = 9.
 *
 */
pub struct Solution {}
use crate::util::tree::{to_tree, TreeNode};

// problem: https://leetcode.com/problems/house-robber-iii/
// discuss: https://leetcode.com/problems/house-robber-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let left = root.as_ref().unwrap().borrow().left.clone();
        let right = root.as_ref().unwrap().borrow().right.clone();
        if left.is_none() && right.is_none() {
            return root.unwrap().borrow().val;
        }
        let (left_left, left_right) = if left.is_none() {
            (None, None)
        } else {
            (
                left.as_ref().unwrap().borrow().left.clone(),
                left.as_ref().unwrap().borrow().right.clone(),
            )
        };

        let (right_left, right_right) = if right.is_none() {
            (None, None)
        } else {
            (
                right.as_ref().unwrap().borrow().left.clone(),
                right.as_ref().unwrap().borrow().right.clone(),
            )
        };

        (root.unwrap().borrow().val
            + Self::rob(left_left)
            + Self::rob(left_right)
            + Self::rob(right_left)
            + Self::rob(right_right))
        .max(Self::rob(left) + Self::rob(right))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_337() {}
}
