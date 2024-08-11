extern crate binary_search_tree;

fn main(){
    println("Running Tests");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_insert_and_contains() {
        root = new_tree();

        // Test that the root contains the correct values
        assert!(root.contains(10));
        assert!(root.contains(5));
        assert!(root.contains(15));
        assert!(root.contains(3));
        assert!(root.contains(7));

        // Test that the root does not contain a value that wasn't inserted
        assert!(!root.contains(20));
        assert!(!root.contains(0));
    }

    #[test]
    fn test_empty_tree() {
        
        let root = new_tree();

        // Test that the root contains its own value
        assert!(root.contains(10));

        // Test that the root does not contain any other value
        assert!(!root.contains(5));
        assert!(!root.contains(15));
    }

    fn new_tree(){
        let root = BinaryTree::new(10);
        // Insert values into the BST
        root.insert(5);
        root.insert(15);
        root.insert(3);
        root.insert(7);
        println!("Binary Search Tree created.");
        return root;
    }
    
}

