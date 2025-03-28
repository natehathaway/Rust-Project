use std::io; // Import the standard input/output library

/// Function to generate Fibonacci numbers up to a given limit
/// 
/// # Arguments
/// 
/// * `limit` - The maximum number up to which Fibonacci sequence should be generated
/// 
/// # Returns
/// 
/// A vector containing Fibonacci sequence numbers up to the specified limit
fn generate_fibonacci(limit: u64) -> Vec<u64> {
    let mut sequence = Vec::new(); // Initialize an empty vector to store Fibonacci numbers
    
    let (mut a, mut b) = (0, 1); // Start with the first two Fibonacci numbers
    
    while a <= limit { // Continue generating numbers until the limit is reached
        sequence.push(a); // Add the current number to the sequence
        
        let next = a + b; // Compute the next Fibonacci number
        a = b; // Move to the next number in the sequence
        b = next; // Update the next number
    }
    
    sequence // Return the generated Fibonacci sequence
}

/// Main function - Entry point of the program
fn main() {
    println!("Fibonacci Sequence Generator"); // Display program title
    
    loop {
        // Prompt the user to enter a limit
        println!("Enter the maximum value for Fibonacci sequence (Enter 0 to exit):");
        
        let mut input = String::new(); // Create a mutable string to store user input
        io::stdin().read_line(&mut input).expect("Failed to read input"); // Read input from the user
        
        // Convert input string to an unsigned 64-bit integer
        let limit: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a positive number.");
                continue;
            }
        };
        
        if limit == 0 {
            println!("Exiting program.");
            break;
        }
        
        // Generate Fibonacci sequence up to the specified limit
        let fibonacci_sequence = generate_fibonacci(limit);
        
        // Display the generated Fibonacci sequence
        println!("Fibonacci sequence up to {}:", limit);
        for num in &fibonacci_sequence {
            print!("{} ", num);
        }
        println!(); // Print a newline for formatting
    }
}
// End of the program
