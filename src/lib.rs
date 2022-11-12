pub struct Node<T> {
    pub value: T,
    nodes: Vec<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            nodes: Vec::new(),
        }
    }

    pub fn push(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    pub fn find<F>(&self, f: F) -> Option<&Node<T>>
    where
        F: Fn(&Node<T>) -> bool,
    {
        self.nodes.iter().find(|&node| f(node))
    }
}

impl<T> Node<T>
where
    T: PartialEq,
{
    pub fn find_by_value(&self, value: T) -> Option<&Node<T>> {
        self.find(|node| node.value == value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Line {
        speaker: String,
        text: String,
    }

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

        let found = tree.find(|node| node.value == 5).unwrap();

        assert_eq!(found.value, 5);
        assert_eq!(found.nodes.len(), 0);
    }

    #[test]
    fn tree_finds_by_value() {
        let mut tree = Node::new(10);

        tree.push(Node::new(5));
        tree.push(Node::new(4));

        let found = tree.find_by_value(5).unwrap();

        assert_eq!(found.value, 5);
        assert_eq!(found.nodes.len(), 0);
    }

    #[test]
    #[should_panic]
    fn tree_finds_none() {
        let mut tree = Node::new(10);

        tree.push(Node::new(5));
        tree.push(Node::new(4));

        tree.find(|node| node.value == 1)
            .expect("No node with this value found!");
    }

    #[test]
    fn tree_multi_level_node() {
        let mut main_tree = Node::new(10);

        let mut node_level_1 = Node::new(3);
        let node_level_2 = Node::new(2);

        node_level_1.push(node_level_2);
        main_tree.push(node_level_1);

        let found = main_tree
            .find(|node| node.value == 3)
            .unwrap()
            .find(|node| node.value == 2);

        assert_eq!(found.unwrap().value, 2);
    }

    #[test]
    fn node_handles_struct() {
        let first_line = Line {
            speaker: String::from("Citizen"),
            text: String::from("Welcome!!"),
        };

        let sub_line = Line {
            speaker: String::from("You"),
            text: String::from("Thanks for having me!"),
        };

        let mut node = Node::new(first_line);
        let sub_node = Node::new(sub_line);

        node.push(sub_node);

        let matching = node.find(|n| n.value.speaker == "You").unwrap();

        assert_eq!(matching.value.speaker, "You");
        assert_eq!(matching.value.text, "Thanks for having me!");
    }
}
