use std::fmt;

// reference: https://github.com/Aloxaf/leetcode_prelude/blob/master/leetcode_prelude/src/linkedlist.rs

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl fmt::Display for ListNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut v = vec![self.val];
        let mut p = self;
        while let Some(next) = p.next.as_ref() {
            v.push(next.val);
            p = next;
        }
        write!(f, "{:?}", v)
    }
}

#[macro_export]
macro_rules! linked_list {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new($crate::ListNode::new(0));
            let mut ref_head = &mut head;

            $(
                ref_head.next = Some(Box::new($crate::ListNode::new($e)));
                ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head;
            head.next
        }
    };
}
