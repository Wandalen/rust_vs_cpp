use std::io;

/// Enum to represent a calculation operation
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Function to perform a calculation and return an Option result
fn calculate(op: Operation, a: f64, b: f64) -> Option<f64> {
    match op {
        Operation::Add => Some(a + b),
        Operation::Subtract => Some(a - b),
        Operation::Multiply => Some(a * b),
        Operation::Divide => (b != 0.0).then(|| a / b),
    }
}

impl std::str::FromStr for Operation {
    type Err = io::Error;
    fn from_str(op: &str) -> Result<Self, Self::Err> {
        match op {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "unexpected operation")),
        }
    }
}

fn main() {
    println!("Enter two numbers and an operation (+, -, *, /):");
    let input = io::stdin().lines().next().unwrap().expect("Failed to read input");

    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
        println!("Invalid input");
        return;
    };

    let num1: f64 = parts[0].parse().expect("Invalid 1st number");
    let num2: f64 = parts[1].parse().expect("Invalid 2nd number");
    let operation: Operation = parts[2].parse().unwrap();

    match calculate(operation, num1, num2) {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: Division by zero or invalid operation"),
    }
}
