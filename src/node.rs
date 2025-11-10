/// A node in the tree, identified by its index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node(pub usize);

impl Node {
    pub fn new(id: usize) -> Self {
        Node(id)
    }

    pub fn index(&self) -> usize {
        self.0
    }
}

impl From<usize> for Node {
    fn from(id: usize) -> Self {
        Node(id)
    }
}

impl From<Node> for usize {
    fn from(node: Node) -> Self {
        node.0
    }
}
