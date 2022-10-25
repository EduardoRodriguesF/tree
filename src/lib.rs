pub struct TreeNode<T> {
    pub value: T,
    nodes: Vec<TreeNode<T>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode { value, nodes: Vec::new() }
    }

    pub fn add_node(&mut self, value: T) {
        self.nodes.push(TreeNode::new(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_tree() {
        let tree = TreeNode::new(10);

        assert_eq!(tree.value, 10);
    }

    #[test]
    fn tree_add_nodes() {
        let mut tree = TreeNode::new(2);

        tree.add_node(5);
        tree.add_node(4);

        assert_eq!(tree.nodes.len(), 2)
    }
}
