use crate::ListNode;
use std::{cell::RefCell, rc::Rc};

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
    let mut head = ListNode::new(1);
    let mut i1 = ListNode::new(2);
    let mut i2 = ListNode::new(3);
    let mut i3 = ListNode::new(4);
    let i4 = ListNode::new(5);
    i3.next = Some(Box::new(i4));
    i2.next = Some(Box::new(i3));
    i1.next = Some(Box::new(i2));
    head.next = Some(Box::new(i1));

    let mut after_head = ListNode::new(1);
    let mut i1 = ListNode::new(2);
    //let mut i2 = ListNode::new(3); <-- Deleted
    let mut i3 = ListNode::new(4);
    let i4 = ListNode::new(5);
    i3.next = Some(Box::new(i4));
    // i2.next = Some(Box::new(i3)); <-- Deleted
    i1.next = Some(Box::new(i3));
    after_head.next = Some(Box::new(i1));

    assert_eq!(
        delete_node(Some(Box::new(head)), 2),
        Some(Box::new(after_head))
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
