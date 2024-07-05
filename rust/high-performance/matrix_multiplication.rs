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
    // Uncomment the line below to print the resulting matrix
    // println!("{:?}", c);
}

fn generate_matrix(rows: usize, cols: usize) -> Vec<Vec<f64>> {
    let mut matrix = vec![vec![0.0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            matrix[i][j] = (i * j) as f64; // Simple deterministic value
        }
    }
    matrix
}

fn multiply_matrices(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows = a.len();
    let cols = b[0].len();
    let mut c = vec![vec![0.0; cols]; rows];

    let threads: Vec<_> = (0..rows).map(|i| {
        let a_row = a[i].clone();
        let b = b.to_vec();
        let mut c_row = c[i].clone();
        thread::spawn(move || {
            for j in 0..cols {
                c_row[j] = (0..b.len()).map(|k| a_row[k] * b[k][j]).sum();
            }
            c_row
        })
    }).collect();

    for (i, thread) in threads.into_iter().enumerate() {
        c[i] = thread.join().unwrap();
    }

    c
}

