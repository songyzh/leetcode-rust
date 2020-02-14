/**
 * [328] Odd Even Linked List
 *
 * Given a singly linked list, group all odd nodes together followed by the even nodes. Please note here we are talking about the node number and not the value in the nodes.
 *
 * You should try to do it in place. The program should run in O(1) space complexity and O(nodes) time complexity.
 *
 * Example 1:
 *
 *
 * Input: 1->2->3->4->5->NULL
 * Output: 1->3->5->2->4->NULL
 *
 *
 * Example 2:
 *
 *
 * Input: 2->1->3->5->6->4->7->NULL
 * Output: 2->3->6->7->1->5->4->NULL
 *
 *
 * Note:
 *
 *
 * 	The relative order inside both the even and odd groups should remain as it was in the input.
 * 	The first node is considered odd, the second node even and so on ...
 *
 *
 */
pub struct Solution {}
use crate::util::linked_list::{to_list, ListNode};

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut head = head;
        let mut dummy1 = Some(Box::new(ListNode::new(0)));
        let mut dummy1_ref = dummy1.as_mut();
        let mut dummy2 = Some(Box::new(ListNode::new(0)));
        let mut dummy2_ref = dummy2.as_mut();
        let mut is_even = true;
        while head.is_some() {
            let next = head.as_mut().unwrap().next.take();
            if is_even {
                dummy1_ref.as_mut().unwrap().next = head.take();
                dummy1_ref = dummy1_ref.unwrap().next.as_mut();
            } else {
                dummy2_ref.as_mut().unwrap().next = head.take();
                dummy2_ref = dummy2_ref.unwrap().next.as_mut();
            }
            head = next;
            is_even = !is_even;
        }
        dummy2_ref.as_mut().unwrap().next = None;
        dummy1_ref.as_mut().unwrap().next = dummy2.unwrap().next;
        dummy1.unwrap().next
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_328() {
        println!("{:?}", Solution::odd_even_list(linked![1, 2, 3, 4, 5]))
    }
}
