#include "rust_chm_wrapper/include/wrapper.h" // Now includes the struct definition and Folly headers

#include <limits> // For sentinel value
#include <memory> // For std::unique_ptr

// The struct ConcurrentHashMapU64Opaque is now defined in the header.

namespace folly_rust_wrapper {

// Sentinel value to indicate "not found" in the find function
const uint64_t NOT_FOUND_SENTINEL = std::numeric_limits<uint64_t>::max();

// Implement the functions declared in the header.

std::unique_ptr<ConcurrentHashMapU64Opaque> new_map() {
    // Create the opaque wrapper on the heap and return a unique_ptr
    return std::make_unique<ConcurrentHashMapU64Opaque>();
}

// Note: Parameter is const&, but we use const_cast because the underlying
// folly::CHM::insert is thread-safe despite logically modifying the map.
bool insert(const ConcurrentHashMapU64Opaque& wrapper, uint64_t key, uint64_t value) {
    // Access the map inside the wrapper and call its insert method
    return const_cast<ConcurrentHashMapU64Opaque&>(wrapper).map.insert(key, value).second;
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

// Note: Parameter is const&, but we use const_cast because the underlying
// folly::CHM::erase is thread-safe despite logically modifying the map.
size_t erase(const ConcurrentHashMapU64Opaque& wrapper, uint64_t key) {
    // Access the map inside the wrapper and call its erase method
    return const_cast<ConcurrentHashMapU64Opaque&>(wrapper).map.erase(key);
}

} // namespace folly_rust_wrapper
