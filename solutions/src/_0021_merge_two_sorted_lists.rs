use crate::ListNode;

pub fn insertion_sort(
    items: &mut Vec<Box<ListNode>>,
    lo: usize,
    hi: usize,
    new_item: &Box<ListNode>,
) {
    // Base case
    if hi - lo <= 1 {
        if hi - lo == 0 {
            items.insert(lo, Box::new(ListNode::new(new_item.val)));
        } else {
            if new_item.val > items[lo].val {
                items.insert(lo + 1, Box::new(ListNode::new(new_item.val)));
            } else {
                items.insert(lo, Box::new(ListNode::new(new_item.val)));
            }
        }
        return;
    }

    // Split by 2 and insert on better half
    let offset = (hi - lo) % 2;
    let middle = lo + (hi - lo) / 2 + offset;

    // let lower_half = &items[lo..(hi - lo) / 2];
    let upper_half = &items[middle..hi];
    if new_item.val > upper_half[0].val {
        // Insert into upper half
        insertion_sort(items, middle, hi, new_item);
    } else {
        // Insert into lower half
        insertion_sort(items, lo, middle, new_item);
    }
}

// Solution
pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut merged: Vec<Box<ListNode>> = vec![];

    // Buffers for binary search
    let lo = 0;
    let mut hi = 0;

    // Insertion sort on list 1
    match list1 {
        Some(head) => {
            let mut cursor = head;
            merged.insert(0, Box::new(ListNode::new(cursor.val)));
            hi += 1;
            while cursor.next.is_some() {
                cursor = cursor.next.unwrap();
                insertion_sort(&mut merged, lo, hi, &cursor);
                hi += 1;
            }
        }
        None => (),
    };

    // Insertion sort on list2
    match list2 {
        Some(head) => {
            let mut cursor = head;
            insertion_sort(&mut merged, lo, hi, &cursor);
            hi += 1;
            while cursor.next.is_some() {
                cursor = cursor.next.unwrap();
                insertion_sort(&mut merged, lo, hi, &cursor);
                hi += 1;
            }
        }
        None => (),
    };

    match merged.len() {
        0 => None,
        _ => {
            let mut tail = ListNode::new(merged[merged.len() - 1].val);
            for i in (0..merged.len() - 1).rev() {
                let val = merged[i].val;
                let mut head = ListNode::new(val);
                head.next = Some(Box::new(tail));
                tail = head;
            }

            Some(Box::new(tail))
        }
    }
}

#[cfg(test)]
#[test]
fn test_1() {
    let mut head_1 = ListNode::new(1);
    let mut i1 = ListNode::new(2);
    let i2 = ListNode::new(4);
    i1.next = Some(Box::new(i2));
    head_1.next = Some(Box::new(i1));

    let mut head_2 = ListNode::new(1);
    let mut i1 = ListNode::new(3);
    let i2 = ListNode::new(4);
    i1.next = Some(Box::new(i2));
    head_2.next = Some(Box::new(i1));

    let mut after_head = ListNode::new(1);
    let mut i1 = ListNode::new(1);
    let mut i2 = ListNode::new(2);
    let mut i3 = ListNode::new(3);
    let mut i4 = ListNode::new(4);
    let i5 = ListNode::new(4);
    i4.next = Some(Box::new(i5));
    i3.next = Some(Box::new(i4));
    i2.next = Some(Box::new(i3));
    i1.next = Some(Box::new(i2));
    after_head.next = Some(Box::new(i1));

    assert_eq!(
        merge_two_lists(Some(Box::new(head_1)), Some(Box::new(head_2))),
        Some(Box::new(after_head))
    );
}

#[test]
fn test_2() {
    assert_eq!(merge_two_lists(None, None), None);
}

#[test]
fn test_3() {
    let head_1 = ListNode::new(1);
    let merged = ListNode::new(1);
    assert_eq!(
        merge_two_lists(Some(Box::new(head_1)), None),
        Some(Box::new(merged))
    );
}

#[test]
fn test_4() {
    let head_1 = ListNode::new(0);
    let merged = ListNode::new(0);
    assert_eq!(
        merge_two_lists(None, Some(Box::new(head_1))),
        Some(Box::new(merged))
    );
}

#[test]
fn test_5() {
    let head_1 = ListNode::new(2);
    let head_2 = ListNode::new(1);
    let mut merged = ListNode::new(1);
    merged.next = Some(Box::new(ListNode::new(2)));
    assert_eq!(
        merge_two_lists(Some(Box::new(head_1)), Some(Box::new(head_2))),
        Some(Box::new(merged))
    );
}
