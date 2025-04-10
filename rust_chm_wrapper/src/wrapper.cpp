#include "rust_chm_wrapper/include/wrapper.h" // Includes the opaque struct declaration

#include "folly/concurrency/ConcurrentHashMap.h" // Include the actual Folly header here
#include <limits> // For sentinel value
#include <memory> // For std::unique_ptr

// Define the concrete Folly map type we are wrapping.
using FollyMapU64 = folly::ConcurrentHashMapSIMD<uint64_t, uint64_t>;

// Define the actual opaque struct that holds the map.
struct folly_rust_wrapper::ConcurrentHashMapU64Opaque {
    FollyMapU64 map;
    // Constructor can take arguments for map initialization if needed
    ConcurrentHashMapU64Opaque() : map() {}
};

namespace folly_rust_wrapper {

// Sentinel value to indicate "not found" in the find function
const uint64_t NOT_FOUND_SENTINEL = std::numeric_limits<uint64_t>::max();

// Implement the functions declared in the header.

std::unique_ptr<ConcurrentHashMapU64Opaque> new_map() {
    // Create the opaque wrapper on the heap and return a unique_ptr
    return std::make_unique<ConcurrentHashMapU64Opaque>();
}

bool insert(ConcurrentHashMapU64Opaque& wrapper, uint64_t key, uint64_t value) {
    // Access the map inside the wrapper and call its insert method
    return wrapper.map.insert(key, value).second;
}

uint64_t find(const ConcurrentHashMapU64Opaque& wrapper, uint64_t key) {
    // Access the map inside the wrapper and call its find method
    auto it = wrapper.map.find(key);
    if (it == wrapper.map.cend()) {
        return NOT_FOUND_SENTINEL; // Return sentinel if not found
    }
    // Note: iterator access is safe here because find() uses hazard pointers
    return it->second;
}

size_t erase(ConcurrentHashMapU64Opaque& wrapper, uint64_t key) {
    // Access the map inside the wrapper and call its erase method
    return wrapper.map.erase(key);
}

} // namespace folly_rust_wrapper
