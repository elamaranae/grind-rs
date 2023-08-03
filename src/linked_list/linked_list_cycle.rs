/*
   Given head, the head of a linked list, determine if the linked list has a cycle in it.

   There is a cycle in a linked list if there is some node in the list that can be reached again by
   continuously following the next pointer. Internally, pos is used to denote the index of the node
   that tail's next pointer is connected to. Note that pos is not passed as a parameter.

   Return true if there is a cycle in the linked list. Otherwise, return false.
*/

use std::{rc::Rc, cell::RefCell};

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct List {
    head: Link
}

impl List {
    fn new(arr: &[i32], close: Option<u32>) -> Self {
        let mut head: Link = None;
        let mut tail_node = None;

        for val in arr.iter().rev() {
            let is_head_none = head.is_none();
            let link = Rc::new(RefCell::new(Node { val: *val, next: head }));
            if is_head_none { tail_node = Some(link.clone()); }
            head = Some(link);
        }

        if close.is_none() || tail_node.is_none() { return Self { head }; }

        let mut close_node = head.clone();

        for _ in 0..close.unwrap() {
            if close_node.is_none() { break; }

            let n = close_node.unwrap();
            close_node = n.borrow().next.clone();
        }

        tail_node.unwrap().borrow_mut().next = close_node;

        Self { head }
    }

    fn has_cycle(&self) -> bool {
        let mut slow = self.head.clone();
        let mut fast = self.head.clone();


        while slow.is_some() && fast.is_some() && fast.as_ref().unwrap().borrow().next.is_some() {
            slow = slow.unwrap().borrow().next.clone();
            fast = fast.unwrap().borrow().next.as_ref().unwrap().borrow().next.clone();

            if fast.is_some() && Rc::ptr_eq(slow.as_ref().unwrap(), fast.as_ref().unwrap()) {
                return true;
            }
        }

        false
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        let mut link_1 = self.head.clone();
        let mut link_2 = other.head.clone();

        while link_1.is_some() && link_2.is_some() {
            let n1 = link_1.unwrap();
            let n2 = link_2.unwrap();
            let node_1 = n1.borrow();
            let node_2 = n2.borrow();

            if node_1.val != node_2.val {
                return false;
            }

            link_1 = node_1.next.clone();
            link_2 = node_2.next.clone();
        }

        link_1.is_none() && link_2.is_none()
    }
}

impl Eq for List {}

#[test]
fn case_0() {
    let list_1 = List::new(&[1,2,3,4,5], Some(2));
    let list_2 = List::new(&[1,2,3,4,5], None);
    assert_ne!(list_1, list_2);
}

#[test]
fn case_1() {
    let list_1 = List::new(&[1,3,3,4,5], None);
    let list_2 = List::new(&[1,2,3,4,5], None);
    assert_ne!(list_1, list_2);
}

#[test]
fn case_2() {
    let list_1 = List::new(&[1,2,5], None);
    let list_2 = List::new(&[1,2,3,4,5], None);
    assert_ne!(list_1, list_2);
}

#[test]
fn case_3() {
    let list_1 = List::new(&[], None);
    let list_2 = List::new(&[1,2,3,4,5], None);
    assert_ne!(list_1, list_2);
}

#[test]
fn case_4() {
    let list_1 = List::new(&[], None);
    let list_2 = List::new(&[], None);
    assert_eq!(list_1, list_2);
}

#[test]
fn case_5() {
    let list_1 = List::new(&[1,2,3], None);
    let list_2 = List::new(&[1,2,3,4,5], None);
    assert_ne!(list_1, list_2);
}

#[test]
fn case_6() {
    let list_1 = List::new(&[1,2,3,4,5], None);

    assert!(!list_1.has_cycle());
}

#[test]
fn case_7() {
    let list_1 = List::new(&[1,2,3,4,5], Some(2));

    assert!(list_1.has_cycle());
}

#[test]
fn case_8() {
    let list_1 = List::new(&[1], Some(0));

    assert!(list_1.has_cycle());
}

#[test]
fn case_9() {
    let list_1 = List::new(&[1], None);

    assert!(!list_1.has_cycle());
}

#[test]
fn case_10() {
    let list_1 = List::new(&[], None);

    assert!(!list_1.has_cycle());
}

#[test]
fn case_11() {
    let list_1 = List::new(&[1], Some(5));

    assert!(!list_1.has_cycle());
}

#[test]
fn case_12() {
    let list_1 = List::new(&[1,2,3,4], None);

    assert!(!list_1.has_cycle());
}

#[test]
fn case_13() {
    let list_1 = List::new(&[], Some(2));

    assert!(!list_1.has_cycle());
}
