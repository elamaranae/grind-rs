/*
 Given a binary search tree (BST), find the lowest common ancestor (LCA) node of two given nodes
 in the BST.
*/

use std::cmp::Ordering;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

#[derive(Debug)]
struct Link(Option<Rc<RefCell<TreeNode>>>);

#[derive(Debug)]
struct BinaryTree {
    root: Link,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Current,
}

impl BinaryTree {
    fn new(arr: &[Option<i32>]) -> Self {
        Self {
            root: Self::create_node(0, arr),
        }
    }

    fn create_node(position: usize, arr: &[Option<i32>]) -> Link {
        if position >= arr.len() || arr[position].is_none() {
            return Link(None);
        }

        Link(Some(Rc::new(RefCell::new(TreeNode {
            val: arr[position].unwrap(),
            left: Self::create_node(2 * position + 1, arr),
            right: Self::create_node(2 * position + 2, arr),
        }))))
    }

    fn run(&self, p: i32, q: i32) -> i32 {
        let mut current_node = Link(self.root.0.clone());

        loop {
            let next_p = current_node.next_step_to_find(p);
            let next_q = current_node.next_step_to_find(q);

            if (next_p != next_q) || next_p == Direction::Current || next_q == Direction::Current {
                return current_node.0.as_ref().unwrap().borrow().val;
            }

            let node = current_node.0.unwrap();
            let n = node.borrow();

            if next_p == Direction::Left {
                current_node = Link(n.left.0.clone());
            } else {
                current_node = Link(n.right.0.clone());
            }
        }

        return 0;
    }
}

impl Link {
    fn next_step_to_find(&self, p: i32) -> Direction {
        match p.cmp(&self.0.as_ref().unwrap().borrow().val) {
            Ordering::Equal => Direction::Current,
            Ordering::Greater => Direction::Right,
            Ordering::Less => Direction::Left,
        }
    }
}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_some() && other.0.is_some() {
            let node_1 = self.0.as_ref().unwrap().borrow();
            let node_2 = other.0.as_ref().unwrap().borrow();
            return node_1.val == node_2.val
                && node_1.left == node_2.left
                && node_1.right == node_2.right;
        }

        self.0.is_none() && other.0.is_none()
    }
}

impl PartialEq for BinaryTree {
    fn eq(&self, other: &Self) -> bool {
        self.root == other.root
    }
}

#[test]
fn case_0() {
    let arr_1 = [
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ];
    let tree_1 = BinaryTree::new(&arr_1);

    let arr_2 = [
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ];
    let tree_2 = BinaryTree::new(&arr_1);

    assert_eq!(tree_1, tree_2);
}

#[test]
fn case_1() {
    let arr_1 = [
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ];
    let tree_1 = BinaryTree::new(&arr_1);

    assert_eq!(tree_1.run(2, 8), 6);
}

#[test]
fn case_2() {
    let arr_1 = [
        Some(6),
        Some(2),
        Some(8),
        Some(0),
        Some(4),
        Some(7),
        Some(9),
        None,
        None,
        Some(3),
        Some(5),
    ];
    let tree_1 = BinaryTree::new(&arr_1);

    assert_eq!(tree_1.run(2, 4), 2);
}

#[test]
fn case_3() {
    let arr_1 = [Some(2), Some(1)];
    let tree_1 = BinaryTree::new(&arr_1);

    assert_eq!(tree_1.run(2, 1), 2);
}
