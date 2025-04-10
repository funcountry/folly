#pragma once

#pragma once

#include <cstdint> // For uint64_t, size_t
#include <memory>  // For std::unique_ptr

// Include the necessary Folly header for the map definition
#include "folly/concurrency/ConcurrentHashMap.h"

// Define a namespace for our wrapper functions to avoid conflicts
namespace folly_rust_wrapper {

// Define the concrete Folly map type we are wrapping.
// Using SIMD version for potential performance benefits.
using FollyMapU64 = folly::ConcurrentHashMapSIMD<uint64_t, uint64_t>;

// Define the actual opaque struct that holds the map.
// This definition is needed here because Rust uses UniquePtr<OpaqueType>.
struct ConcurrentHashMapU64Opaque {
    FollyMapU64 map;
    // Constructor can take arguments for map initialization if needed
    ConcurrentHashMapU64Opaque() : map() {}
    // Ensure it's not copyable/movable in ways C++ might implicitly generate,
    // matching UniquePtr's expectations.
    ConcurrentHashMapU64Opaque(const ConcurrentHashMapU64Opaque&) = delete;
    ConcurrentHashMapU64Opaque& operator=(const ConcurrentHashMapU64Opaque&) = delete;
    ConcurrentHashMapU64Opaque(ConcurrentHashMapU64Opaque&&) = delete;
    ConcurrentHashMapU64Opaque& operator=(ConcurrentHashMapU64Opaque&&) = delete;
};


// Functions to be called from Rust, operating on the opaque struct
std::unique_ptr<ConcurrentHashMapU64Opaque> new_map();
bool insert(ConcurrentHashMapU64Opaque& map, uint64_t key, uint64_t value);
uint64_t find(const ConcurrentHashMapU64Opaque& map, uint64_t key); // Returns value or sentinel
size_t erase(ConcurrentHashMapU64Opaque& map, uint64_t key); // Returns number erased

} // namespace folly_rust_wrapper
