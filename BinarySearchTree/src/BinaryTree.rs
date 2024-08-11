
mod binary_tree{

    struct BinaryTree<T>
    {
        key :T,
        left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,

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
    fn insert(&mut self, new_value: T)
    where
        T: PartialOrd,
    {
        if new_value < self.key {
            match self.left {
                Some(ref mut left) => left.insert(new_value),
                None => self.left = Some(Box::new(BinaryTree::new(new_value))),
            }
        } else {
            match self.right {
                Some(ref mut right) => right.insert(new_value),
                None => self.right = Some(Box::new(BinaryTree::new(new_value))),
            }
        }
    }
    //     pub fn search(data i128){
    //         if data == key{
    //             return BranchNode;
    //         }
    //         if data > key{
    //             BranchNode = left;
    //             return self::search(key);
    //         }
    //         else{
    //             BranchNode = right;
    //             return self::search(key);
    //         }
    //     }

        // pub fn insert( node1 :BinaryTree<T> node**, data :i128){
        //     if node1 == Null {
        //         let self::key = data;
        //         let self::left = Null;
        //         let self::right = Null;
        //     }
        //     else if node.key > data {
        //         self::insert(&(*node1).left, key);
        //     }
        //     else{
        //         self::insert(&(*node1).right, key);
        //     }
            
        // }

        
    }

}