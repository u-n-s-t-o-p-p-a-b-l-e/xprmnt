use std::time::Instant;
use std::thread;

fn main() {
    let size = 1000;
    let a = generate_matrix(size, size);
    let b = generate_matrix(size, size);

    let start = Instant::now();
    let c = multiply_matrices(&a, &b);
    let duration = start.elapsed();
}
