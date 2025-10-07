use crate::{Collection, CollectionResult, CollectionStore};
use dioxus_signals::*;
use dioxus_stores::Store;

/// A reference to an item in a generic collection
///
/// Provides both read and write access to the item's value,
/// as well as methods to select or remove the item.
///
/// CollectionItem implements `Readable` and `Writable`, so you can:
/// - Call `.read()` to get a reactive read reference
/// - Call `.write()` to get a mutable write reference
/// - Call `.boxed()` to get a `ReadSignal`
/// - Call `.boxed_mut()` to get a `WriteSignal`
#[derive(Clone, Copy, PartialEq, Store)]
pub struct CollectionItem<C: 'static>
where
    C: Collection + 'static,
{
    pub(crate) store: CollectionStore<C>,
    pub(crate) key: C::Key,
}

impl<C> std::fmt::Debug for CollectionItem<C>
where
    C: Collection + 'static,
    C::Key: Clone + PartialEq + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CollectionItem")
            .field("key", &self.key)
            .field("is_selected", &self.is_selected())
            .finish()
    }
}

impl<C> Readable for CollectionItem<C>
where
    C: Collection + 'static,
    C::Key: Clone + PartialEq,
{
    type Target = C::Value;
    type Storage = UnsyncStorage;

    fn try_read_unchecked(&self) -> Result<ReadableRef<'static, Self>, BorrowError>
    where
        Self::Target: 'static,
    {
        self.store.items().try_read_unchecked().map(|value| {
            UnsyncStorage::map(value, |collection: &C| {
                collection
                    .get(&self.key)
                    .unwrap_or_else(|| {
                        // This should never happen if the API is used correctly
                        // but we provide a fallback to avoid panics
                        panic!("Attempted to access a key that does not exist in the collection. This is a bug.")
                    })
            })
        })
    }

    fn try_peek_unchecked(&self) -> Result<ReadableRef<'static, Self>, BorrowError>
    where
        Self::Target: 'static,
    {
        self.store.items().try_peek_unchecked().map(|value| {
            UnsyncStorage::map(value, |collection: &C| {
                collection
                    .get(&self.key)
                    .unwrap_or_else(|| {
                        panic!("Attempted to access a key that does not exist in the collection. This is a bug.")
                    })
            })
        })
    }

    fn subscribers(&self) -> dioxus_core::Subscribers
    where
        Self::Target: 'static,
    {
        self.store.items().subscribers()
    }
}

impl<C> Writable for CollectionItem<C>
where
    C: Collection + 'static,
    C::Key: Clone + PartialEq,
{
    type WriteMetadata = Box<dyn std::any::Any>;

    fn try_write_unchecked(&self) -> Result<WritableRef<'static, Self>, BorrowMutError>
    where
        Self::Target: 'static,
    {
        self.store.items().try_write_unchecked().map(|value| {
            WriteLock::map(value, |collection: &mut C| {
                collection
                    .get_mut(&self.key)
                    .unwrap_or_else(|| {
                        panic!("Attempted to access a key that does not exist in the collection. This is a bug.")
                    })
            })
        })
    }
}

impl<C> CollectionItem<C>
where
    C: Collection + 'static,
    C::Key: Clone + PartialEq,
{
    /// Get the key of this item
    pub fn key(&self) -> C::Key {
        self.key.clone()
    }

    /// Get the key-value tuple for this item
    ///
    /// This is a convenience method that returns both the key and the cloned value.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// if let Some(item) = store.selected() {
    ///     let (key, value) = item.kv();
    ///     // Use key and value
    /// }
    /// ```
    pub fn kv(&self) -> (C::Key, C::Value)
    where
        C::Value: Clone,
    {
        (self.key.clone(), self.read().clone())
    }

    /// Set/replace the value of this item
    ///
    /// Directly updates the value without cloning.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// let item = store.get(&key);
    /// item.set(new_value);
    /// ```
    pub fn set(&self, value: C::Value) {
        self.store.items().write().set(self.key.clone(), value);
    }

    /// Check if this item is currently selected
    pub fn is_selected(&self) -> bool {
        *self.store.selected_key_signal().read() == Some(self.key.clone())
    }

    /// Select this item
    pub fn select(&self) -> CollectionResult<()> {
        self.store.select(&self.key)
    }

    /// Remove this item from the collection
    ///
    /// Returns the removed value.
    pub fn remove(&self) -> Option<C::Value>
    where
        C::Value: Clone,
    {
        self.store.remove(&self.key)
    }
}
