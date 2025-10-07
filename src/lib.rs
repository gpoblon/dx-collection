//! # Dioxus Collection Store
//!
#![doc = include_str!("../README.md")]
//!
//! ## Code Examples
//!
//! ### Vec<T>
//!
//! ```rust,no_run
//! use dioxus_collection_store::use_collection;
//!
//! let store = use_collection(|| vec![1, 2, 3]);
//! store.push(4);
//! store.insert(1, 42); // [1, 42, 2, 3, 4]
//! store.swap(&1, &3); // [1, 3, 2, 42, 4]
//! store.select(&0).ok(); // selected: 1
//! if let Some(selected) = store.selected() {
//!     selected.remove();
//! }
//! assert_eq!(store.len(), 4);
//! ```
//!
//! ### HashMap<K, V>
//!
//! ```rust,no_run
//! use dioxus_collection_store::use_collection;
//! use std::collections::HashMap;
//!
//! let store = use_collection(|| HashMap::<&'static str, &'static str>::new());
//! store.insert("key", "value");
//! store.select(&"key").ok();
//! if let Some(selected) = store.selected() {
//!     selected.remove();
//! }
//! assert!(store.is_empty());
//! ```
//!
//! ### BTreeMap<K, V>
//!
//! ```rust,no_run
//! use dioxus_collection_store::use_collection;
//! use std::collections::BTreeMap;
//!
//! let store = use_collection(|| BTreeMap::<&'static str, &'static str>::new());
//! store.insert("key", "value");
//! store.select(&"key").ok();
//! if let Some(selected) = store.selected() {
//!     selected.remove();
//! }
//! assert!(store.is_empty());
//! ```

pub(crate) mod collection_item;
pub(crate) mod collection_store;
pub(crate) mod collection_trait;
pub mod error;
pub(crate) mod hook;

// Implementations for standard library collections
pub mod implementations;

// Re-exports
pub use collection_item::CollectionItem;
pub(crate) use collection_store::CollectionData;
pub use collection_store::CollectionStore;
pub use collection_trait::{Collection, SequentialCollection};
pub use error::{CollectionError, CollectionResult};
pub use hook::use_collection;

#[cfg(test)]
mod tests;
