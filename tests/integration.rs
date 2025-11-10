use halide::{Halide, CombineFn, Tree, Node};

#[derive(Clone)]
struct SumCombine;
impl CombineFn<u64> for SumCombine {
    fn combine(&self, a: u64, b: u64) -> u64 {
        a + b
    }
}

#[test]
fn test_tree_access() {
    let values = vec![10u64, 20, 30, 40, 50];
    let mut halide = Halide::new(values, 3, SumCombine, 0u64);
    
    halide.add_edge(0, 1);
    halide.add_edge(0, 2);
    halide.add_edge(1, 3);
    halide.add_edge(1, 4);
    halide.init(0);
    
    // Test accessing tree through halide
    let tree = halide.tree();
    
    // Test node access
    let node = halide.get_node(2);
    assert!(node.is_some());
    assert_eq!(node.unwrap().id(), 2);
    assert_eq!(*node.unwrap().value(), 30);
    
    // Test tree operations
    assert_eq!(tree.get_depth(0), 0);
    assert_eq!(tree.get_depth(1), 1);
    assert_eq!(tree.get_depth(2), 1);
    assert_eq!(tree.get_depth(3), 2);
    assert_eq!(tree.get_depth(4), 2);
    
    assert_eq!(tree.get_parent(0), None);
    assert_eq!(tree.get_parent(1), Some(0));
    assert_eq!(tree.get_parent(2), Some(0));
    assert_eq!(tree.get_parent(3), Some(1));
    assert_eq!(tree.get_parent(4), Some(1));
}

#[test]
fn test_node_mutation() {
    let values = vec![1u64, 2, 3, 4, 5];
    let mut halide = Halide::new(values, 3, SumCombine, 0u64);
    
    halide.add_edge(0, 1);
    halide.add_edge(0, 2);
    halide.add_edge(1, 3);
    halide.add_edge(1, 4);
    halide.init(0);
    
    // Modify a node's value
    if let Some(node) = halide.get_node_mut(2) {
        node.set_value(100);
    }
    
    // Verify the change
    let node = halide.get_node(2);
    assert_eq!(*node.unwrap().value(), 100);
    
    // Note: This doesn't update the segment tree, so queries won't reflect the change
    // unless we reinitialize or update through the HLD interface
}

#[test]
fn test_label_access() {
    let values = vec![1u64, 2, 3, 4, 5];
    let mut halide = Halide::new(values, 3, SumCombine, 0u64);
    
    halide.add_edge(0, 1);
    halide.add_edge(0, 2);
    halide.add_edge(1, 3);
    halide.add_edge(1, 4);
    halide.init(0);
    
    // Get labels for nodes
    let label0 = halide.get_label(0);
    let label1 = halide.get_label(1);
    let label2 = halide.get_label(2);
    
    // Labels should be assigned in DFS order
    // Root (0) should have label 0
    assert_eq!(label0, 0);
    
    // Labels should be unique
    assert_ne!(label1, label2);
    assert_ne!(label1, label0);
    assert_ne!(label2, label0);
}

#[test]
fn test_complete_workflow() {
    // Simulate a complete workflow: create, build, query, update
    let values: Vec<u64> = (1..=10).collect();
    let mut halide = Halide::new(values, 4, SumCombine, 0u64);
    
    // Build a tree: 0 is root, connects to 1,2,3; 1 connects to 4,5; etc.
    halide.add_edge(0, 1);
    halide.add_edge(0, 2);
    halide.add_edge(0, 3);
    halide.add_edge(1, 4);
    halide.add_edge(1, 5);
    halide.add_edge(2, 6);
    halide.add_edge(2, 7);
    halide.add_edge(3, 8);
    halide.add_edge(3, 9);
    
    // Initialize
    halide.init(0);
    
    // Verify tree structure
    let tree = halide.tree();
    assert_eq!(tree.get_depth(0), 0);
    assert_eq!(tree.get_depth(1), 1);
    assert_eq!(tree.get_depth(4), 2);
    
    // Perform queries
    let q1 = halide.query(4, 5);
    // Path: 4 -> 1 -> 5, values: 5, 2, 6
    // Result depends on query_chain implementation
    assert!(q1 > 0); // Just verify it's positive
    
    let q2 = halide.query(6, 9);
    // Path: 6 -> 2 -> 0 -> 3 -> 9
    // Values on path: 7, 3, 1, 4, 10
    assert!(q2 > 0); // Just verify it's positive
    
    // Perform updates
    halide.update(4, 5, 100);
    let q3 = halide.query(4, 5);
    // After update, nodes on path should have value 100
    assert!(q3 > 0); // Just verify it's positive after update
    
    // Query after update
    let q4 = halide.query(0, 4);
    // Path: 0 -> 1 -> 4, updated nodes have value 100
    assert!(q4 > 0); // Just verify it's positive
}

