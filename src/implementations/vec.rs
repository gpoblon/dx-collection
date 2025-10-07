use crate::collection_trait::{Collection, SequentialCollection};

/// Implementation of Collection trait for `Vec<T>`
///
/// For Vec, the Key type is usize (index) and Value is the element type T.
impl<T> Collection for Vec<T> {
    type Key = usize;
    type Value = T;

    fn get(&self, key: &Self::Key) -> Option<&Self::Value> {
        <[T]>::get(self, *key)
    }

    fn get_mut(&mut self, key: &Self::Key) -> Option<&mut Self::Value> {
        <[T]>::get_mut(self, *key)
    }

    fn set(&mut self, key: Self::Key, value: Self::Value) -> bool {
        if key < self.len() {
            self[key] = value;
            true
        } else {
            false
        }
    }

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        if key == self.len() {
            // Insert at the end
            self.push(value);
            None
        } else if key < self.len() {
            // Replace existing element
            Some(std::mem::replace(&mut self[key], value))
        } else {
            // Out of bounds
            None
        }
    }

    fn remove(&mut self, key: &Self::Key) -> Option<Self::Value> {
        if *key < self.len() {
            Some(self.remove(*key))
        } else {
            None
        }
    }

    fn keys(&self) -> Vec<Self::Key> {
        (0..self.len()).collect()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn clear(&mut self) {
        self.clear();
    }
}

/// Vec also implements SequentialCollection
impl<T> SequentialCollection for Vec<T> {
    fn push(&mut self, value: Self::Value) {
        self.push(value);
    }

    fn pop(&mut self) -> Option<Self::Value> {
        Vec::pop(self)
    }

    fn first(&self) -> Option<&Self::Value> {
        <[T]>::first(self)
    }

    fn swap(&mut self, key1: &Self::Key, key2: &Self::Key) {
        if *key1 < self.len() && *key2 < self.len() {
            <[T]>::swap(self, *key1, *key2);
        }
    }
}
