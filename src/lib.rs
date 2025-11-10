pub mod node;
pub mod segment_tree;

pub use segment_tree::{CombineFn, LazyApplyFn, LazyFunc};
use segment_tree::SegmentTree;

/// Heavy-Light Decomposition structure for tree path queries and updates
pub struct HeavyLightDecomposition<T, C, LA, LF>
where
    T: Clone + Copy + Default + PartialEq,
    C: CombineFn<T>,
    LA: LazyApplyFn<T>,
    LF: LazyFunc<T>,
{
    n: usize,
    lg: usize,
    edges: Vec<Vec<usize>>,
    bigchild: Vec<Option<usize>>,
    sz: Vec<usize>,
    depth: Vec<usize>,
    chain: Vec<usize>,
    label: Vec<usize>,
    label_time: usize,
    par: Vec<Option<usize>>,
    lca_lift: Vec<Vec<Option<usize>>>,
    seg_tree: SegmentTree<T, C, LA, LF>,
    combine_fn: C,
    sentinel: T,
}

impl<T, C, LA, LF> HeavyLightDecomposition<T, C, LA, LF>
where
    T: Clone + Copy + Default + PartialEq,
    C: CombineFn<T> + Clone,
    LA: LazyApplyFn<T> + Clone,
    LF: LazyFunc<T> + Clone,
{
    /// Create a new HLD instance
    /// 
    /// # Arguments
    /// * `n` - Number of nodes in the tree
    /// * `lg` - Logarithm base 2 of maximum depth (for binary lifting)
    /// * `combine_fn` - Function to combine two segment tree values
    /// * `lazy_apply_fn` - Function to apply lazy updates
    /// * `lazy_func` - Function to apply lazy value to current value
    /// * `sentinel` - Sentinel value for segment tree queries
    /// * `lazy_sentinel` - Sentinel value for lazy propagation
    pub fn new(
        n: usize,
        lg: usize,
        combine_fn: C,
        lazy_apply_fn: LA,
        lazy_func: LF,
        sentinel: T,
        lazy_sentinel: Option<T>,
    ) -> Self {
        let seg_tree = SegmentTree::new(n, combine_fn.clone(), lazy_apply_fn.clone(), lazy_func.clone(), sentinel, lazy_sentinel);
        
        Self {
            n,
            lg,
            edges: vec![Vec::new(); n],
            bigchild: vec![None; n],
            sz: vec![0; n],
            depth: vec![0; n],
            chain: (0..n).collect(),
            label: vec![0; n],
            label_time: 0,
            par: vec![None; n],
            lca_lift: vec![vec![None; lg]; n],
            seg_tree,
            combine_fn,
            sentinel,
        }
    }

    /// Initialize arrays (call this before adding edges)
    pub fn init_arrays(&mut self) {
        for i in 0..self.n {
            self.edges[i].clear();
            self.chain[i] = i;
        }
    }

    /// Add an undirected edge between nodes u and v
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.edges[u].push(v);
        self.edges[v].push(u);
    }

    /// Initialize the tree structure with initial values
    /// 
    /// # Arguments
    /// * `arr` - Initial values for each node
    /// * `root` - Root node index (default: 0)
    pub fn init_tree(&mut self, arr: &[T], root: usize) {
        // Build LCA structure
        self.lca_dfs(root, None);

        // Compute subtree sizes and identify heavy children
        self.dfs_size(root, None, 0);

        // Compute chains
        self.dfs_chains(root, None);

        // Label nodes and initialize segment tree
        self.label_time = 0;
        self.dfs_labels(arr, root, None);
    }

    fn lca_dfs(&mut self, v: usize, par: Option<usize>) {
        self.lca_lift[v][0] = par;

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

    fn dfs_size(&mut self, v: usize, p: Option<usize>, d: usize) {
        self.sz[v] = 1;
        self.depth[v] = d;
        self.par[v] = p;
        let mut bigc = None;
        let mut bigv = 0;

        let edges_v = self.edges[v].clone();
        for x in edges_v {
            if Some(x) != p {
                self.dfs_size(x, Some(v), d + 1);
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

        let edges_v = self.edges[v].clone();
        for x in edges_v {
            if Some(x) != p {
                self.dfs_chains(x, Some(v));
            }
        }
    }

    fn dfs_labels(&mut self, arr: &[T], v: usize, p: Option<usize>) {
        self.label[v] = self.label_time;
        self.label_time += 1;
        self.seg_tree.point_update(self.label[v], arr[v]);

        if let Some(bc) = self.bigchild[v] {
            self.dfs_labels(arr, bc, Some(v));
        }

        let edges_v = self.edges[v].clone();
        for x in edges_v {
            if Some(x) != p && Some(x) != self.bigchild[v] {
                self.dfs_labels(arr, x, Some(v));
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

    /// Query a chain from v to p (excludes p)
    fn query_chain(&mut self, mut v: usize, p: usize) -> T {
        let mut val = self.sentinel;
        
        while self.depth[p] < self.depth[v] {
            let mut top = self.chain[v];
            if self.depth[top] <= self.depth[p] {
                let diff = self.depth[v] - self.depth[p];
                if diff > 0 {
                    top = self.get_kth_ancestor(v, diff - 1);
                    if top == usize::MAX {
                        break;
                    }
                } else {
                    break;
                }
            }
            val = self.combine_fn.combine(val, self.seg_tree.query(self.label[top], self.label[v]));
            if let Some(parent) = self.par[top] {
                v = parent;
            } else {
                break;
            }
        }
        
        val
    }

    /// Query the path between nodes u and v
    pub fn query(&mut self, u: usize, v: usize) -> T {
        let lc = self.lca(u, v);
        let val1 = self.query_chain(u, lc);
        let val2 = self.query_chain(v, lc);
        let combined = self.combine_fn.combine(val1, val2);
        let lc_val = self.seg_tree.query(self.label[lc], self.label[lc]);
        self.combine_fn.combine(combined, lc_val)
    }

    /// Update a chain from v to p (excludes p)
    fn update_chain(&mut self, mut v: usize, p: usize, val: T) {
        while self.depth[p] < self.depth[v] {
            let mut top = self.chain[v];
            if self.depth[top] <= self.depth[p] {
                let diff = self.depth[v] - self.depth[p];
                if diff > 0 {
                    top = self.get_kth_ancestor(v, diff - 1);
                    if top == usize::MAX {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.seg_tree.update(self.label[top], self.label[v], val);
            if let Some(parent) = self.par[top] {
                v = parent;
            } else {
                break;
            }
        }
    }

    /// Update the path between nodes u and v
    pub fn update(&mut self, u: usize, v: usize, val: T) {
        let lc = self.lca(u, v);
        self.update_chain(u, lc, val);
        self.update_chain(v, lc, val);
        self.seg_tree.update(self.label[lc], self.label[lc], val);
    }

    /// Get the label (position in segment tree) of a node
    pub fn get_label(&self, node: usize) -> usize {
        self.label[node]
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

    #[derive(Clone)]
    struct SimpleLazyApply;
    impl LazyApplyFn<u64> for SimpleLazyApply {
        fn apply(&self, _lazy_val: u64, new_val: u64) -> u64 {
            new_val
        }
    }

    #[derive(Clone)]
    struct SimpleLazyFunc;
    impl LazyFunc<u64> for SimpleLazyFunc {
        fn apply(&self, _cur_val: u64, lazy_val: u64, _l: usize, _r: usize) -> u64 {
            lazy_val
        }
    }

    #[test]
    fn test_basic_hld() {
        let n = 5;
        let lg = 3;
        let combine = XorCombine;
        let lazy_apply = SimpleLazyApply;
        let lazy_func = SimpleLazyFunc;
        
        let mut hld = HeavyLightDecomposition::new(
            n,
            lg,
            combine,
            lazy_apply,
            lazy_func,
            0u64,
            Some(0u64),
        );

        hld.init_arrays();
        hld.add_edge(0, 1);
        hld.add_edge(0, 2);
        hld.add_edge(1, 3);
        hld.add_edge(1, 4);

        let arr = vec![1u64, 2, 3, 4, 5];
        hld.init_tree(&arr, 0);

        // Test LCA
        let lca = hld.lca(3, 4);
        assert_eq!(lca, 1);
    }
}
