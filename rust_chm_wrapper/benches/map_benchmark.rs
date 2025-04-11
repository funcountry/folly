use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::prelude::*;
use rust_chm_wrapper::FollyMap; // Import your map wrapper
use std::sync::Arc; // Removed Mutex
use std::thread;

const MAP_SIZE: u64 = 1_000_000; // Number of elements to pre-populate
const OPERATIONS_PER_THREAD: u64 = 100_000; // Number of operations each thread performs

// Function simulating the find-and-replace workload for a single thread
// Takes Arc<FollyMap> directly, no Mutex needed.
fn find_replace_task(map: &Arc<FollyMap>, rng: &mut impl Rng, num_ops: u64) {
    // Call methods directly on the shared reference (&Arc<FollyMap> derefs to &FollyMap)
    for _ in 0..num_ops {
        let key = rng.gen_range(0..MAP_SIZE);
        let new_value = rng.gen::<u64>(); // Generate a new value for replacement

        // Find the key (takes &self)
        if map.find(key).is_some() {
            // If found, replace the value (insert takes &self)
            // Note: This isn't truly atomic find-and-replace, another thread
            // could erase between find and insert. Folly's insert_or_assign
            // would be better if exposed. For this benchmark, we proceed.
            map.insert(key, new_value);
        }
        // If not found, we do nothing in this specific benchmark variant

        black_box(()); // Prevent loop optimization
    }
}


fn concurrent_find_replace_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ConcurrentFindReplace");
    group.throughput(Throughput::Elements(OPERATIONS_PER_THREAD)); // Measure throughput per operation

    // --- Benchmark Setup ---
    // Create map mutably first for setup
    let mut map_for_setup = FollyMap::new();
    println!(
        "Pre-populating map with {} elements for benchmark...",
        MAP_SIZE
    );
    {
        let mut rng = StdRng::seed_from_u64(42); // Use fixed seed for reproducibility
        for i in 0..MAP_SIZE {
            map_for_setup.insert(i, rng.gen());
        }
    }
    // Now wrap in Arc for sharing
    let map_arc = Arc::new(map_for_setup);
    println!("Map pre-population complete.");

    // --- Single Thread Benchmark ---
    // Clone the Arc for the single-thread bench run
    let map_arc_single = Arc::clone(&map_arc);
    group.bench_function(BenchmarkId::new("SingleThread", OPERATIONS_PER_THREAD), |b| {
        b.iter(|| {
            let mut rng = StdRng::seed_from_u64(123); // Different seed for benchmark run
            // Pass the Arc directly
            find_replace_task(black_box(&map_arc_single), black_box(&mut rng), OPERATIONS_PER_THREAD);
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
                // Clone the Arc for the scope
                let map_scope_clone = Arc::clone(&map_arc);
                thread::scope(|s| {
                    for i in 0..num_threads {
                        // Clone the Arc for the thread
                        let map_thread_clone = Arc::clone(&map_scope_clone);
                        s.spawn(move || {
                            let mut rng = StdRng::seed_from_u64(456 + i as u64); // Seed per thread
                            // Pass the Arc directly
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
