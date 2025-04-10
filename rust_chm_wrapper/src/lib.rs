#[cxx::bridge(namespace = "folly_rust_wrapper")]
mod ffi {
    // Shared types defined in C++
    unsafe extern "C++" {
        include!("rust_chm_wrapper/include/wrapper.h");

        // Opaque type for the C++ map
        type ConcurrentHashMapU64;

        // Functions exposed from C++
        fn new_map() -> UniquePtr<ConcurrentHashMapU64>;
        fn insert(map: &mut ConcurrentHashMapU64, key: u64, value: u64) -> bool;
        fn find(map: &ConcurrentHashMapU64, key: u64) -> u64; // Returns value or sentinel (e.g., 0 or max)
        fn erase(map: &mut ConcurrentHashMapU64, key: u64) -> usize; // Returns number of elements erased (0 or 1)
    }
}

// Public Rust struct that wraps the C++ map pointer
pub struct FollyMap {
    map_ptr: cxx::UniquePtr<ffi::ConcurrentHashMapU64>,
}

impl FollyMap {
    /// Creates a new Folly ConcurrentHashMap.
    pub fn new() -> Self {
        FollyMap {
            map_ptr: ffi::new_map(),
        }
    }

    /// Inserts a key-value pair. Returns true if inserted, false if key already existed.
    pub fn insert(&mut self, key: u64, value: u64) -> bool {
        ffi::insert(&mut self.map_ptr, key, value)
    }

    /// Finds a value by key. Returns the value if found, or a sentinel value (currently u64::MAX) if not found.
    /// TODO: Improve return type (e.g., Option<u64>)
    pub fn find(&self, key: u64) -> Option<u64> {
        let result = ffi::find(&self.map_ptr, key);
        // Using u64::MAX as a sentinel for not found, as defined in wrapper.cpp
        if result == u64::MAX {
            None
        } else {
            Some(result)
        }
    }

    /// Erases a key. Returns true if an element was erased, false otherwise.
    pub fn erase(&mut self, key: u64) -> bool {
        ffi::erase(&mut self.map_ptr, key) > 0
    }
}

impl Default for FollyMap {
    fn default() -> Self {
        Self::new()
    }
}

// Basic tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_find_erase() {
        let mut map = FollyMap::new();

        // Insert
        assert!(map.insert(10, 100));
        assert!(map.insert(20, 200));
        assert!(!map.insert(10, 101)); // Key 10 already exists

        // Find
        assert_eq!(map.find(10), Some(100));
        assert_eq!(map.find(20), Some(200));
        assert_eq!(map.find(30), None); // Key 30 does not exist

        // Erase
        assert!(map.erase(10)); // Erase existing key
        assert!(!map.erase(30)); // Try erasing non-existent key
        assert_eq!(map.find(10), None); // Verify erasure
        assert_eq!(map.find(20), Some(200)); // Verify other key still exists

        // Erase remaining key
        assert!(map.erase(20));
        assert_eq!(map.find(20), None);
    }
}
