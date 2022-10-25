pub struct Node<T> {
    pub value: T,
    nodes: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, nodes: Vec::new() }
    }

    pub fn add_node(&mut self, value: T) {
        self.nodes.push(Node::new(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_tree() {
        let tree = Node::new(10);

        assert_eq!(tree.value, 10);
    }

    #[test]
    fn tree_add_nodes() {
        let mut tree = Node::new(10);

        tree.add_node(5);
        tree.add_node(4);

        assert_eq!(tree.nodes.len(), 2)
    }
}
