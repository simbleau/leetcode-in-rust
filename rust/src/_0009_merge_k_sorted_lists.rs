// Time: O(N logk), where k is the number of linked lists
// Space: O(1)

// class Solution(object):
//     def mergeKLists(self, lists):
//         """
//         :type lists: List[ListNode]
//         :rtype: ListNode
//         """
//         amount = len(lists)
//         interval = 1
//         while interval < amount:
//             for i in range(0, amount - interval, interval * 2):
//                 lists[i] = self.merge2Lists(lists[i], lists[i + interval])
//             interval *= 2
//         return lists[0] if amount > 0 else None
//
//     def merge2Lists(self, l1, l2):
//         head = point = ListNode(0)
//         while l1 and l2:
//             if l1.val <= l2.val:
//                 point.next = l1
//                 l1 = l1.next
//             else:
//                 point.next = l2
//                 l2 = l1
//                 l1 = point.next.next
//             point = point.next
//         if not l1:
//             point.next=l2
//         else:
//             point.next=l1
//         return head.next

use std::{cell::RefCell, rc::Rc};

use crate::ListNode;

pub fn merge_2_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if list1.is_none() && list2.is_none() {
        return None;
    }

    // Merge on list1
    let ptr_1 = match list1 {
        Some(list) => Some(list),
        None => return list2,
    };
    let ptr_2 = list2;

    let head = Rc::new(RefCell::new(ptr_1.unwrap()));
    {
        let head_clone = head.clone();
        let cursor = &mut *head_clone.borrow_mut();
        while ptr_2.is_some() {
            if let Some(node) = ptr_2 && {
            }
        }
    }

    let head = Rc::try_unwrap(head).unwrap().into_inner();
    Some(head)
}

pub fn merge_k_lists(
    lists: Vec<Option<Box<ListNode>>>,
) -> Option<Box<ListNode>> {
    None
}

#[cfg(test)]
#[test]
fn test_1() {
    let mut head_1 = ListNode::new(1);
    let mut i2 = ListNode::new(4);
    let i3 = ListNode::new(5);
    i2.next = Some(Box::new(i3));
    head_1.next = Some(Box::new(i2));

    let mut head_2 = ListNode::new(1);
    let mut i2 = ListNode::new(3);
    let i3 = ListNode::new(4);
    i2.next = Some(Box::new(i3));
    head_2.next = Some(Box::new(i2));

    let mut head_3 = ListNode::new(2);
    let mut i2 = ListNode::new(6);
    head_3.next = Some(Box::new(i2));

    let mut head_merged = ListNode::new(1);
    let mut i2 = ListNode::new(1);
    let mut i3 = ListNode::new(2);
    let mut i4 = ListNode::new(3);
    let mut i5 = ListNode::new(4);
    let mut i6 = ListNode::new(4);
    let mut i7 = ListNode::new(5);
    let i8 = ListNode::new(6);
    i7.next = Some(Box::new(i8));
    i6.next = Some(Box::new(i7));
    i5.next = Some(Box::new(i6));
    i4.next = Some(Box::new(i5));
    i3.next = Some(Box::new(i4));
    i2.next = Some(Box::new(i3));
    head_merged.next = Some(Box::new(i2));

    assert_eq!(
        merge_k_lists(vec![
            Some(Box::new(head_1)),
            Some(Box::new(head_2)),
            Some(Box::new(head_3))
        ]),
        Some(Box::new(head_merged))
    );
}

#[test]
fn test_2() {
    assert_eq!(merge_k_lists(vec![]), None);
}

#[test]
fn test_3() {
    assert_eq!(merge_k_lists(vec![None]), None);
}
