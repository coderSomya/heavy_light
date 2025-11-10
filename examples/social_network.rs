/// Social Network Example: Mutual Friends and Friend Recommendations
/// 
/// This example demonstrates how to use Halide for:
/// - Finding mutual friends between two users
/// - Recommending friends based on connection paths
/// - Finding the shortest connection path between users
/// - Analyzing friend network statistics

use halide::{Halide, CombineFn};

#[derive(Clone)]
struct FriendSetCombine;
impl CombineFn<u64> for FriendSetCombine {
    // Using bitwise OR to combine friend sets (represented as bitmasks)
    fn combine(&self, a: u64, b: u64) -> u64 {
        a | b
    }
}

fn main() {

    // Create a social network with 8 users
    // Each user has a unique ID and a friend set (bitmask)
    let n = 8;
    let mut friend_sets = vec![0u64; n];
    
    // Initialize friend sets (each user is friends with themselves)
    for i in 0..n {
        friend_sets[i] = 1u64 << i;
    }

    // Create Halide instance for friend set queries
    let mut network = Halide::new(friend_sets.clone(), 3, FriendSetCombine, 0u64);
    
    // Build friendship connections (simpler tree structure)
    // User 0 is friends with 1, 2
    network.add_edge(0, 1);
    network.add_edge(0, 2);
    
    // User 1 is friends with 3, 4
    network.add_edge(1, 3);
    network.add_edge(1, 4);
    
    // User 2 is friends with 5, 6
    network.add_edge(2, 5);
    network.add_edge(2, 6);
    
    // User 3 is friends with 7
    network.add_edge(3, 7);
    
    network.init(0);

    // Find mutual friends
    let user3_friends = network.get_node(3).unwrap().value();
    let user4_friends = network.get_node(4).unwrap().value();
    let mutual = *user3_friends & *user4_friends;
    println!("Mutual friends between user 3 and 4: {}\n", mutual.count_ones());

    // Friend recommendations
    let tree = network.tree();
    let lca = tree.lca(3, 5);
    let depth3 = tree.get_depth(3);
    let depth5 = tree.get_depth(5);
    let depth_lca = tree.get_depth(lca);
    let path_length = depth3 + depth5 - 2 * depth_lca;
    println!("Connection path length between user 3 and 5: {} hops\n", path_length);
}
