pub mod node;
pub mod segment_tree;
pub mod tree;

pub use segment_tree::CombineFn;
pub use node::Node;
pub use tree::Tree;
use segment_tree::{SegmentTree, DefaultLazyApply, DefaultLazyFunc};

/// Heavy-Light Decomposition structure for tree path queries and updates
pub struct Halide<T, C>
where
    T: Clone + Copy + Default + PartialEq,
    C: CombineFn<T>,
{
    tree: Tree<T>,
    bigchild: Vec<Option<usize>>,
    sz: Vec<usize>,
    chain: Vec<usize>,
    label: Vec<usize>,
    label_time: usize,
    seg_tree: SegmentTree<T, C, DefaultLazyApply, DefaultLazyFunc>,
    combine_fn: C,
    sentinel: T,
}

impl<T, C> Halide<T, C>
where
    T: Clone + Copy + Default + PartialEq,
    C: CombineFn<T> + Clone,
{
    /// Create a new Halide instance
    /// 
    /// # Arguments
    /// * `values` - Initial values for each node (index corresponds to node id)
    /// * `lg` - Logarithm base 2 of maximum depth (for binary lifting)
    /// * `combine_fn` - Function to combine two segment tree values
    /// * `sentinel` - Sentinel value for segment tree queries (identity element for combine)
    pub fn new(values: Vec<T>, lg: usize, combine_fn: C, sentinel: T) -> Self {
        let n = values.len();
        let tree = Tree::new(n, values, lg);
        
        let lazy_apply = DefaultLazyApply;
        let lazy_func = DefaultLazyFunc;
        let lazy_sentinel = None;
        let seg_tree = SegmentTree::new(n, combine_fn.clone(), lazy_apply, lazy_func, sentinel, lazy_sentinel);
        
        Self {
            tree,
            bigchild: vec![None; n],
            sz: vec![0; n],
            chain: (0..n).collect(),
            label: vec![0; n],
            label_time: 0,
            seg_tree,
            combine_fn,
            sentinel,
        }
    }

    /// Add an undirected edge between nodes u and v
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.tree.add_edge(u, v);
    }

    /// Initialize the tree structure (call after adding all edges)
    /// 
    /// # Arguments
    /// * `root` - Root node index (default: 0)
    pub fn init(&mut self, root: usize) {
        // Initialize tree (builds LCA structure, depth, parent)
        self.tree.init(root);

        // Compute subtree sizes and identify heavy children
        self.dfs_size(root, None);

        // Compute chains
        self.dfs_chains(root, None);

        // Label nodes and initialize segment tree
        self.label_time = 0;
        self.dfs_labels(root, None);
    }

    fn dfs_size(&mut self, v: usize, p: Option<usize>) {
        self.sz[v] = 1;
        let mut bigc = None;
        let mut bigv = 0;

        let edges_v = self.tree.get_edges(v).clone();
        for x in edges_v {
            if Some(x) != p {
                self.dfs_size(x, Some(v));
                self.sz[v] += self.sz[x];
                if self.sz[x] > bigv {
                    bigc = Some(x);
                    bigv = self.sz[x];
                }
            }
        }

        self.bigchild[v] = bigc;
    }

    fn dfs_chains(&mut self, v: usize, p: Option<usize>) {
        if let Some(bc) = self.bigchild[v] {
            self.chain[bc] = self.chain[v];
        }

        let edges_v = self.tree.get_edges(v).clone();
        for x in edges_v {
            if Some(x) != p {
                self.dfs_chains(x, Some(v));
            }
        }
    }

    fn dfs_labels(&mut self, v: usize, p: Option<usize>) {
        self.label[v] = self.label_time;
        self.label_time += 1;
        
        if let Some(node) = self.tree.get_node(v) {
            self.seg_tree.point_update(self.label[v], *node.value());
        }

        if let Some(bc) = self.bigchild[v] {
            self.dfs_labels(bc, Some(v));
        }

        let edges_v = self.tree.get_edges(v).clone();
        for x in edges_v {
            if Some(x) != p && Some(x) != self.bigchild[v] {
                self.dfs_labels(x, Some(v));
            }
        }
    }

    /// Query a chain from v to p (excludes p)
    fn query_chain(&mut self, mut v: usize, p: usize) -> T {
        let mut val = self.sentinel;
        let depth = self.tree.depth();
        let par = self.tree.par();
        
        while depth[p] < depth[v] {
            let mut top = self.chain[v];
            if depth[top] <= depth[p] {
                let diff = depth[v] - depth[p];
                if diff > 0 {
                    top = self.tree.get_kth_ancestor(v, diff - 1);
                    if top == usize::MAX {
                        break;
                    }
                } else {
                    break;
                }
            }
            val = self.combine_fn.combine(val, self.seg_tree.query(self.label[top], self.label[v]));
            if let Some(parent) = par[top] {
                v = parent;
            } else {
                break;
            }
        }
        
        val
    }

    /// Query the path between nodes u and v
    pub fn query(&mut self, u: usize, v: usize) -> T {
        let lc = self.tree.lca(u, v);
        let val1 = self.query_chain(u, lc);
        let val2 = self.query_chain(v, lc);
        let combined = self.combine_fn.combine(val1, val2);
        let lc_val = self.seg_tree.query(self.label[lc], self.label[lc]);
        self.combine_fn.combine(combined, lc_val)
    }

    /// Update a chain from v to p (excludes p)
    fn update_chain(&mut self, mut v: usize, p: usize, val: T) {
        let depth = self.tree.depth();
        let par = self.tree.par();
        
        while depth[p] < depth[v] {
            let mut top = self.chain[v];
            if depth[top] <= depth[p] {
                let diff = depth[v] - depth[p];
                if diff > 0 {
                    top = self.tree.get_kth_ancestor(v, diff - 1);
                    if top == usize::MAX {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.seg_tree.update(self.label[top], self.label[v], val);
            if let Some(parent) = par[top] {
                v = parent;
            } else {
                break;
            }
        }
    }

    /// Update the path between nodes u and v
    pub fn update(&mut self, u: usize, v: usize, val: T) {
        let lc = self.tree.lca(u, v);
        self.update_chain(u, lc, val);
        self.update_chain(v, lc, val);
        self.seg_tree.update(self.label[lc], self.label[lc], val);
    }

    /// Get the label (position in segment tree) of a node
    pub fn get_label(&self, node: usize) -> usize {
        self.label[node]
    }

    /// Get a reference to a node
    pub fn get_node(&self, id: usize) -> Option<&Node<T>> {
        self.tree.get_node(id)
    }

    /// Get a mutable reference to a node
    pub fn get_node_mut(&mut self, id: usize) -> Option<&mut Node<T>> {
        self.tree.get_node_mut(id)
    }

    /// Get a reference to the underlying tree
    pub fn tree(&self) -> &Tree<T> {
        &self.tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example combine functions
    #[derive(Clone)]
    struct XorCombine;
    impl CombineFn<u64> for XorCombine {
        fn combine(&self, a: u64, b: u64) -> u64 {
            a ^ b
        }
    }

    #[derive(Clone)]
    struct SumCombine;
    impl CombineFn<u64> for SumCombine {
        fn combine(&self, a: u64, b: u64) -> u64 {
            a + b
        }
    }

    #[test]
    fn test_basic_halide() {
        let values = vec![1u64, 2, 3, 4, 5];
        let lg = 3;
        let combine = XorCombine;
        
        let mut halide = Halide::new(
            values,
            lg,
            combine,
            0u64, // sentinel
        );

        halide.add_edge(0, 1);
        halide.add_edge(0, 2);
        halide.add_edge(1, 3);
        halide.add_edge(1, 4);

        halide.init(0); // root is node 0

        // Test LCA
        let lca = halide.tree().lca(3, 4);
        assert_eq!(lca, 1);
    }
}
