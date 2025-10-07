use crate::{Collection, CollectionError, CollectionItem, CollectionResult, SequentialCollection};
use dioxus_signals::*;

use dioxus_stores::*;

/// Generic collection data wrapper for the Store
#[derive(Store)]
pub struct CollectionData<C>
where
    C: Collection + 'static,
{
    pub(crate) items: C,
    pub(crate) selected_key: Option<C::Key>,
}
/// Generic collection store that works with any Collection implementation
///
/// This provides a type-safe reactive wrapper around any collection type
/// that implements the `Collection` trait, including:
/// - Built-in types: `Vec<T>`, `HashMap<K, V>`
/// - Custom collections: any type implementing the `Collection` trait
///
/// The store provides:
/// - **Reactive updates**: Changes automatically trigger Dioxus re-renders
/// - **Selection management**: Track and manage a selected item
/// - **Signal-style API**: Familiar `read()`, `write()`, `set()`, `peek()` methods
/// - **Type safety**: All operations are type-checked at compile time
///
/// # Examples
///
/// ```rust,no_run
/// use dioxus_collection_store::{CollectionStore, use_collection};
///
/// // Works with Vec
/// let store = use_collection(|| vec![1, 2, 3]);
/// store.push(4);
/// let item = store.get(&0);
/// let signal = item.read();  // ReadSignal
///
/// // Works with HashMap
/// use std::collections::HashMap;
/// let scores = use_collection(|| {
///     let mut map = HashMap::new();
///     map.insert("Alice".to_string(), 95);
///     map
/// });
///
/// // Works with custom collections implementing Collection trait
/// # struct CircularBuffer<T> { data: Vec<T> }
/// # impl<T> CircularBuffer<T> { fn new(cap: usize) -> Self { Self { data: Vec::new() } } }
/// let logs = use_collection(|| CircularBuffer::new(5));
/// ```
#[derive(PartialEq)]
pub struct CollectionStore<C>
where
    C: Collection + 'static,
{
    inner: Store<CollectionData<C>>,
}

impl<C> std::fmt::Debug for CollectionStore<C>
where
    C: Collection + 'static,
    C::Key: Clone + PartialEq + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CollectionStore")
            .field("len", &self.len())
            .field("selected_key", &self.selected_key())
            .finish()
    }
}

impl<C> Copy for CollectionStore<C> where C: Collection + 'static {}

impl<C> Clone for CollectionStore<C>
where
    C: Collection + 'static,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<C> CollectionStore<C>
