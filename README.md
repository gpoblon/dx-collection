[![ci](https://github.com/gpoblon/dioxus-collection/actions/workflows/ci.yml/badge.svg)](https://github.com/gpoblon/dioxus-collection/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/gpoblon/dioxus-collection/graph/badge.svg?token=CBuDPi6XgO)](https://codecov.io/gh/gpoblon/dioxus-collection)

# dioxus-collection-store

Efficient trait-based reactive collection management for Dioxus 0.7 heavily relying on dioxus new `Store` system.

Unified interface for `Vec`, `HashMap`, `BTreeMap` and custom collections with built-in selection management and reactive updates.

## Features

- Unified `Collection` trait (and optional `SequentialCollection`) for all collection types
- Built-in and safe selection and item management
- **Powerful iterators**: use `filter()`, `map()`, `find()`, ... seamlessly with reactive mutations. `iter()` provides both read and write access (no need for `iter_mut()`). 
- API focused on developer experience
- Signal-based reactivity using Dioxus Stores
- Signal-style API (`peek()`, `read()`, `write()`, `set(value)`)
- Custom error types with `CollectionError`
- `use_collection` hook to easily initialize your collection
- Reactivity happens at the lens (item) level, not at the collection level, ensuring optimal performance

## Why Collections?

Without this library, you must pass the entire collection plus an index to child components. More importantly, **there's no built-in selection management**.

For an extensive demonstration, check out the [examples/comparison.rs] example.

**Benefits**:
- ✅ **Built-in selection management** - no manual `selected_index` signal
- ✅ **Single prop per child** - just pass the item
- ✅ **Automatic selection clearing** - when removing selected item
- ✅ **No bounds checking needed** - handled internally
- ✅ **Direct item operations** - `item.select()`, `item.remove()`, `item.set()`

### Comparison

| Feature | Signal<Vec<T>> | Store<Vec<T>> | CollectionStore |
|---------|---------------|---------------|-----------------|
| **Selection Management** | ❌ Manual `selected_index` signal | ❌ Manual `selected_index` signal | ✅ Built-in |
| **Props per Child** | 3 (collection, index, selected) | 3 (index, item, selected) | 1 (item only) |
| **Selection Check** | Manual `==` comparison | Manual `==` comparison | `item.is_selected()` |
| **Item Access** | `todos.read()[index]` | `todo()` or `todo.read()` | `item.read()` |
| **Item Mutation** | `todos.write()[index] = ...` | `item.set(...)` | `item.set(...)` |
| **Bounds Checking** | ❌ Manual | ⚠️ Manual for some ops | ✅ Automatic |
| **Remove Item** | Manual + clear selection | Manual + clear selection | `item.remove()` (auto-clear) |
| **Select Item** | `selected.set(Some(index))` | `selected.set(Some(index))` | `item.select()` |
| **Granular Reactivity** | ❌ Entire Vec | ✅ Per-item | ✅ Per-item |

## Quick Start

```rust
use dioxus::prelude::*;
use dioxus_collection_store::{use_collection, CollectionItem};

#[component]
fn App() -> Element {
    let items = use_collection(|| vec!["Hello", "World"]);
    
    rsx! {
        // Built-in selection display
        if let Some(selected) = items.selected() {
            p { "Selected: {selected.read()}" }
        }
        
        button { onclick: move |_| items.push("!"), "Add" }
        
        // Single prop per item - selection is built-in!
        for item in items.iter() {
            Item { item }
        }
    }
}

#[component]
fn Item(item: CollectionItem<Vec<&'static str>>) -> Element {
    rsx! {
        div {
            onclick: move |_| { let _ = item.select(); },
            background: if item.is_selected() { "yellow" } else { "white" },
            "{item.read()}"
        }
    }
}
```

## Iterators: Readable + Writable

Unlike traditional Rust collections requiring separate `iter()` and `iter_mut()`, 
**CollectionStore provides a single iterator with both capabilities**:

```rust
let tasks = use_collection(|| vec!["task1", "URGENT: task2", "task3"]);

// Filter and mutate in ONE pass!
tasks.iter()
    .filter(|item| item.read().contains("URGENT"))
    .for_each(|item| {
        item.write().push_str(" ✓");
    });

// Mix read and write in same loop
for item in tasks.iter() {
    let text = item.read();
    if text.len() > 10 {
        item.write().push_str(" [LONG]");
    }
}
```

This is possible thanks to **Dioxus signals** which handle borrow safety at runtime,
giving you the flexibility of dynamic borrow checking with the ergonomics of a single API.

## Collections Supported

### Vec<T>
```rust
let store = use_collection(|| vec![1, 2, 3]);
store.push(4);
store.insert(1, 42); // [1, 42, 2, 3, 4]
store.swap(&1, &3); // [1, 3, 2, 42, 4]
store.select(&0).ok(); // selected: 1
if let Some(selected) = store.selected() {
    selected.remove();
}
assert_eq!(store.len(), 4);
```

### HashMap<K, V>
```rust
let store = use_collection(|| std::collections::HashMap::<&'static str, &'static str>::new());
store.insert("key", "value");
store.select(&"key").ok();
if let Some(selected) = store.selected() {
    selected.remove();
}
assert!(store.is_empty());
```

### BTreeMap<K, V>
```rust
let store = use_collection(|| std::collections::BTreeMap::<&'static str, &'static str>::new());
store.insert("key", "value");
store.select(&"key").ok();
if let Some(selected) = store.selected() {
    selected.remove();
}
assert!(store.is_empty());
```

### Custom Collections
```rust
use dioxus_collection_store::{Collection, SequentialCollection};

struct CircularBuffer<T> { /* ... */ }

impl<T: Clone> Collection for CircularBuffer<T> {
    type Key = usize;
    type Value = T;
    // Implement required methods...
}

// Use it like any other collection
let logs = use_collection(|| CircularBuffer::new(5));
logs.push("Log entry");
```

## Error Handling

```rust
match store.select(&key) {
    Ok(()) => println!("Selected"),
    Err(CollectionError::KeyNotFound) => println!("Key not found"),
    Err(e) => println!("Error: {}", e),
}
```

## Examples

```sh
# See all three approaches (Signal, Store, CollectionStore) side-by-side in action
cargo run --example comparison

# Complete demo with Vec, HashMap, BTreeMap, and custom CircularBuffer
cargo run --example collections

# Iterator power: filter + map + mutate in one pass
cargo run --example iterator
```

## Installation

```toml
[dependencies]
dioxus-collection = { git = "https://github.com/gpoblon/dioxus-collection.git" } // soon to be published
dioxus = { version = "0.7" } // soon to be released
```

## Alternative design

Initially, the Collection API was designed to impl on Store directly. You can see how it looked in examples/alternative_design.rs.
While a lot simpler to implement, I tried to provide an encapsulated, trait based implementation.
It had the benefit to provide more freedom to the user: full access to the Store API.
However, it had a few notable downsides:
- No common interface for different collection types, so it must manually be implemented over all collections
- Less safe to use, as users can mess with the internal state directly
- No common hook.

## License

MIT OR Apache-2.0
