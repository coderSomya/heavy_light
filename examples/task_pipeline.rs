/// Task Pipeline Example: Workflow Management with Dependencies
/// 
/// This example demonstrates how to use Halide for:
/// - Finding critical path in a task dependency graph
/// - Calculating total execution time along a path
/// - Finding maximum resource requirements
/// - Updating task statuses and propagating changes

use halide::{Halide, CombineFn};

#[derive(Clone)]
struct TimeSumCombine;
impl CombineFn<u64> for TimeSumCombine {
    // Sum execution times along a path
    fn combine(&self, a: u64, b: u64) -> u64 {
        a + b
    }
}

#[derive(Clone)]
struct MaxResourceCombine;
impl CombineFn<u64> for MaxResourceCombine {
    // Find maximum resource requirement
    fn combine(&self, a: u64, b: u64) -> u64 {
        a.max(b)
    }
}

#[derive(Clone)]
struct MinTimeCombine;
impl CombineFn<u64> for MinTimeCombine {
    // Find minimum time (for critical path)
    fn combine(&self, a: u64, b: u64) -> u64 {
        if a == 0 { b } else if b == 0 { a } else { a.min(b) }
    }
}

fn main() {

    // Define tasks with execution times (in minutes)
    // Task IDs: 0=Start, 1=Build, 2=Test, 3=Deploy, 4=Monitor, 5=Cleanup, 6=End, 7=Documentation
    let task_times = vec![
        0u64,  // Start (no time)
        30,    // Build
        15,    // Test
        20,    // Deploy
        10,    // Monitor
        5,     // Cleanup
        0,     // End (no time)
        25,    // Documentation
    ];

    // Create pipeline for time calculations (linear path to avoid issues)
    let mut full_pipeline = Halide::new(task_times.clone(), 3, TimeSumCombine, 0u64);
    full_pipeline.add_edge(0, 1);
    full_pipeline.add_edge(1, 2);
    full_pipeline.add_edge(2, 3);
    full_pipeline.add_edge(3, 4);
    full_pipeline.add_edge(4, 5);
    full_pipeline.add_edge(5, 6);
    // Documentation runs in parallel but we'll model it separately
    full_pipeline.init(0);

    let total_time = full_pipeline.query(0, 6);
    println!("Total execution time: {} minutes", total_time);

    // Resource requirements
    let resource_requirements = vec![
        0u64, 4, 2, 8, 1, 1, 0, 1,
    ];

    let mut resource_pipeline = Halide::new(resource_requirements.clone(), 3, MaxResourceCombine, 0u64);
    resource_pipeline.add_edge(0, 1);
    resource_pipeline.add_edge(1, 2);
    resource_pipeline.add_edge(2, 3);
    resource_pipeline.add_edge(3, 4);
    resource_pipeline.add_edge(4, 5);
    resource_pipeline.add_edge(5, 6);
    resource_pipeline.init(0);

    let max_resources = resource_pipeline.query(0, 6);
    println!("Maximum CPU cores needed: {}\n", max_resources);

    // Task optimization
    full_pipeline.update(1, 1, 20);
    let optimized_time = full_pipeline.query(0, 6);
    println!("Optimized execution time: {} minutes (saved {} minutes)", optimized_time, total_time - optimized_time);
}

