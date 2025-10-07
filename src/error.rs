//! Error types for dioxus-collection-store
//!
//! This module provides custom error types for better error handling
//! throughout the library.

use std::fmt;

/// Errors that can occur when working with collections
#[derive(Debug, Clone, PartialEq)]
pub enum CollectionError {
    /// The specified key was not found in the collection
    KeyNotFound,

    /// The key is out of bounds (for indexed collections like Vec)
    OutOfBounds { index: usize, len: usize },

    /// The collection is empty
    EmptyCollection,

    /// Failed to access an item that should exist
    InvalidAccess { reason: String },

    /// A borrow error occurred while accessing the collection
    BorrowError,

    /// A mutable borrow error occurred while accessing the collection
    BorrowMutError,
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionError::KeyNotFound => {
                write!(f, "Key not found in collection")
            }
            CollectionError::OutOfBounds { index, len } => {
                write!(
                    f,
                    "Index {} out of bounds for collection of length {}",
                    index, len
                )
            }
            CollectionError::EmptyCollection => {
                write!(f, "Collection is empty")
            }
            CollectionError::InvalidAccess { reason } => {
                write!(f, "Invalid access: {}", reason)
            }
            CollectionError::BorrowError => {
                write!(f, "Failed to borrow collection (already borrowed mutably)")
            }
            CollectionError::BorrowMutError => {
                write!(f, "Failed to borrow collection mutably (already borrowed)")
            }
        }
    }
}

impl std::error::Error for CollectionError {}

/// Result type for collection operations
pub type CollectionResult<T> = Result<T, CollectionError>;

// Convert from String (for backward compatibility)
impl From<String> for CollectionError {
    fn from(s: String) -> Self {
        CollectionError::InvalidAccess { reason: s }
    }
}

impl From<&str> for CollectionError {
    fn from(s: &str) -> Self {
        CollectionError::InvalidAccess {
            reason: s.to_string(),
        }
    }
}
