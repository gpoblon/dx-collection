use dioxus_collection_store::{Collection, SequentialCollection};

/// Example implementation of the Collection trait for a custom collection type
///
/// # Purpose
///
/// This module demonstrates how to implement the `Collection` and `SequentialCollection`
/// traits for custom collection types. This is a key extensibility point of the library.
///
/// # What This Shows
///
/// - How to create a custom collection with specialized behavior (circular buffer)
/// - How to implement the required `Collection` trait methods
/// - How to optionally implement `SequentialCollection` for `push()` support
/// - How the custom collection integrates seamlessly with `CollectionStore`
///
/// # Usage in Other Examples
///
/// - `collections.rs` - Full demo with all collection types
///
/// # Implement Your Own
///
/// To create your own custom collection:
/// 1. Define your collection struct with its data and constraints
/// 2. Implement `Collection` trait (required methods: get, set, insert, remove, keys, len)
/// 3. Optionally implement `SequentialCollection` for push support
/// 4. Use it with `use_collection()` just like Vec or HashMap!
#[derive(Clone, PartialEq)]
pub struct CircularBuffer<T> {
    data: Vec<T>,
    capacity: usize,
    start: usize,
    len: usize,
}

impl<T> CircularBuffer<T> {
    /// Create a new circular buffer with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
            start: 0,
            len: 0,
        }
    }

    /// Get the actual index in the internal vector
    fn real_index(&self, logical_index: usize) -> Option<usize> {
        if logical_index >= self.len {
            return None;
        }
        Some((self.start + logical_index) % self.data.len())
    }
}

impl<T: Clone> Collection for CircularBuffer<T> {
    type Key = usize;
    type Value = T;

    fn get(&self, key: &usize) -> Option<&T> {
        self.real_index(*key)
            .and_then(|idx| self.data.as_slice().get(idx))
    }

    fn get_mut(&mut self, key: &usize) -> Option<&mut T> {
        if let Some(idx) = self.real_index(*key) {
            self.data.as_mut_slice().get_mut(idx)
        } else {
            None
        }
    }

    fn set(&mut self, key: usize, value: T) -> bool {
        if let Some(idx) = self.real_index(key) {
            self.data[idx] = value;
            true
        } else {
            false
        }
    }

    fn insert(&mut self, key: usize, value: T) -> Option<T> {
        if let Some(idx) = self.real_index(key) {
            Some(std::mem::replace(&mut self.data[idx], value))
        } else {
            None
        }
    }

    fn remove(&mut self, key: &usize) -> Option<T> {
        if *key >= self.len {
            return None;
        }

        let real_idx = self.real_index(*key)?;
        let removed = self.data.remove(real_idx);

        // Adjust the buffer state
        if real_idx < self.start {
            self.start = self.start.saturating_sub(1);
        }
        self.len -= 1;

        Some(removed)
    }

    fn keys(&self) -> Vec<Self::Key> {
        (0..self.len).collect()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn clear(&mut self) {
        self.data.clear();
        self.start = 0;
        self.len = 0;
    }
}

impl<T: Clone> SequentialCollection for CircularBuffer<T> {
    fn push(&mut self, value: T) {
        if self.data.len() < self.capacity {
            // Buffer not yet full
            self.data.push(value);
            self.len += 1;
        } else {
            // Buffer is full, overwrite the oldest item
            let write_idx = (self.start + self.len) % self.capacity;
            if write_idx < self.data.len() {
                self.data[write_idx] = value;
            }
            // Move start pointer (oldest item is now overwritten)
            self.start = (self.start + 1) % self.capacity;
            // Length stays the same (still at capacity)
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.len -= 1;
            let idx = (self.start + self.len) % self.data.len();
            Some(self.data.remove(idx))
        }
    }

    fn first(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data.get(&self.start)
        }
    }

    fn swap(&mut self, key1: &usize, key2: &usize) {
        if let (Some(idx1), Some(idx2)) = (self.real_index(*key1), self.real_index(*key2)) {
            self.data.swap(&idx1, &idx2);
        }
    }
}
