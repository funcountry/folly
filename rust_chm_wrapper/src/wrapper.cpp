#include "rust_chm_wrapper/include/wrapper.h"
#include "folly/concurrency/ConcurrentHashMap.h" // Include the actual Folly header
#include <limits> // For sentinel value

namespace folly_rust_wrapper {

// Define the specific type we are wrapping again for clarity in the .cpp file
// Make sure this matches the alias in wrapper.h
using FollyMapU64 = folly::ConcurrentHashMapSIMD<uint64_t, uint64_t>;

// Sentinel value to indicate "not found" in the find function
const uint64_t NOT_FOUND_SENTINEL = std::numeric_limits<uint64_t>::max();

std::unique_ptr<ConcurrentHashMapU64> new_map() {
    // Create the map on the heap and return a unique_ptr
    // You might want to pass initial size parameters here
    return std::make_unique<ConcurrentHashMapU64>();
}

bool insert(ConcurrentHashMapU64& map, uint64_t key, uint64_t value) {
    // insert returns a pair<iterator, bool>
    return map.insert(key, value).second;
}

uint64_t find(const ConcurrentHashMapU64& map, uint64_t key) {
    auto it = map.find(key);
    if (it == map.cend()) {
        return NOT_FOUND_SENTINEL; // Return sentinel if not found
    }
    // Note: iterator access is safe here because find() uses hazard pointers
    return it->second;
}

size_t erase(ConcurrentHashMapU64& map, uint64_t key) {
    // erase returns the number of elements removed (0 or 1)
    return map.erase(key);
}

} // namespace folly_rust_wrapper
