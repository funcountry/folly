use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::prelude::*;
use rust_chm_wrapper::FollyMap; // Import your map wrapper
use std::sync::{Arc, Mutex};
use std::thread;

const MAP_SIZE: u64 = 1_000_000; // Number of elements to pre-populate
const OPERATIONS_PER_THREAD: u64 = 100_000; // Number of operations each thread performs

// Function simulating the find-and-replace workload for a single thread
fn find_replace_task(map: &Mutex<FollyMap>, rng: &mut impl Rng, num_ops: u64) {
    for _ in 0..num_ops {
        let key = rng.gen_range(0..MAP_SIZE);
        let new_value = rng.gen::<u64>(); // Generate a new value for replacement

        // Lock the map to perform operations
        let mut map_guard = map.lock().unwrap();

        // Find the key
        if map_guard.find(key).is_some() {
            // If found, replace the value (using insert which acts as insert_or_assign)
            map_guard.insert(key, new_value);
        }
        // If not found, we do nothing in this specific benchmark variant
        // (Could add an insert here for a different workload)

        // Mutex guard is dropped here, unlocking the map
        black_box(()); // Prevent loop optimization
    }
}

fn concurrent_find_replace_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ConcurrentFindReplace");
    group.throughput(Throughput::Elements(OPERATIONS_PER_THREAD)); // Measure throughput per operation

    // --- Benchmark Setup ---
    let map = Arc::new(Mutex::new(FollyMap::new()));
    println!(
        "Pre-populating map with {} elements for benchmark...",
        MAP_SIZE
    );
    {
        let mut map_guard = map.lock().unwrap();
        let mut rng = StdRng::seed_from_u64(42); // Use fixed seed for reproducibility
        for i in 0..MAP_SIZE {
            map_guard.insert(i, rng.gen());
        }
    }
    println!("Map pre-population complete.");

    // --- Single Thread Benchmark ---
    group.bench_function(BenchmarkId::new("SingleThread", OPERATIONS_PER_THREAD), |b| {
        b.iter(|| {
            let mut rng = StdRng::seed_from_u64(123); // Different seed for benchmark run
            find_replace_task(black_box(&map), black_box(&mut rng), OPERATIONS_PER_THREAD);
        });
    });

    // --- Multi Thread Benchmark ---
    let num_threads = thread::available_parallelism().map_or(4, |n| n.get()); // Use available cores or default to 4
    let total_ops = OPERATIONS_PER_THREAD * num_threads as u64;
    group.throughput(Throughput::Elements(total_ops)); // Adjust throughput for total operations

    group.bench_function(
        BenchmarkId::new("MultiThread", format!("{}_threads", num_threads)),
        |b| {
            b.iter(|| {
                let map_clone = Arc::clone(&map);
                thread::scope(|s| {
                    for i in 0..num_threads {
                        let map_thread_clone = Arc::clone(&map_clone);
                        s.spawn(move || {
                            let mut rng = StdRng::seed_from_u64(456 + i as u64); // Seed per thread
                            find_replace_task(
                                black_box(&map_thread_clone),
                                black_box(&mut rng),
                                OPERATIONS_PER_THREAD,
                            );
                        });
                    }
                });
            });
        },
    );

    group.finish();
}

criterion_group!(benches, concurrent_find_replace_benchmark);
criterion_main!(benches);
