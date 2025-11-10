use crate::node::Node;

/// A tree structure containing nodes
pub struct Tree<T> {
    nodes: Vec<Node<T>>,
    edges: Vec<Vec<usize>>,
    depth: Vec<usize>,
    par: Vec<Option<usize>>,
    lca_lift: Vec<Vec<Option<usize>>>,
    lg: usize,
    initialized: bool,
}

impl<T> Tree<T> {
    pub(crate) fn new(n: usize, values: Vec<T>, lg: usize) -> Self {
        let nodes = values
            .into_iter()
            .enumerate()
            .map(|(id, value)| Node::new(id, value))
            .collect();
        
        Self {
            nodes,
            edges: vec![Vec::new(); n],
            depth: vec![0; n],
            par: vec![None; n],
            lca_lift: vec![vec![None; lg]; n],
            lg,
            initialized: false,
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

    pub(crate) fn depth(&self) -> &[usize] {
        &self.depth
    }

    pub(crate) fn par(&self) -> &[Option<usize>] {
        &self.par
    }

    /// Initialize the tree structure (call after adding all edges)
    /// 
    /// # Arguments
    /// * `root` - Root node index
    pub fn init(&mut self, root: usize) {
        self.lca_dfs(root, None);
        self.initialized = true;
    }

    fn lca_dfs(&mut self, v: usize, par: Option<usize>) {
        self.lca_lift[v][0] = par;
        self.par[v] = par;
        self.depth[v] = if let Some(p) = par { self.depth[p] + 1 } else { 0 };

        for i in 1..self.lg {
            if let Some(prev) = self.lca_lift[v][i - 1] {
                self.lca_lift[v][i] = self.lca_lift[prev][i - 1];
            } else {
                self.lca_lift[v][i] = None;
            }
        }

        let edges_v = self.edges[v].clone();
        for x in edges_v {
            if Some(x) != par {
                self.lca_dfs(x, Some(v));
            }
        }
    }

    /// Find the lowest common ancestor of two nodes
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }

        let d = self.depth[a] - self.depth[b];
        let mut v = self.get_kth_ancestor(a, d);
        
        if v == b {
            return v;
        }

        for i in (0..self.lg).rev() {
            if self.lca_lift[v][i] != self.lca_lift[b][i] {
                if let Some(v_lift) = self.lca_lift[v][i] {
                    v = v_lift;
                }
                if let Some(b_lift) = self.lca_lift[b][i] {
                    b = b_lift;
                }
            }
        }

        self.lca_lift[b][0].unwrap_or(b)
    }

    /// Get the k-th ancestor of node v
    pub fn get_kth_ancestor(&self, mut v: usize, mut k: usize) -> usize {
        for i in (0..self.lg).rev() {
            if v == usize::MAX {
                return v;
            }
            if (1 << i) <= k {
                if let Some(lift) = self.lca_lift[v][i] {
                    v = lift;
                    k -= 1 << i;
                } else {
                    return usize::MAX;
                }
            }
        }
        v
    }

    /// Get the depth of a node
    pub fn get_depth(&self, node: usize) -> usize {
        self.depth[node]
    }

    /// Get the parent of a node
    pub fn get_parent(&self, node: usize) -> Option<usize> {
        self.par[node]
    }
}