where
    C: Collection + 'static,
    C::Key: Clone + PartialEq,
{
    /// Create a new CollectionStore from a collection
    pub fn new(collection: C) -> Self {
        let store = Store::new(CollectionData {
            items: collection,
            selected_key: None,
        });
        Self { inner: store }
    }

    /// Get the length of the collection
    pub fn len(&self) -> usize {
        self.inner.items().read().len()
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.inner.items().read().is_empty()
    }

    /// Get a Store for the items collection
    ///
    /// Returns a Store providing reactive access to the underlying collection.
    pub fn items(
        &self,
    ) -> impl Writable<Target = C, WriteMetadata = Box<dyn std::any::Any>, Storage = UnsyncStorage> + Copy
    {
        self.inner.items()
    }

    /// Get a CollectionItem for a specific key in the collection
    ///
    /// This returns a CollectionItem that provides signal-based reactive access to a single item.
    /// Use `.read()` to get a reactive reference or `.boxed()` to get a ReadSignal.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// let item = store.get(&1);  // Get item for index 1
    /// let value_ref = item.read();  // Get reactive reference
    /// let signal = item.boxed();    // Or get ReadSignal
    /// item.set(42);                 // Update the value directly
    /// ```
    pub fn get(&self, key: &C::Key) -> CollectionItem<C> {
        CollectionItem {
            store: *self,
            key: key.clone(),
        }
    }

    /// Read a value from the collection by key (returns a ReadSignal, no clone)
    ///
    /// Returns a ReadSignal that provides reactive access to the value.
    /// Creates a reactive dependency on this item's value.
    ///
    /// This is a convenience method equivalent to `self.get(key).boxed()`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// let signal = store.read(&1);  // ReadSignal for value at index 1
    /// assert_eq!(*signal.read(), 2);
    /// ```
    pub fn read(&self, key: &C::Key) -> ReadSignal<C::Value>
    where
        C::Value: 'static,
    {
        self.get(key).boxed()
    }

    /// Write to a value in the collection by key (returns a WriteSignal, no clone)
    ///
    /// Returns a WriteSignal that provides mutable reactive access to the value.
    ///
    /// This is a convenience method equivalent to `self.get(key).boxed_mut()`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// let signal = store.write(&1);  // WriteSignal for value at index 1
    /// *signal.write() = 42;
    /// ```
    pub fn write(&self, key: &C::Key) -> WriteSignal<C::Value>
    where
        C::Value: 'static,
    {
        self.get(key).boxed_mut()
    }

    /// Peek at a value from the collection by key without subscribing (returns ReadSignal, no clone)
    ///
    /// Returns a ReadSignal that can be used to peek at the value without creating a reactive dependency.
    ///
    /// This is a convenience method equivalent to `self.get(key).boxed()`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// let signal = store.peek(&1);  // ReadSignal for value at index 1
    /// assert_eq!(*signal.peek(), 2);  // Peek without subscribing
    /// ```
    pub fn peek(&self, key: &C::Key) -> ReadSignal<C::Value>
    where
        C::Value: 'static,
    {
        self.get(key).boxed()
    }

    /// Get a Store for the selected key
    ///
    /// This returns the selected field Store that provides reactive access to the selection state.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// store.select(&1).ok();
    /// let selected_signal = store.selected_key_signal();
    /// let key = selected_signal.read();  // Some(1)
    /// ```
    pub fn selected_key_signal(&self) -> impl Writable<Target = Option<C::Key>> + Copy {
        self.inner.selected_key()
    }

    /// Get the currently selected item as a CollectionItem
    ///
    /// Returns `None` if no item is selected.
    /// This follows the same pattern as store.rs where `selected()` returns an item reference,
    /// not the cloned value.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// store.select(&1).ok();
    /// if let Some(item) = store.selected() {
    ///     let signal = item.read();
    ///     // Use the signal reactively
    /// }
    /// ```
    pub fn selected(&self) -> Option<CollectionItem<C>> {
        let key = self.selected_key()?;
        Some(CollectionItem { store: *self, key })
    }

    /// Check if a key exists in the collection
    pub fn contains_key(&self, key: &C::Key) -> bool {
        self.inner.items().read().get(key).is_some()
    }

    /// Insert or update a value in the collection by key
    ///
    /// This method updates an existing key or inserts a new one.
    /// Returns the previous value if the key existed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// store.insert(1, 42);  // Update index 1 to value 42
    /// ```
    pub fn insert(&self, key: C::Key, value: C::Value) -> Option<C::Value>
    where
        C::Value: Clone,
    {
        self.inner.items().write().insert(key, value)
    }

    /// Set/replace a value in the collection by key.
    ///
    /// For Vec collections, this will panic if the index is out of bounds.
    /// For HashMap collections, this will insert if the key doesn't exist.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// store.set(1, 42);  // Sets index 1 to 42
    /// ```
    pub fn set(&self, key: C::Key, value: C::Value)
    where
        C::Value: Clone,
    {
        self.inner.items().write().set(key, value);
    }

    /// Remove an item from the collection
    ///
    /// If the removed item was selected, the selection will be cleared.
    pub fn remove(&self, key: &C::Key) -> Option<C::Value>
    where
        C::Value: Clone,
    {
        // Clear selection if we're removing the selected item
        if self.selected_key() == Some(key.clone()) {
            self.clear_selection();
        }
        self.inner.items().write().remove(key)
    }

    /// Get an iterator over the collection items
    ///
    /// Returns an iterator of `CollectionItem` references that implement both `Readable` and `Writable`.
    /// This leverages Dioxus signals for efficient reactive access, allowing you to:
    ///
    /// - **Read values reactively**: `item.read()` creates a reactive dependency
    /// - **Mutate values in-place**: `item.write()` provides mutable access
    /// - **Combine with Rust iterators**: Use `filter()`, `map()`, `find()`, etc.
    ///
    /// Unlike traditional Rust collections that require separate `iter()` and `iter_mut()` methods,
    /// **a single iterator provides both read and write capabilities** thanks to the signal system.
    ///
    /// # Examples
    ///
    /// ## Read-only iteration
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3, 4, 5]);
    ///
    /// for item in store.iter() {
    ///     println!("{}", item.read());
    /// }
    /// ```
    ///
    /// ## Combining filter and mutation in one pass
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3, 4, 5]);
    ///
    /// // Filter and mutate in a single iteration!
    /// store.iter()
    ///     .filter(|item| *item.read() > 2)
    ///     .for_each(|item| {
    ///         let current = *item.read();
    ///         item.set(current * 10);
    ///     });
    /// // Result: [1, 2, 30, 40, 50]
    /// ```
    ///
    /// ## Mixed read/write in same loop
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec!["task1", "urgent_task", "task2"]);
    ///
    /// for item in store.iter() {
    ///     let value = item.read();
    ///     if value.contains("urgent") {
    ///         // Read-only: just log
    ///         println!("Found urgent: {}", value);
    ///     } else {
    ///         // Mutate: add suffix
    ///         item.write().push_str(" [processed]");
    ///     }
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = CollectionItem<C>> + '_
    where
        C::Key: Clone,
    {
        let keys: Vec<C::Key> = self.inner.items().read().keys();
        let store = *self;

        keys.into_iter()
            .map(move |key| CollectionItem { store, key })
    }

    /// Select an item by its key
    pub fn select(&self, key: &C::Key) -> CollectionResult<()> {
        if self.contains_key(key) {
            self.inner.selected_key().set(Some(key.clone()));
            Ok(())
        } else {
            Err(CollectionError::KeyNotFound)
        }
    }

    /// Select an item by its value
    ///
    /// Recommended to use `Self::select` instead whenever possible.
    /// Not recommended for performance reasons and values may not be unique.
    /// If values are not unique, this method selects the first matching item.
    pub fn select_by_value(&self, value: &C::Value) -> CollectionResult<()>
    where
        C::Value: PartialEq,
    {
        for item in self.iter() {
            if &*item.read() == value {
                return item.select();
            }
        }
        Err(CollectionError::KeyNotFound)
    }

    /// Select an item by its display string value
    ///
    /// Recommended to use `Self::select` instead whenever possible.
    /// Not recommended for performance reasons and values may not be unique.
    /// If displayed values are not unique, this method selects the first matching item.
    pub fn select_by_display(&self, display_value: &str) -> CollectionResult<()>
    where
        C::Value: std::fmt::Display,
    {
        for item in self.iter() {
            if item.read().to_string() == display_value {
                return item.select();
            }
        }
        Err(CollectionError::KeyNotFound)
    }

    /// Get the currently selected key
    pub fn selected_key(&self) -> Option<C::Key> {
        self.inner.selected_key().read().clone()
    }

    /// Clear the selection
    pub fn clear_selection(&self) {
        self.selected_key_signal().set(None);
    }

    /// Remove all items from the collection
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// store.clear();
    /// assert!(store.is_empty());
    /// ```
    pub fn clear(&self) {
        self.inner.items().write().clear();
        self.clear_selection();
    }

    /// Extend the collection with multiple key-value pairs
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::collections::HashMap;
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(HashMap::new());
    /// store.extend(vec![
    ///     ("key1".to_string(), 1),
    ///     ("key2".to_string(), 2),
    /// ]);
    /// assert_eq!(store.len(), 2);
    /// ```
    pub fn extend<I: IntoIterator<Item = (C::Key, C::Value)>>(&self, items: I)
    where
        C::Value: Clone,
    {
        self.inner.items().write().extend(items);
    }
}

