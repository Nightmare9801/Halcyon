## Halcyon
## Description
Halcyon is a Rust program that approximates the value of the golden ratio (phi) using the Newton-Raphson method within a few seconds.

## Requirements
- Rust programming language installed on your system

## Usage
- Clone the repository: git clone <repository-url>
- Navigate to the project directory: cd Halcyon
- Compile the Rust program: cargo build --release
- Run the program: cargo run --release

## Algorithm
- The program uses the Newton-Raphson method to approximate the value of the golden ratio (phi).
- It iteratively refines its estimate of phi until a desired level of accuracy is achieved.

## Performance
- Halcyon is designed to approximate phi upto 100000 digits within a minute.
- The Newton-Raphson method allows for efficient convergence to the true value of phi.

## Implementation
- The program leverages Rust's performance capabilities to compute the approximation of phi.
- The Newton-Raphson method is a robust numerical technique for finding roots of equations and is well-suited for this task.

## Future Enhancements
- Implement additional methods for calculating phi to compare and optimize performance.
- Add support for user-defined precision levels and input parameters.

## License
This project is licensed under the MIT License. See the LICENSE file for more information.
