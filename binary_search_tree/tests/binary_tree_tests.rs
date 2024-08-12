// extern crate binary_search_tree; // Replace with your actual crate name


fn main(){
    println!("Running Tests");
}

    use binary_search_tree::binary_tree::BinaryTree;
    
    #[test]
    fn test_insert_and_search() {
        let mut bst = BinaryTree::new(10);
    
        // Insert some values into the BST
        bst.insert(5);
        bst.insert(15);
        bst.insert(3);
        bst.insert(7);
        bst.insert(18);
    
        // Search for a value that exists
        let result = bst.search(&7);
        assert_eq!(result, Some(7));
    
        // Search for a value that doesn't exist
        let result = bst.search(&20);
        assert_eq!(result, None);
    
        // Search for the root value
        let result = bst.search(&10);
        assert_eq!(result, Some(10));
    
        // Search for another existing value
        let result = bst.search(&15);
        assert_eq!(result, Some(15));
    }
