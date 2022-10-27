pub struct Node<T> {
    pub value: T,
    nodes: Vec<Node<T>>,
}

impl<T> Node<T> where T : PartialEq {
    pub fn new(value: T) -> Self {
        Node { value, nodes: Vec::new() }
    }

    pub fn add_node(&mut self, value: T) {
        self.nodes.push(Node::new(value));
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

        tree.add_node(5);
        tree.add_node(4);

        assert_eq!(tree.nodes.len(), 2)
    }

    #[test]
    fn tree_finds_nodes() {
        let mut tree = Node::new(10);

        tree.add_node(5);
        tree.add_node(4);

        let found = tree.find_node(5).unwrap();

        assert_eq!(found.value, 5);
        assert_eq!(found.nodes.len(), 0);
    }

    #[test]
    #[should_panic]
    fn tree_finds_none() {
        let mut tree = Node::new(10);

        tree.add_node(5);
        tree.add_node(4);

        tree.find_node(1).expect("No node with this value found!");
    }
}
