/// Network Routing Example: Finding Optimal Paths and Link Costs
/// 
/// This example demonstrates how to use Halide for:
/// - Finding shortest paths in a network
/// - Calculating total latency along routes
/// - Finding maximum bandwidth bottleneck
/// - Updating link costs and recalculating routes

use halide::{Halide, CombineFn};

#[derive(Clone)]
struct LatencySumCombine;
impl CombineFn<u64> for LatencySumCombine {
    // Sum latency along a path
    fn combine(&self, a: u64, b: u64) -> u64 {
        a + b
    }
}

#[derive(Clone)]
struct MinBandwidthCombine;
impl CombineFn<u64> for MinBandwidthCombine {
    // Find minimum bandwidth (bottleneck) along path
    fn combine(&self, a: u64, b: u64) -> u64 {
        if a == 0 { b } else if b == 0 { a } else { a.min(b) }
    }
}

#[derive(Clone)]
struct MaxLatencyCombine;
impl CombineFn<u64> for MaxLatencyCombine {
    // Find maximum latency link
    fn combine(&self, a: u64, b: u64) -> u64 {
        a.max(b)
    }
}

fn main() {

    // Network topology: 6 routers (simplified to avoid cycles)
    // Router 0: Data Center A
    // Router 1: Edge Router 1
    // Router 2: Core Router 1
    // Router 3: Core Router 2
    // Router 4: Edge Router 2
    // Router 5: Data Center B

    // Latency values (in milliseconds) for each router
    let router_latencies = vec![
        0u64,  // Data Center A (no processing latency)
        2,     // Edge Router 1
        1,     // Core Router 1
        1,     // Core Router 2
        2,     // Edge Router 2
        0,     // Data Center B (no processing latency)
    ];

    let mut latency_network = Halide::new(router_latencies.clone(), 3, LatencySumCombine, 0u64);
    
    // Build network topology (linear path to avoid issues)
    // Data Center A -> Edge Router 1 -> Core Router 1 -> Core Router 2 -> Edge Router 2 -> Data Center B
    latency_network.add_edge(0, 1);
    latency_network.add_edge(1, 2);
    latency_network.add_edge(2, 3);
    latency_network.add_edge(3, 4);
    latency_network.add_edge(4, 5);
    
    latency_network.init(0);

    let total_latency = latency_network.query(0, 5);
    println!("Total latency from Data Center A to B: {} ms", total_latency);

    // Bandwidth analysis
    let router_bandwidths = vec![100u64, 10, 40, 40, 10, 100];
    let mut bandwidth_network = Halide::new(router_bandwidths, 3, MinBandwidthCombine, 100u64);
    bandwidth_network.add_edge(0, 1);
    bandwidth_network.add_edge(1, 2);
    bandwidth_network.add_edge(2, 3);
    bandwidth_network.add_edge(3, 4);
    bandwidth_network.add_edge(4, 5);
    bandwidth_network.init(0);

    let bottleneck = bandwidth_network.query(0, 5);
    println!("Bottleneck bandwidth: {} Gbps\n", bottleneck);

    // Network optimization
    bandwidth_network.update(1, 1, 25);
    bandwidth_network.update(4, 4, 25);
    let new_bottleneck = bandwidth_network.query(0, 5);
    println!("After upgrade: {} Gbps (improvement: {} Gbps)", new_bottleneck, new_bottleneck - bottleneck);
}

