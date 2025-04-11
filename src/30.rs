fn main() {
    // Example usage of the function
    let result = add(3, 5);
    println!("The sum is: {}", result);
}

/// Adds two numbers.
/// 
/// # Arguments
/// * `a` - The first number to be added.
/// * `b` - The second number to be added.
fn add(a: i32, b: i32) -> i32 {
    a + b
}
