#pragma once

#pragma once

#include <memory> // For std::unique_ptr

// Define the specific type we are wrapping for CXX
// We use uint64_t for simplicity here. Adjust as needed.
// Note: Default template arguments need to be specified if not using the exact defaults.
// Check ConcurrentHashMap.h for the defaults if necessary.
// Using the SIMD version as an example.
#include <folly/concurrency/ConcurrentHashMap.h> // Include full definition for alias
using FollyMapU64 = folly::ConcurrentHashMapSIMD<uint64_t, uint64_t>;


// Define a namespace for our wrapper functions to avoid conflicts
namespace folly_rust_wrapper {

// Alias the Folly map type for CXX to recognize as an opaque type
using ConcurrentHashMapU64 = FollyMapU64;

// Functions to be called from Rust
std::unique_ptr<ConcurrentHashMapU64> new_map();
bool insert(ConcurrentHashMapU64& map, uint64_t key, uint64_t value);
uint64_t find(const ConcurrentHashMapU64& map, uint64_t key); // Returns value or sentinel
size_t erase(ConcurrentHashMapU64& map, uint64_t key); // Returns number erased

} // namespace folly_rust_wrapper
