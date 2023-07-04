// Define the binary tree structure
#[derive(Debug)]
struct BinaryTree<T: Clone> {
    value: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T: Clone> BinaryTree<T> {
    // Create a new binary tree node
    fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    // Insert a value into the binary tree
    fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        if value < self.value {
            if let Some(left) = &mut self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(BinaryTree::new(value)));
            }
        } else if let Some(right) = &mut self.right {
            right.insert(value);
        } else {
            self.right = Some(Box::new(BinaryTree::new(value)));
        }
    }

    // Perform an in-order traversal of the binary tree
    fn inorder(&self)
    where
        T: std::fmt::Display,
    {
        if let Some(left) = &self.left {
            left.inorder();
        }
        println!("{}", self.value);
        if let Some(right) = &self.right {
            right.inorder();
        }
    }
}

fn main() {
    // Create a binary tree
    let mut tree = BinaryTree::new(5);
    tree.insert(3);
    tree.insert(8);
    tree.insert(2);
    tree.insert(4);
    tree.insert(7);
    tree.insert(9);

    tree.inorder();
}
