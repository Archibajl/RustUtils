// mod binary_tree {
use std::boxed::Box;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::option::Option::Some;

pub struct BinaryTree<T> {
    value: T,
    left: Option<Box<RefCell<BinaryTree<T>>>>,
    right: Option<Box<RefCell<BinaryTree<T>>>>,
}
impl<T: PartialOrd + Clone> BinaryTree<T> {
    #![no_main]

    // Constructor for creating a new TreeNode
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    // Method to insert a value into the BinaryTree
    pub fn insert(&mut self, value: T) {
        match value.partial_cmp(&self.value).unwrap() {
            Ordering::Less => {
                if let Some(left) = &self.left {
                    left.borrow_mut().insert(value);
                } else {
                    self.left = Some(Box::new(RefCell::new(BinaryTree::new(value))));
                }
            }
            Ordering::Greater => {
                if let Some(right) = &self.right {
                    right.borrow_mut().insert(value);
                } else {
                    self.right = Some(Box::new(RefCell::new(BinaryTree::new(value))));
                }
            }
            Ordering::Equal => {
                // Do nothing for equal values (assuming no duplicates)
            }
        }
    }
    // Method to search and return a single item if found
    pub fn search(&self, value: &T) -> Option<T> {
        match value.partial_cmp(&self.value) {
            Some(Ordering::Equal) => Some(self.value.clone()),
            Some(Ordering::Less) => {
                if let Some(left) = &self.left {
                    left.borrow().search(value)
                } else {
                    None
                }
            }
            Some(Ordering::Greater) => {
                if let Some(right) = &self.right {
                    right.borrow().search(value)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
