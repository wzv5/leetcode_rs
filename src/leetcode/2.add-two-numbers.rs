/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

// @lc code=start
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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut p = &mut result;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut a = 0;
        loop {
            if l1.is_none() && l2.is_none() {
                break;
            }
            let v1 = match l1 {
                Some(node) => {
                    l1 = node.next;
                    node.val
                }
                None => 0,
            };
            let v2 = match l2 {
                Some(node) => {
                    l2 = node.next;
                    node.val
                }
                None => 0,
            };
            let mut v = v1 + v2 + a;
            a = if v >= 10 {
                v -= 10;
                1
            } else {
                0
            };
            let p1 = p.as_mut().unwrap();
            p1.next = Some(Box::new(ListNode::new(v)));
            p = &mut p1.next;
        }
        if a == 1 {
            p.as_mut().unwrap().next = Some(Box::new(ListNode::new(1)));
        }
        result.unwrap().next
    }
}
// @lc code=end

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_slice(s: &[i32]) -> Option<Box<ListNode>> {
        let mut result = Some(Box::new(ListNode::new(0)));
        let mut p = &mut result;
        for i in s {
            let p1 = p.as_mut().unwrap();
            p1.next = Some(Box::new(ListNode::new(*i)));
            p = &mut p1.next;
        }
        result.unwrap().next
    }
}

struct Solution;

#[test]
fn test() {
    assert_eq!(
        Solution::add_two_numbers(
            ListNode::from_slice(&[2, 4, 3][..]),
            ListNode::from_slice(&[5, 6, 4][..]),
        ),
        ListNode::from_slice(&[7, 0, 8][..])
    );
    assert_eq!(
        Solution::add_two_numbers(
            ListNode::from_slice(&[0][..]),
            ListNode::from_slice(&[0][..]),
        ),
        ListNode::from_slice(&[0][..])
    );
    assert_eq!(
        Solution::add_two_numbers(
            ListNode::from_slice(&[9, 9, 9, 9, 9, 9, 9][..]),
            ListNode::from_slice(&[9, 9, 9, 9][..]),
        ),
        ListNode::from_slice(&[8, 9, 9, 9, 0, 0, 0, 1][..])
    );
}
