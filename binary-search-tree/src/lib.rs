
mod binary_search_tree{
    use std::option::Option::Some;

    struct BinaryTree<T>
    {
        key :T,
        left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>
    }

     impl<T> BinaryTree<T>{
        #![no_main]

        // Constructor for creating a new TreeNode
    fn new(key: T) -> Self {
        BinaryTree::<T> {
            key,
            left: None,
            right: None,
        }
    }

    // Method to insert a new value into the BST
    pub fn insert(&mut self, new_value: T)
    where
        T: PartialOrd,
    {
        if new_value < self.key {
            match self.left {
                Some(ref mut left) => left.insert( new_value),
                None => self.left = Some(Box::new(BinaryTree::new(new_value))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert( new_value),
                None => self.right = Some(Box::new(BinaryTree::new(new_value))),
            }
        }
    }
        pub fn search(&self, value: T) -> Option<Box<BinaryTree<T>>> {
            if value == self.key {
            return self.key;
        } else if value < self.key {
            match self.left {
                Some(left) => left.search(value),
                None => self.left,
            }
        } else {
            match self.right {
                Some(right) => right.search(value),
                None => self.right.key,
            }
        }
        }

        
    }

}