#pragma once

#pragma once

#include <cstdint> // For uint64_t, uint8_t, size_t
#include <memory>  // For std::unique_ptr

// Forward declarations needed for the specific ConcurrentHashMap instantiation
namespace std {
template <class T> struct hash;
template <class T> struct equal_to;
template <class T> class allocator;
template <class T> class atomic;
class mutex;
} // namespace std

namespace folly {
namespace detail {
namespace concurrenthashmap {
namespace simd {
template <
    typename KeyType,
    typename ValueType,
    uint8_t ShardBits,
    typename HashFn,
    typename KeyEqual,
    typename Allocator,
    template <typename>
    class Atom,
    class Mutex>
class SIMDTable;
} // namespace simd
} // namespace concurrenthashmap
} // namespace detail

// Forward declare the specific ConcurrentHashMap template instantiation
// This matches ConcurrentHashMapSIMD<uint64_t, uint64_t> on platforms supporting it
template <
    typename KeyType,
    typename ValueType,
    typename HashFn,
    typename KeyEqual,
    typename Allocator,
    uint8_t ShardBits,
    template <typename> class Atom,
    class Mutex,
    template <
        typename,
        typename,
        uint8_t,
        typename,
        typename,
        typename,
        template <typename>
        class,
        class>
    class Impl>
class ConcurrentHashMap;

// Define the specific type we are wrapping for CXX, using the forward declaration
using FollyMapU64 = ConcurrentHashMap<
    uint64_t,
    uint64_t,
    std::hash<uint64_t>,
    std::equal_to<uint64_t>,
    std::allocator<uint8_t>,
    8,
    std::atomic,
    std::mutex,
    folly::detail::concurrenthashmap::simd::SIMDTable>;

} // namespace folly

// Define a namespace for our wrapper functions to avoid conflicts
namespace folly_rust_wrapper {

// Use the forward-declared type directly for CXX opaque type
using ConcurrentHashMapU64 = folly::FollyMapU64;

// Functions to be called from Rust
std::unique_ptr<ConcurrentHashMapU64> new_map();
bool insert(ConcurrentHashMapU64& map, uint64_t key, uint64_t value);
uint64_t find(const ConcurrentHashMapU64& map, uint64_t key); // Returns value or sentinel
size_t erase(ConcurrentHashMapU64& map, uint64_t key); // Returns number erased

} // namespace folly_rust_wrapper
