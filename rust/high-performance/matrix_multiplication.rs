use std::time::Instant;
use std::thread;

fn main() {
    let size = 1000;
    let a = generate_matrix(size, size);
    let b = generate_matrix(size, size);

    let start = Instant::now();
    let c = multiply_matrices(&a, &b);
    let duration = start.elapsed();

    println!("Time taken: {:?}", duration);
    println!("{:?}", c);
}

fn generate_matrix(rows: usize, cols: usize) ->  Vec<Vec<f64>> { 
    let mut matrix = vec![vec![0.0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            matrix[i][j] = (i * j) as f64;
        }
    }
    matrix
}
