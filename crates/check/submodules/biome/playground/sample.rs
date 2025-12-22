// Sample Rust file with intentional formatting and linting issues

use std::collections::HashMap;
use std::io::Write; // Unused import

fn main() {
    let x = 5;
    let mut y = 10;

    // Formatting issues: inconsistent spacing
    if x > 0 {
        println!("x is positive");
    }

    // Unused variable
    let unused = 42;

    // Bad formatting
    let numbers = vec![1, 2, 3, 4, 5];

    for i in 0..5 {
        println!("{}", i);
    }

    calculate_sum(x, y);
}

fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

// Unused function
fn unused_function() {
    let x = 1;
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}
