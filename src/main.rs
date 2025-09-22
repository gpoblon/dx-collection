use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

/// Hook for creating and managing a CollectionStore
///
/// # Example
/// ```rust
/// let collection = use_collection(|| vec!["item1".to_string(), "item2".to_string()]);
/// ```
pub fn use_collection<T>(initial_items: impl FnOnce() -> Vec<T>) -> CollectionStore<T>
where
    T: 'static + Clone,
{
    let store = use_store(|| Collection {
        items: initial_items(),
        selected: None,
    });
    CollectionStore::from_store(store)
}

#[derive(Store)]
struct Collection<T> {
    items: Vec<T>,
    selected: Option<usize>,
}

/// Safe wrapper around Store<Collection<T>> that prevents direct manipulation of selection state
/// Users can only interact with items in a controlled manner
#[derive(Clone, PartialEq)]
pub struct CollectionStore<T> {
    inner: Store<Collection<T>>,
}

impl<T> CollectionStore<T>
where
    T: 'static,
{
    /// Create a new CollectionStore from an existing store
    /// This is used internally by the use_collection hook
    fn from_store(store: Store<Collection<T>>) -> Self {
        Self { inner: store }
    }

    /// Get the number of items in the collection
    pub fn len(&self) -> usize {
        self.inner.items().len()
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.inner.items().is_empty()
    }

    /// Get an iterator over the items (read-only)
    pub fn iter(&self) -> impl Iterator<Item = ReadOnlyItem<T>> {
        (0..self.len()).map(move |index| ReadOnlyItem {
            collection: self.inner,
            index,
        })
    }

    /// Add an item to the collection
    pub fn push(&self, item: T) {
        self.inner.items().write().push(item);
    }

    /// Remove an item by index (internal use)
    fn remove_at_index(&self, index: usize) -> Option<T> {
        if index < self.len() {
            Some(self.inner.items().write().remove(index))
        } else {
            None
        }
    }

    /// Get the currently selected item as an editable store
    pub fn get_selected_item(&self) -> Option<Store<T>>
    where
        T: Clone,
    {
        self.inner.get_selected_item().map(|item| item.into())
    }

    /// Safely select an item by index (internal use)
    fn select_index(&self, index: usize) -> Result<(), String> {
        if self.len() <= index {
            return Err(format!(
                "Item index {} out of bounds (max: {})",
                index,
                if self.is_empty() { 0 } else { self.len() - 1 }
            ));
        }
        self.inner.selected().set(Some(index));
        Ok(())
    }

    /// Select an item by its value
    pub fn select_by_value(&self, value: &T) -> Result<(), String>
    where
        T: PartialEq + Clone,
    {
        for item in self.iter() {
            if item.value() == *value {
                return item.select();
            }
        }
        Err("Item not found in collection".to_string())
    }

    /// Select an item by its display string value
    pub fn select_by_display(&self, display_value: &str) -> Result<(), String>
    where
        T: std::fmt::Display + Clone,
    {
        for item in self.iter() {
            if item.value().to_string() == display_value {
                return item.select();
            }
        }
        Err("Item not found in collection".to_string())
    }

    /// Clear the selection
    pub fn clear_selection(&self) {
        self.inner.selected().set(None);
    }

    /// For internal component use only - access to underlying store for reactive iteration
    pub(crate) fn __internal_store(&self) -> Store<Collection<T>> {
        self.inner
    }
}

/// A read-only reference to an item in the collection with its index
pub struct ReadOnlyItem<T> {
    collection: Store<Collection<T>>,
    index: usize,
}

impl<T> ReadOnlyItem<T>
where
    T: Clone + 'static,
{
    /// Get the value of this item
    pub fn value(&self) -> T {
        self.collection.items().read()[self.index].clone()
    }

    /// Check if this item is selected
    pub fn is_selected(&self) -> bool {
        *self.collection.selected().read() == Some(self.index)
    }

    /// Select this item
    pub fn select(&self) -> Result<(), String> {
        let store = CollectionStore {
            inner: self.collection,
        };
        store.select_index(self.index)
    }

    /// Remove this item from the collection
    pub fn remove(&self) -> Option<T> {
        let store = CollectionStore {
            inner: self.collection,
        };
        store.remove_at_index(self.index)
    }
}

#[store]
impl<T: 'static, Lens> Store<Collection<T>, Lens> {
    /// Returns a store for the selected item that can be edited
    /// Reimplements the logic of .index() method to avoid using unexposed IndexWrite type
    fn get_selected_item(
        &self,
    ) -> Option<
        Store<
            T,
            MappedMutSignal<
                T,
                MappedMutSignal<Vec<T>, Lens>,
                impl Fn(&Vec<T>) -> &T + Copy + 'static,
                impl Fn(&mut Vec<T>) -> &mut T + Copy + 'static,
            >,
        >,
    > {
        let index = (*self.selected().read())?;
        if self.items().len() <= index {
            return None;
        }

        // Use hash_child to create the same subscription behavior as .index()
        // This creates a scoped store that only updates when the specific index changes
        Some(
            self.items()
                .selector()
                .hash_child(
                    &index,
                    move |vec: &Vec<T>| &vec[index],
                    move |vec: &mut Vec<T>| &mut vec[index],
                )
                .into(),
        )
    }
}

#[component]
fn App() -> Element {
    let string_collection = use_collection(|| {
        vec![
            "Hello, world!".to_string(),
            "This is a test string.".to_string(),
            "Dioxus is awesome!".to_string(),
        ]
    });

    let number_collection = use_collection(|| vec![42, 100, 256, 512]);

    rsx! {
        div {
            h2 { "String Collection" }
            Selector { collection: string_collection.clone() }

            if let Some(s) = string_collection.get_selected_item() {
                ItemEditor { item: s }
            }
        }

        div {
            h2 { "Number Collection" }
            Selector { collection: number_collection.clone() }

            if let Some(num) = number_collection.get_selected_item() {
                ItemEditor { item: num }
            }
        }
    }
}

#[component]
fn Selector<T>(collection: CollectionStore<T>) -> Element
where
    T: std::fmt::Display + Clone + 'static + PartialEq,
{
    rsx! {
        h3 { "Items" }
        select {
            value: collection.get_selected_item().map(|s| s.read().to_string()).unwrap_or_default(),
            onchange: move |evt| {
                if let Err(e) = collection.select_by_display(&evt.value()) {
                    eprintln!("Selection error: {}", e);
                }
            },
            // Options populated from items
            option { value: "", "Choose an item" }
            for item in collection.iter() {
                option {
                    key: "{item.value()}",
                    value: "{item.value()}",
                    selected: item.is_selected(),
                    "{item.value()}"
                }
            }
        }
    }
}

#[component]
pub fn ItemEditor<T>(item: Store<T>) -> Element
where
    T: std::fmt::Display + std::str::FromStr + Clone + 'static,
    T::Err: std::fmt::Debug,
{
    rsx! {
        input {
            value: item().to_string(),
            onchange: move |e| {
                if let Ok(parsed) = e.value().parse::<T>() {
                    item.set(parsed);
                }
            },
        }
    }
}
