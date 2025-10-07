/// Base trait for all collections
///
/// This trait provides a unified interface for different collection types
/// (Vec, HashMap, etc.) allowing users to work with them in a generic way.
///
/// # Associated Types
///
/// - `Key`: The type used to index into the collection
/// - `Value`: The type of values stored in the collection
/// - `Iter`: The iterator type returned by `iter()`
///
/// # Examples
///
/// ## Using with Vec
///
/// ```
/// use dioxus_collection_store::Collection;
///
/// let mut vec = vec![1, 2, 3];
/// assert_eq!(vec.get(&1), Some(&2));
/// vec.insert(1, 10);
/// assert_eq!(vec.get(&1), Some(&10));
/// ```
///
/// ## Using with HashMap
///
/// ```
/// use dioxus_collection_store::Collection;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("key".to_string(), 42);
/// assert_eq!(map.get(&"key".to_string()), Some(&42));
/// ```
///
/// ## Iterating
///
/// ```
/// use dioxus_collection_store::Collection;
///
/// let vec = vec!["a", "b", "c"];
/// for (idx, value) in Collection::iter(&vec) {
///     println!("[{}] = {}", idx, *value);
/// }
/// ```
pub trait Collection {
    /// The type used to index into the collection
    /// (e.g., usize for Vec, K for HashMap<K, V>)
    type Key: Clone;
    type Value;

    /// Get a reference to the value associated with the given key (or index for SequentialCollection)
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::Collection;
    ///
    /// let vec = vec![10, 20, 30];
    /// assert_eq!(vec.get(&1), Some(&20));
    /// assert_eq!(vec.get(&5), None);
    /// ```
    fn get(&self, key: &Self::Key) -> Option<&Self::Value>;

    /// Get a mutable reference to the value associated with the given key (or index for SequentialCollection)
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::Collection;
    ///
    /// let mut vec = vec![10, 20, 30];
    /// if let Some(value) = vec.get_mut(&1) {
    ///     *value = 99;
    /// }
    /// assert_eq!(vec[1], 99);
    /// ```
    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value>;

    /// Update the value for an existing key
    ///
    /// Returns `true` if successful, `false` if key doesn't exist.
    /// For replacing with previous value, use `insert()` instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::Collection;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// assert!(vec.set(1, 99));
    /// assert_eq!(vec[1], 99);
    /// assert!(!vec.set(10, 99));  // Out of bounds
    /// ```
    fn set(&mut self, key: Self::Key, value: Self::Value) -> bool;

    /// Insert a key-value pair
    ///
    /// Returns the previous value if the key already existed.
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::Collection;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert_eq!(map.insert("key".to_string(), 1), None);
    /// assert_eq!(map.insert("key".to_string(), 2), Some(1));
    /// ```
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    /// Remove the value associated with the given key
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::Collection;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// assert_eq!(Collection::remove(&mut vec, &1), Some(2));
    /// assert_eq!(vec.len(), 2);
    /// ```
    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value>;

    /// Check if a key exists in the collection
    fn contains_key(&self, key: &Self::Key) -> bool {
        self.get(key).is_some()
    }

    /// Get an iterator over all keys in the collection
    ///
    /// This is a required method that each collection must implement
    /// to provide iteration support.
    fn keys(&self) -> Vec<Self::Key>;

    /// Get the number of elements in the collection
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::Collection;
    ///
    /// let vec = vec![1, 2, 3];
    /// assert_eq!(vec.len(), 3);
    /// ```
    fn len(&self) -> usize;

    /// Check if the collection is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::Collection;
    ///
    /// let vec: Vec<i32> = Vec::new();
    /// assert!(vec.is_empty());
    /// ```
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Remove all elements from the collection
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::Collection;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// vec.clear();
    /// assert!(vec.is_empty());
    /// ```
    fn clear(&mut self);

    /// Extend the collection with multiple key-value pairs
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::Collection;
    /// use std::collections::HashMap;
    ///
    /// let mut map: HashMap<String, i32> = HashMap::new();
    /// Collection::extend(&mut map, vec![("a".to_string(), 1), ("b".to_string(), 2)]);
    /// assert_eq!(map.len(), 2);
    /// ```
    fn extend<I: IntoIterator<Item = (Self::Key, Self::Value)>>(&mut self, items: I)
    where
        Self::Value: Clone,
    {
        for (key, value) in items {
            self.insert(key, value);
        }
    }
}

/// Trait for sequential collections (that support push operations)
///
/// This trait extends the base Collection trait for collections that maintain
/// an ordered sequence of elements, like Vec or CircularBuffer.
///
/// # Examples
///
/// ```
/// use dioxus_collection_store::SequentialCollection;
///
/// let mut vec = Vec::new();
/// vec.push(1);
/// vec.push(2);
/// assert_eq!(vec.len(), 2);
/// assert!(!vec.is_empty());
/// ```
pub trait SequentialCollection: Collection {
    /// Add an element to the end of the collection
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::SequentialCollection;
    ///
    /// let mut vec = vec![1, 2];
    /// vec.push(3);
    /// assert_eq!(vec.len(), 3);
    /// ```
    fn push(&mut self, value: Self::Value);

