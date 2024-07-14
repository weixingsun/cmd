use std::sync::Arc;
use std::thread;
use std::time::Instant;

const THREADS: usize = 8; // Set the number of threads you want to run in parallel
const BUFFER_SIZE: usize = 1_000_000_000; // Set the buffer size
const ITERATIONS: usize = 100; // Set the number of iterations

fn main() {
    let data = Arc::new(vec![0u8; BUFFER_SIZE]);

    let mut handles = Vec::with_capacity(THREADS);

    let start_time = Instant::now();

    for _ in 0..THREADS {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut local_data = vec![0u8; BUFFER_SIZE];
            for _ in 0..ITERATIONS {
                local_data.copy_from_slice(&data_clone);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_time = start_time.elapsed();
    let bytes_copied = (BUFFER_SIZE * ITERATIONS * THREADS) as u128;
    let bandwidth = bytes_copied * 1_000_000_000 / (elapsed_time.as_nanos());

    println!(
        "Elapsed time: {:.2?}, Bandwidth: {:.2} MB/s",
        elapsed_time,
        (bandwidth as f64) / (1024.0 * 1024.0)
    );
}