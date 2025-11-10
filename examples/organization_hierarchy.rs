/// Organization Hierarchy Example: Company Structure and Reporting
/// 
/// This example demonstrates how to use Halide for:
/// - Finding reporting chains in an organization
/// - Calculating total salary costs along a department
/// - Finding maximum authority level in a chain
/// - Updating employee information and propagating changes

use halide::{Halide, CombineFn};

#[derive(Clone)]
struct SalarySumCombine;
impl CombineFn<u64> for SalarySumCombine {
    // Sum salaries along reporting chain
    fn combine(&self, a: u64, b: u64) -> u64 {
        a + b
    }
}

#[derive(Clone)]
struct MaxLevelCombine;
impl CombineFn<u64> for MaxLevelCombine {
    // Find maximum authority level
    fn combine(&self, a: u64, b: u64) -> u64 {
        a.max(b)
    }
}

#[derive(Clone)]
struct EmployeeCountCombine;
impl CombineFn<u64> for EmployeeCountCombine {
    // Count employees (each node represents 1 employee)
    fn combine(&self, a: u64, b: u64) -> u64 {
        a + b
    }
}

fn main() {

    // Organization structure:
    // 0: CEO
    // 1: CTO
    // 2: CFO
    // 3: VP Engineering
    // 4: VP Product
    // 5: Director Finance
    // 6: Senior Manager Engineering
    // 7: Manager Engineering
    // 8: Senior Engineer
    // 9: Engineer

    // Annual salaries (in thousands of dollars)
    let salaries = vec![
        500u64, // CEO
        300,    // CTO
        280,    // CFO
        200,    // VP Engineering
        180,    // VP Product
        150,    // Director Finance
        120,    // Senior Manager Engineering
        100,    // Manager Engineering
        90,     // Senior Engineer
        70,     // Engineer
    ];

    let mut org_hierarchy = Halide::new(salaries.clone(), 4, SalarySumCombine, 0u64);
    
    // Build organizational hierarchy
    // CEO -> CTO, CFO
    org_hierarchy.add_edge(0, 1); // CEO -> CTO
    org_hierarchy.add_edge(0, 2); // CEO -> CFO
    
    // CTO -> VP Engineering, VP Product
    org_hierarchy.add_edge(1, 3); // CTO -> VP Engineering
    org_hierarchy.add_edge(1, 4); // CTO -> VP Product
    
    // CFO -> Director Finance
    org_hierarchy.add_edge(2, 5); // CFO -> Director Finance
    
    // VP Engineering -> Senior Manager Engineering
    org_hierarchy.add_edge(3, 6); // VP Engineering -> Senior Manager
    
    // Senior Manager -> Manager Engineering
    org_hierarchy.add_edge(6, 7); // Senior Manager -> Manager
    
    // Manager -> Senior Engineer, Engineer
    org_hierarchy.add_edge(7, 8); // Manager -> Senior Engineer
    org_hierarchy.add_edge(7, 9); // Manager -> Engineer
    
    org_hierarchy.init(0);

    let chain_salary = org_hierarchy.query(9, 0);
    println!("Total salary cost along Engineer->CEO chain: ${}K", chain_salary);

    let engineering_cost = org_hierarchy.query(1, 9);
    let finance_cost = org_hierarchy.query(2, 5);
    println!("Engineering dept cost: ${}K, Finance dept cost: ${}K\n", engineering_cost, finance_cost);

    // Authority levels
    let authority_levels = vec![10u64, 8, 8, 6, 6, 5, 4, 3, 2, 1];
    let mut authority_hierarchy = Halide::new(authority_levels.clone(), 4, MaxLevelCombine, 0u64);
    authority_hierarchy.add_edge(0, 1);
    authority_hierarchy.add_edge(0, 2);
    authority_hierarchy.add_edge(1, 3);
    authority_hierarchy.add_edge(1, 4);
    authority_hierarchy.add_edge(2, 5);
    authority_hierarchy.add_edge(3, 6);
    authority_hierarchy.add_edge(6, 7);
    authority_hierarchy.add_edge(7, 8);
    authority_hierarchy.add_edge(7, 9);
    authority_hierarchy.init(0);

    let max_authority = authority_hierarchy.query(9, 1);
    println!("Maximum authority in Engineering chain: {}\n", max_authority);

    // Salary adjustments
    let old_engineering_cost = engineering_cost;
    org_hierarchy.update(1, 9, (salaries[1] as f64 * 1.1) as u64);
    org_hierarchy.update(3, 3, (salaries[3] as f64 * 1.1) as u64);
    org_hierarchy.update(6, 6, (salaries[6] as f64 * 1.1) as u64);
    org_hierarchy.update(7, 7, (salaries[7] as f64 * 1.1) as u64);
    org_hierarchy.update(8, 8, (salaries[8] as f64 * 1.1) as u64);
    org_hierarchy.update(9, 9, (salaries[9] as f64 * 1.1) as u64);
    
    let new_engineering_cost = org_hierarchy.query(1, 9);
    println!("After 10% raise: ${}K (increase: ${}K)", new_engineering_cost, new_engineering_cost - old_engineering_cost);
}

