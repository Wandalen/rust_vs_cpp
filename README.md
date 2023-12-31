# rust_vs_cpp

```c++
#include <iostream>
#include <optional>

// Enum to represent a calculation operation
enum class Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
};

// Function to perform a calculation and return an optional result
std::optional<double> calculate(Operation op, double a, double b) {
    switch (op) {
        case Operation::Add:
            return a + b;
        case Operation::Subtract:
            return a - b;
        case Operation::Multiply:
            return a * b;
        case Operation::Divide:
            if (b == 0.0) {
                return std::nullopt; // Error: Division by zero
            }
            return a / b;
    }
    return std::nullopt; // Invalid operation
}

int main() {
    std::cout << "Enter two numbers and an operation (+, -, *, /): ";
    double num1, num2;
    char op;
    std::cin >> num1 >> num2 >> op;

    Operation operation;

    switch (op) {
        case '+':
            operation = Operation::Add;
            break;
        case '-':
            operation = Operation::Subtract;
            break;
        case '*':
            operation = Operation::Multiply;
            break;
        case '/':
            operation = Operation::Divide;
            break;
        default:
            std::cout << "Invalid operation" << std::endl;
            return 1;
    }

    std::optional<double> result = calculate(operation, num1, num2);

    if (result.has_value()) {
        std::cout << "Result: " << result.value() << std::endl;
    } else {
        std::cout << "Error: Division by zero or invalid operation" << std::endl;
    }

    return 0;
}
```

```rust
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

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
```
