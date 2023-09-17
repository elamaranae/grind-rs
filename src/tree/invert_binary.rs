// Given the root of a binary tree, invert the tree, and return its root.

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

impl BinaryTree {
    fn new(arr: &[i32]) -> Self {
        Self {
            root: Self::create_node(0, arr),
        }
    }

    fn create_node(position: usize, arr: &[i32]) -> Link {
        if position >= arr.len() {
            return Link(None);
        }

        Link(Some(Rc::new(RefCell::new(TreeNode {
            val: arr[position],
            left: Self::create_node(2 * position + 1, arr),
            right: Self::create_node(2 * position + 2, arr),
        }))))
    }

    fn invert(&mut self) {
        self.root.invert();
    }
}

impl Link {
    fn invert(&mut self) {
        if let Some(node) = &mut self.0 {
            let mut mut_node = node.borrow_mut();
            mut_node.left.invert();
            mut_node.right.invert();
            let temp = mut_node.left.0.take();
            mut_node.left.0 = mut_node.right.0.take();
            mut_node.right.0 = temp;
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
    let arr_1 = [4, 2, 7, 1, 3, 6, 9];
    let tree_1 = BinaryTree::new(&arr_1);

    let arr_2 = [4, 2, 7, 1, 3, 6, 9];
    let tree_2 = BinaryTree::new(&arr_2);

    assert_eq!(tree_1, tree_2);
}

#[test]
fn case_1() {
    let arr_1 = [4, 2, 7, 1, 3, 6, 9];
    let tree_1 = BinaryTree::new(&arr_1);

    let arr_2 = [4, 2, 7, 2, 3, 6, 9];
    let tree_2 = BinaryTree::new(&arr_2);

    assert_ne!(tree_1, tree_2);
}

#[test]
fn case_2() {
    let arr_1 = [4, 2, 7, 3, 6, 9];
    let tree_1 = BinaryTree::new(&arr_1);

    let arr_2 = [4, 2, 7, 1, 3, 6, 9];
    let tree_2 = BinaryTree::new(&arr_2);

    assert_ne!(tree_1, tree_2);
}

#[test]
fn case_3() {
    let arr_1 = [4, 2, 7, 1, 3, 6, 9];
    let mut tree_1 = BinaryTree::new(&arr_1);
    tree_1.invert();

    let arr_2 = [4, 7, 2, 9, 6, 3, 1];
    let tree_2 = BinaryTree::new(&arr_2);

    assert_eq!(tree_1, tree_2);
}

#[test]
fn case_4() {
    let arr_1 = [2,1,3];
    let mut tree_1 = BinaryTree::new(&arr_1);
    tree_1.invert();

    let arr_2 = [2,3,1];
    let tree_2 = BinaryTree::new(&arr_2);

    assert_eq!(tree_1, tree_2);
}
