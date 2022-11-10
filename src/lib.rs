pub struct Node<T> {
    pub value: T,
    nodes: Vec<Node<T>>,
}

impl<T> Node<T> where T : PartialEq {
    pub fn new(value: T) -> Self {
        Node { value, nodes: Vec::new() }
    }

    pub fn add_node(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    pub fn find_node(&self, value: T) -> Option<&Node<T>> {
        for node in self.nodes.iter() {
            if node.value == value {
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

        tree.add_node(Node::new(5));
        tree.add_node(Node::new(4));

        assert_eq!(tree.nodes.len(), 2)
    }

    #[test]
    fn tree_finds_nodes() {
        let mut tree = Node::new(10);

        tree.add_node(Node::new(5));
        tree.add_node(Node::new(4));

        let found = tree.find_node(5).unwrap();

        assert_eq!(found.value, 5);
        assert_eq!(found.nodes.len(), 0);
    }

    #[test]
    #[should_panic]
    fn tree_finds_none() {
        let mut tree = Node::new(10);

        tree.add_node(Node::new(5));
        tree.add_node(Node::new(4));

        tree.find_node(1).expect("No node with this value found!");
    }

    #[test]
    fn tree_multi_level_node() {
        let mut main_tree = Node::new(10);

        let mut node_level_1 = Node::new(3);
        let node_level_2 = Node::new(2);

        node_level_1.add_node(node_level_2);
        main_tree.add_node(node_level_1);

        let found = main_tree.find_node(3).unwrap().find_node(2);

        assert_eq!(found.unwrap().value, 2);
    }
}