/// Conversion from `Store<CollectionData<C>>` to `CollectionStore<C>`
///
/// This allows you to use `.into()` for type conversion, which is more idiomatic
/// than a custom `from_store()` method.
///
/// # Examples
///
/// ```rust,ignore
/// use dioxus_collection_store::{CollectionStore, CollectionData};
/// use dioxus_stores::Store;
///
/// let store: Store<CollectionData<Vec<i32>>> = /* ... */;
/// let collection_store: CollectionStore<Vec<i32>> = store.into();
/// ```
impl<C> From<Store<CollectionData<C>>> for CollectionStore<C>
where
    C: Collection + 'static,
    C::Key: Clone + PartialEq,
{
    fn from(store: Store<CollectionData<C>>) -> Self {
        Self { inner: store }
    }
}

/// Extension trait for SequentialCollection stores
impl<C> CollectionStore<C>
where
    C: SequentialCollection + 'static,
    C::Key: Clone + PartialEq,
{
    /// Push a new item to the collection (for sequential collections)
    pub fn push(&self, value: C::Value)
    where
        C::Value: Clone,
    {
        self.inner.items().write().push(value);
    }

    /// Remove and return the last element
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// assert_eq!(store.pop(), Some(3));
    /// assert_eq!(store.len(), 2);
    /// ```
    pub fn pop(&self) -> Option<C::Value>
    where
        C::Value: Clone,
    {
        self.inner.items().write().pop()
    }

    /// Get a reference to the first element
    ///
    /// Returns `None` if the collection is empty.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// let first = store.first();
    /// assert_eq!(first.map(|item| *item.read()), Some(1));
    /// ```
    pub fn first(&self) -> Option<CollectionItem<C>>
    where
        C::Key: From<usize>,
    {
        if self.is_empty() {
            None
        } else {
            Some(CollectionItem {
                store: *self,
                key: C::Key::from(0),
            })
        }
    }

    /// Get a reference to the last element
    ///
    /// Returns `None` if the collection is empty.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// let last = store.last();
    /// assert_eq!(last.map(|item| *item.read()), Some(3));
    /// ```
    pub fn last(&self) -> Option<CollectionItem<C>>
    where
        C::Key: From<usize>,
    {
        let len = self.len();
        if len == 0 {
            None
        } else {
            Some(CollectionItem {
                store: *self,
                key: C::Key::from(len - 1),
            })
        }
    }

    /// Swap two elements by their keys
    ///
    /// This is especially useful for drag & drop reordering!
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use dioxus_collection_store::CollectionStore;
    ///
    /// let store = CollectionStore::new(vec![1, 2, 3]);
    /// store.swap(&0, &2).ok();
    /// // Now the collection is [3, 2, 1]
    /// ```
    pub fn swap(&self, key1: &C::Key, key2: &C::Key) -> CollectionResult<()> {
        if self.contains_key(key1) && self.contains_key(key2) {
            self.inner.items().write().swap(key1, key2);
            Ok(())
        } else {
            Err(CollectionError::KeyNotFound)
        }
    }
}