    /// Remove and return the last element
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::SequentialCollection;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// assert_eq!(vec.pop(), Some(3));
    /// assert_eq!(vec.len(), 2);
    /// ```
    fn pop(&mut self) -> Option<Self::Value>;

    /// Get the first element
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::SequentialCollection;
    ///
    /// let vec = vec![1, 2, 3];
    /// assert_eq!(vec.first(), Some(&1));
    /// ```
    fn first(&self) -> Option<&Self::Value>;

    /// Swap two elements by their keys
    ///
    /// # Examples
    ///
    /// ```
    /// use dioxus_collection_store::SequentialCollection;
    ///
    /// let mut vec = vec![1, 2, 3];
    /// vec.swap(&0, &2);
    /// assert_eq!(vec, vec![3, 2, 1]);
    /// ```
    fn swap(&mut self, key1: &Self::Key, key2: &Self::Key);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_vec_collection() {
        let mut vec = vec![1, 2, 3];

        // Test get
        assert_eq!(Collection::get(&vec, &0), Some(&1));
        assert_eq!(Collection::get(&vec, &3), None);

        // Test set
        assert!(Collection::set(&mut vec, 0, 10));
        assert_eq!(vec[0], 10);
        assert!(!Collection::set(&mut vec, 5, 20)); // Out of bounds

        // Test insert
        assert_eq!(Collection::insert(&mut vec, 1, 99), Some(2));
        assert_eq!(vec[1], 99);

        // Test remove
        assert_eq!(Collection::remove(&mut vec, &1), Some(99));

        // Test keys
        let keys = Collection::keys(&vec);
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn test_vec_sequential() {
        let mut vec: Vec<i32> = Vec::new();

        assert!(Collection::is_empty(&vec));

        SequentialCollection::push(&mut vec, 1);
        SequentialCollection::push(&mut vec, 2);
        SequentialCollection::push(&mut vec, 3);

        assert_eq!(Collection::len(&vec), 3);
        assert!(!Collection::is_empty(&vec));

        // Test first
        assert_eq!(SequentialCollection::first(&vec), Some(&1));

        // Test pop
        assert_eq!(SequentialCollection::pop(&mut vec), Some(3));
        assert_eq!(Collection::len(&vec), 2);

        // Test swap
        SequentialCollection::swap(&mut vec, &0, &1);
        assert_eq!(vec, vec![2, 1]);

        // Test clear
        Collection::clear(&mut vec);
        assert!(Collection::is_empty(&vec));
        assert_eq!(SequentialCollection::first(&vec), None);
    }

    #[test]
    fn test_hashmap_collection() {
        let mut map = HashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);

        // Test get
        assert_eq!(Collection::get(&map, &"a".to_string()), Some(&1));
        assert_eq!(Collection::get(&map, &"c".to_string()), None);

        // Test set
        assert!(Collection::set(&mut map, "a".to_string(), 10));
        assert_eq!(map["a"], 10);
        assert!(!Collection::set(&mut map, "c".to_string(), 20)); // Key doesn't exist

        // Test insert
        assert_eq!(Collection::insert(&mut map, "a".to_string(), 100), Some(10));
        assert_eq!(Collection::insert(&mut map, "c".to_string(), 3), None);

        // Test extend
        Collection::extend(&mut map, vec![("d".to_string(), 4), ("e".to_string(), 5)]);
        assert_eq!(Collection::len(&map), 5); // a, b, c, d, e

        // Test remove
        assert_eq!(Collection::remove(&mut map, &"a".to_string()), Some(100));
        assert_eq!(Collection::remove(&mut map, &"a".to_string()), None);

        // Test keys
        let keys = Collection::keys(&map);
        assert_eq!(keys.len(), 4); // b, c, d, e

        // Test clear
        Collection::clear(&mut map);
        assert!(Collection::is_empty(&map));
    }

    #[test]
    fn test_btreemap_collection() {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);

        // Test get
        assert_eq!(Collection::get(&map, &"a".to_string()), Some(&1));
        assert_eq!(Collection::get(&map, &"c".to_string()), None);

        // Test set
        assert!(Collection::set(&mut map, "a".to_string(), 10));
        assert_eq!(map["a"], 10);
        assert!(!Collection::set(&mut map, "c".to_string(), 20)); // Key doesn't exist

        // Test insert
        assert_eq!(Collection::insert(&mut map, "a".to_string(), 100), Some(10));
        assert_eq!(Collection::insert(&mut map, "c".to_string(), 3), None);

        // Test extend
        Collection::extend(&mut map, vec![("d".to_string(), 4), ("e".to_string(), 5)]);
        assert_eq!(Collection::len(&map), 5); // a, b, c, d, e

        // Test remove
        assert_eq!(Collection::remove(&mut map, &"a".to_string()), Some(100));
        assert_eq!(Collection::remove(&mut map, &"a".to_string()), None);

        // Test keys (BTreeMap keeps keys sorted)
        let keys = Collection::keys(&map);
        assert_eq!(keys.len(), 4); // b, c, d, e
        assert_eq!(
            keys,
            vec![
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string()
            ]
        );

        // Test clear
        Collection::clear(&mut map);
        assert!(Collection::is_empty(&map));
    }
}
