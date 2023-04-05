/*
   You are given the heads of two sorted linked lists list1 and list2.

   Merge the two lists in a one sorted list. The list should be made by splicing together the nodes
   of the first two lists. Return the head of the merged linked list.
*/

use std::mem;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct List {
    head: Link
}

impl List {
    fn from_array(arr: &[i32]) -> List {
        let mut head: Link = None;

        for val in arr.iter().rev() {
            let link = Box::new(Node { val: *val, next: head });
            head = Some(link);
        }

        List { head }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        let mut link_1 = &self.head;
        let mut link_2 = &other.head;

        while link_1.is_some() && link_2.is_some() {
            let node_1 = link_1.as_ref().unwrap();
            let node_2 = link_2.as_ref().unwrap();

            if node_1.val != node_2.val {
                return false;
            }

            link_1 = &node_1.next;
            link_2 = &node_2.next;
        }

        link_1.is_none() && link_2.is_none()
    }
}

impl Eq for List {}

pub fn run(list_1: List, list_2: List) -> List {

    let mut link_1 = list_1.head;
    let mut link_2 = list_2.head;
    let mut head: Link = None;
    let mut tail = &mut head;

    while link_1.is_some() && link_2.is_some() {
        if link_1.as_ref().unwrap().val < link_2.as_ref().unwrap().val {
            let temp = link_1.as_mut().unwrap().next.take();
            *tail = link_1;
            link_1 = temp;
        } else {
            let temp = link_2.as_mut().unwrap().next.take();
            *tail = link_2;
            link_2 = temp;
        }

        tail = &mut tail.as_mut().unwrap().next;
    }

    *tail = if link_1.is_some() { link_1 } else { link_2 };

    List { head }
}

#[test]
fn case_0() {
   let list_1 = List::from_array(&[1,2,3,4,7]); 
   let list_2 = List::from_array(&[1,2,3,4,7]); 
   let expected = List::from_array(&[1,1,2,2,3,3,4,4,7,7]);

   let actual = run(list_1, list_2);

   assert_eq!(actual, expected);
}

#[test]
fn case_1() {
   let list_1 = List::from_array(&[]); 
   let list_2 = List::from_array(&[1,2,3,4,7]); 
   let expected = List::from_array(&[1,2,3,4,7]); 

   let actual = run(list_1, list_2);
   assert_eq!(actual, expected);
}

#[test]
fn case_2() {
   let list_1 = List::from_array(&[]); 
   let list_2 = List::from_array(&[]); 
   let expected = List::from_array(&[]);

   let actual = run(list_1, list_2);

   assert_eq!(actual, expected);
}

#[test]
fn case_3() {
   let list_1 = List::from_array(&[1,2,4]); 
   let list_2 = List::from_array(&[1,3,4]); 
   let expected = List::from_array(&[1,1,2,3,4,4]);

   let actual = run(list_1, list_2);

   assert_eq!(actual, expected);
}