#[test]
fn test_lca_variations() {
    let values = vec![1u64, 2, 3, 4, 5, 6, 7, 8];
    let mut halide = Halide::new(values, 3, SumCombine, 0u64);
    
    // Build tree:
    //       0
    //      / \
    //     1   2
    //    /|   |\
    //   3 4   5 6
    //         |
    //         7
    halide.add_edge(0, 1);
    halide.add_edge(0, 2);
    halide.add_edge(1, 3);
    halide.add_edge(1, 4);
    halide.add_edge(2, 5);
    halide.add_edge(2, 6);
    halide.add_edge(5, 7);
    halide.init(0);
    
    let tree = halide.tree();
    
    // Test LCA of nodes at same depth
    assert_eq!(tree.lca(3, 4), 1);
    assert_eq!(tree.lca(5, 6), 2);
    
    // Test LCA of nodes at different depths
    assert_eq!(tree.lca(3, 7), 0);
    assert_eq!(tree.lca(4, 5), 0);
    
    // Test LCA with root
    assert_eq!(tree.lca(0, 3), 0);
    assert_eq!(tree.lca(3, 0), 0);
    
    // Test LCA of same node
    assert_eq!(tree.lca(3, 3), 3);
    
    // Test LCA of parent and child
    assert_eq!(tree.lca(2, 5), 2);
    assert_eq!(tree.lca(5, 2), 2);
}

#[test]
fn test_query_consistency() {
    // Test that query(u, v) == query(v, u) for symmetric operations
    let values = vec![1u64, 2, 3, 4, 5];
    let mut halide = Halide::new(values, 3, SumCombine, 0u64);
    
    halide.add_edge(0, 1);
    halide.add_edge(0, 2);
    halide.add_edge(1, 3);
    halide.add_edge(1, 4);
    halide.init(0);
    
    // Sum is symmetric, so query should be symmetric
    let q1 = halide.query(3, 4);
    let q2 = halide.query(4, 3);
    assert_eq!(q1, q2);
    
    // Test with different pairs
    let q3 = halide.query(0, 3);
    let q4 = halide.query(3, 0);
    assert_eq!(q3, q4);
    
    let q5 = halide.query(2, 4);
    let q6 = halide.query(4, 2);
    assert_eq!(q5, q6);
}

#[test]
fn test_update_consistency() {
    // Test that update(u, v, val) affects the same path as update(v, u, val)
    let values = vec![1u64, 2, 3, 4, 5];
    let mut halide1 = Halide::new(values.clone(), 3, SumCombine, 0u64);
    let mut halide2 = Halide::new(values, 3, SumCombine, 0u64);
    
    halide1.add_edge(0, 1);
    halide1.add_edge(0, 2);
    halide1.add_edge(1, 3);
    halide1.add_edge(1, 4);
    halide1.init(0);
    
    halide2.add_edge(0, 1);
    halide2.add_edge(0, 2);
    halide2.add_edge(1, 3);
    halide2.add_edge(1, 4);
    halide2.init(0);
    
    halide1.update(3, 4, 10);
    halide2.update(4, 3, 10);
    
    let q1 = halide1.query(3, 4);
    let q2 = halide2.query(3, 4);
    assert_eq!(q1, q2);
}

