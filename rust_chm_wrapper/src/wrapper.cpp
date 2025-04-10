#include "rust_chm_wrapper/include/wrapper.h" // Now includes only forward declarations

#include "folly/concurrency/ConcurrentHashMap.h" // Include the actual Folly header here
#include <limits> // For sentinel value

namespace folly_rust_wrapper {

// Define the specific type we are wrapping again for use within the C++ impl.
// This must match the type alias used in the header's forward declaration section.
using FollyMapU64 = folly::ConcurrentHashMapSIMD<uint64_t, uint64_t>;

// Verify that the type used in the header (aliased via folly::FollyMapU64)
// is the same as the one we define locally for implementation.
static_assert(std::is_same<ConcurrentHashMapU64, FollyMapU64>::value,
              "Type mismatch between header forward declaration and cpp implementation");


// Sentinel value to indicate "not found" in the find function
const uint64_t NOT_FOUND_SENTINEL = std::numeric_limits<uint64_t>::max();

// Use the concrete type (via alias FollyMapU64) in the implementation
std::unique_ptr<ConcurrentHashMapU64> new_map() {
    // Create the map on the heap and return a unique_ptr
    // You might want to pass initial size parameters here
    return std::make_unique<FollyMapU64>();
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
