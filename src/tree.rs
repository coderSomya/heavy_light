use crate::node::Node;

/// A tree structure containing nodes
pub struct Tree<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Vec<usize>>,
}

impl<T> Tree<T> {
    pub(crate) fn new(n: usize, values: Vec<T>) -> Self {
        let nodes = values
            .into_iter()
            .enumerate()
            .map(|(id, value)| Node::new(id, value))
            .collect();
        
        Self {
            nodes,
            edges: vec![Vec::new(); n],
        }
    }

    pub(crate) fn add_edge(&mut self, u: usize, v: usize) {
        self.edges[u].push(v);
        self.edges[v].push(u);
    }

    pub(crate) fn get_node(&self, id: usize) -> Option<&Node<T>> {
        self.nodes.get(id)
    }

    pub(crate) fn get_node_mut(&mut self, id: usize) -> Option<&mut Node<T>> {
        self.nodes.get_mut(id)
    }

    pub(crate) fn get_edges(&self, id: usize) -> &Vec<usize> {
        &self.edges[id]
    }
}

