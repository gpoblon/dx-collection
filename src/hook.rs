use crate::{Collection, CollectionData, CollectionStore};

use dioxus_stores::*;

/// Hook for creating a generic reactive collection store
///
/// Creates a `CollectionStore` wrapper around any type implementing the `Collection` trait.
/// This works with built-in collections (`Vec`, `HashMap`) and custom collection types.
///
/// # Examples
///
/// ## With Vec
///
/// ```rust,no_run
/// use dioxus_collection_store::use_collection;
///
/// // In a Dioxus component:
/// let numbers = use_collection(|| vec![1, 2, 3]);
/// numbers.push(4);
/// ```
///
/// ## With HashMap
///
/// ```rust,no_run
/// use dioxus_collection_store::use_collection;
/// use std::collections::HashMap;
///
/// let scores = use_collection(|| {
///     let mut map = HashMap::new();
///     map.insert("Alice".to_string(), 95);
///     map
/// });
/// ```
///
/// ## With Custom Collections
///
/// ```rust,no_run
/// # use dioxus_collection_store::use_collection;
/// # struct CircularBuffer<T> { data: Vec<T> }
/// # impl<T> CircularBuffer<T> { fn new(cap: usize) -> Self { Self { data: Vec::new() } } }
/// // Works with any type implementing Collection trait
/// let logs = use_collection(|| CircularBuffer::new(5));
/// ```
pub fn use_collection<C>(initial: impl FnOnce() -> C) -> CollectionStore<C>
where
    C: Collection + 'static,
    C::Key: Clone + PartialEq,
{
    let store = use_store(|| CollectionData {
        items: initial(),
        selected_key: None,
    });
    CollectionStore::from(store)
}
