use std::{cell::RefCell, rc::Rc};

// Definition for singly-linked list.
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
}

// Solution
pub fn delete_node(
    head: Option<Box<ListNode>>,
    n: i32,
) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    if n == 0 {
        return head.unwrap().next;
    }

    let head = Rc::new(RefCell::new(head.unwrap()));

    {
        // Scoped memory
        let cursor = head.clone();
        let mut cursor = cursor.borrow_mut();

        let mut i = 0;
        while let Some(node) = cursor.next.as_mut() {
            if i + 1 == n {
                match node.next.take() {
                    Some(child) => node.next = child.next,
                    None => node.next = None,
                }
                break;
            }
            i += 1;
        }
    }

    let head = Rc::try_unwrap(head).unwrap().into_inner();
    Some(head)
}

#[cfg(test)]
#[test]
fn test_1() {
    let mut linked_list = ListNode::new(1);
    linked_list.next = Some(Box::new(ListNode::new(2)));
    linked_list.next = Some(Box::new(ListNode::new(3)));
    linked_list.next = Some(Box::new(ListNode::new(4)));
    linked_list.next = Some(Box::new(ListNode::new(5)));

    let mut after = ListNode::new(1);
    after.next = Some(Box::new(ListNode::new(2)));
    // after.next = Some(Box::new(ListNode::new(3))); <-- Deleted
    after.next = Some(Box::new(ListNode::new(4)));
    after.next = Some(Box::new(ListNode::new(5)));

    assert_eq!(
        delete_node(Some(Box::new(linked_list)), 2),
        Some(Box::new(after))
    );
}

#[test]
fn test_2() {
    let linked_list = Some(Box::new(ListNode::new(1)));
    assert_eq!(delete_node(linked_list, 0), None);
}

#[test]
fn test_3() {
    assert_eq!(delete_node(None, 0), None);
}
