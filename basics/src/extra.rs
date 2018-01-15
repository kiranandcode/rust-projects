pub enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>)
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>
}
impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree::Empty
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty => *self = BinaryTree::NonEmpty(Box::new(TreeNode{
                element: value,
                left: BinaryTree::Empty,
                right: BinaryTree::Empty
            })),
            BinaryTree::NonEmpty(ref mut node) => 
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
        }
    }
}