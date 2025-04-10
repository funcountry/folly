#pragma once

#include <cstdint> // For uint64_t, size_t
#include <memory>  // For std::unique_ptr

// Define a namespace for our wrapper functions to avoid conflicts
namespace folly_rust_wrapper {

// Define an opaque struct that will hide the Folly implementation details.
// The actual definition will be in wrapper.cpp
struct ConcurrentHashMapU64Opaque;

// Functions to be called from Rust, operating on the opaque struct
std::unique_ptr<ConcurrentHashMapU64Opaque> new_map();
bool insert(ConcurrentHashMapU64Opaque& map, uint64_t key, uint64_t value);
uint64_t find(const ConcurrentHashMapU64Opaque& map, uint64_t key); // Returns value or sentinel
size_t erase(ConcurrentHashMapU64Opaque& map, uint64_t key); // Returns number erased

} // namespace folly_rust_wrapper
