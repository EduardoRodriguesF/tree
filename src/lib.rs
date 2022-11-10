pub struct Node<T> {
    pub value: T,
    nodes: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node { value, nodes: Vec::new() }
    }

    pub fn push(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    pub fn find_node<F>(&self, f: F) -> Option<&Node<T>>
        where F: Fn(&Node<T>) -> bool {
        for node in self.nodes.iter() {
            if f(node) {
                return Some(node);
            }
        }

        None
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

        tree.push(Node::new(5));
        tree.push(Node::new(4));

        assert_eq!(tree.nodes.len(), 2)
    }

    #[test]
    fn tree_finds_nodes() {
        let mut tree = Node::new(10);

        tree.push(Node::new(5));
        tree.push(Node::new(4));

        let found = tree.find_node(|node| node.value == 5).unwrap();

        assert_eq!(found.value, 5);
        assert_eq!(found.nodes.len(), 0);
    }

    #[test]
    #[should_panic]
    fn tree_finds_none() {
        let mut tree = Node::new(10);

        tree.push(Node::new(5));
        tree.push(Node::new(4));

        tree.find_node(|node| node.value == 1).expect("No node with this value found!");
    }

    #[test]
    fn tree_multi_level_node() {
        let mut main_tree = Node::new(10);

        let mut node_level_1 = Node::new(3);
        let node_level_2 = Node::new(2);

        node_level_1.push(node_level_2);
        main_tree.push(node_level_1);

        let found = main_tree
            .find_node(|node| node.value == 3)
            .unwrap()
            .find_node(|node| node.value == 2);

        assert_eq!(found.unwrap().value, 2);
    }
}
