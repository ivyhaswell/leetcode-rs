use crate::{linked_list, ListNode};

impl Solution {
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = &mut l1;
        let mut l2 = &mut l2;
        // 进位
        let mut carry = 0;

        let mut ans = ListNode::new(0);
        let mut curr = &mut ans;

        loop {
            if l1.is_none() && l2.is_none() && carry == 0 {
                break;
            }

            let val1 = if let Some(n1) = l1 { n1.val } else { 0 };
            let val2 = if let Some(n2) = l2 { n2.val } else { 0 };
            let sum = val1 + val2 + carry;

            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();

            if let Some(node) = l1 {
                l1 = &mut node.next
            };

            if let Some(node) = l2 {
                l2 = &mut node.next
            };
        }

        ans.next
    }
}

struct Solution {}

#[test]
pub fn test() {
    assert_eq!(
        Solution::add_two_numbers(linked_list![2, 4, 3], linked_list![5, 6, 4]),
        linked_list![7, 0, 8]
    );
    assert_eq!(
        Solution::add_two_numbers(linked_list![0], linked_list![0]),
        linked_list![0]
    );
    assert_eq!(
        Solution::add_two_numbers(linked_list![9, 9, 9, 9, 9, 9, 9], linked_list![9, 9, 9, 9]),
        linked_list![8, 9, 9, 9, 0, 0, 0, 1]
    );
}
