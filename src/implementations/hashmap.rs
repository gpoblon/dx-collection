use crate::collection_trait::Collection;
use std::collections::HashMap;
use std::hash::Hash;

/// Implementation of Collection trait for HashMap<K, V>
///
/// For HashMap, the Key type is K and Value is V.
/// K must be Clone to satisfy the iterator requirements.
impl<K, V> Collection for HashMap<K, V>
where
    K: Eq + Hash + Clone,
{
    type Key = K;
    type Value = V;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.get(key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        self.get_mut(key)
    }

    fn set(&mut self, key: Self::Key, value: Self::Value) -> bool {
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.entry(key) {
            e.insert(value);
            true
        } else {
            false
        }
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        self.remove(key)
    }

    fn keys(&self) -> Vec<Self::Key> {
        self.keys().cloned().collect()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn clear(&mut self) {
        self.clear();
    }
}
