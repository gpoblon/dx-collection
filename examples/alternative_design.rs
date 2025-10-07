/// This is basically a leftover from an alternative store design
/// It adds selection management directly into the store, rather than encapsulation.
/// It has the following benefit: full access to the Store API.
/// However, it has multiple downsides:
///  - Less safe to use, as users can mess with the internal state directly
///  - No common interface for different collection types, so it must manually be implemented over all collection types
use dioxus::prelude::*;

/// An item in the Store<State<C>>
/// Provides direct reactive access to a specific element
#[derive(Clone, Copy, PartialEq)]
pub struct StoreItem<C: 'static, Lens: Readable + Writable = WriteSignal<Coll<C>>> {
    store: Store<Coll<C>, Lens>,
    index: usize,
}

impl<C: 'static, Lens> StoreItem<C, Lens>
where
    Lens: Readable<Target = Coll<C>, Storage = UnsyncStorage> + Writable + Copy + 'static,
{
    /// Get the key (index) of this element
    pub fn key(&self) -> usize {
        self.index
    }

    /// Get a ReadSignal for this specific element
    pub fn read(&self) -> ReadSignal<C> {
        self.store.items().index(self.index).boxed()
    }

    /// Get a WriteSignal for this specific element
    pub fn write(&self) -> WriteSignal<C> {
        self.store.items().index(self.index).boxed_mut()
    }

    /// Set the value
    pub fn set(&self, value: C) {
        self.store.items().write()[self.index] = value;
    }

    /// Check if this element is selected
    pub fn is_selected(&self) -> bool {
        *self.store.selected_index().read() == Some(self.index)
    }

    /// Select this element
    pub fn select(&self) -> Result<(), String> {
        self.store.select(self.index)
    }

    /// Remove this element
    pub fn remove(&self) -> Option<C>
    where
        C: Clone,
    {
        self.store.remove(self.index)
    }
}

#[derive(Store)]
pub struct Coll<C>
where
    C: 'static,
{
    items: Vec<C>,
    selected_index: Option<usize>,
}

impl<T> From<Vec<T>> for Coll<T>
where
    T: 'static,
{
    fn from(items: Vec<T>) -> Self {
        Self {
            items,
            selected_index: None,
        }
    }
}

#[store(pub)]
impl<C: 'static, Lens: Readable + Writable> Store<Coll<C>, Lens> {
    /// Get the length of the collection
    fn len(&self) -> usize {
        self.items().read().len()
    }

    /// Check if the collection is empty
    fn is_empty(&self) -> bool {
        self.items().read().is_empty()
    }

    /// Get a StoreItem for a specific key (index) in the collection
    fn get(&self, key: usize) -> StoreItem<C, Lens> {
        StoreItem {
            store: *self,
            index: key,
        }
    }

    /// Returns the selected item that can be edited
    fn selected(&self) -> Option<StoreItem<C, Lens>> {
        let index = *self.selected_index().peek().as_ref()?;
        Some(StoreItem {
            store: *self,
            index,
        })
    }

    /// Get the currently selected key (index)
    fn selected_key(&self) -> Option<usize> {
        self.selected_index().read().clone()
    }

    /// Check if a key exists in the collection
    fn contains_key(&self, key: usize) -> bool {
        self.items().read().get(key).is_some()
    }

    /// Insert or update a value at the specified index
    fn insert(&self, key: usize, value: C)
    where
        C: Clone,
    {
        if key < self.items().peek().len() {
            self.items().write()[key] = value;
        } else {
            self.items().write().push(value);
        }
    }

    /// Set/replace a value at the specified index
    fn set(&self, key: usize, value: C)
    where
        C: Clone,
    {
        if key < self.items().peek().len() {
            self.items().write()[key] = value;
        }
    }

    /// Push a new item to the collection
    fn push(&self, value: C) {
        self.items().write().push(value)
    }

    /// Remove and return the last element
    fn pop(&self) -> Option<C>
    where
        C: Clone,
    {
        let result = self.items().write().pop();
        // Clear selection if we popped the selected item
        if let Some(selected_idx) = self.selected_key() {
            if selected_idx >= self.len() {
                self.clear_selection();
            }
        }
        result
    }

    /// Iterate over items in the collection
    fn iter(&self) -> impl Iterator<Item = StoreItem<C, Lens>> + '_ {
        (0..self.items().peek().len()).map(move |index| StoreItem {
            store: *self,
            index,
        })
    }

    /// Remove an item from the collection
    fn remove(&self, key: usize) -> Option<C>
    where
        C: Clone,
    {
        if self.items().peek().len() <= key {
            return None;
        }

        // Clear selection if we're removing the selected item
        if self.selected_key() == Some(key) {
            self.clear_selection();
        }

        Some(self.items().write().remove(key))
    }

    /// Select an item by its key
    fn select(&self, key: usize) -> Result<(), String> {
        if self.items().peek().len() <= key {
            return Err("Key out of bounds".to_string());
        }
        self.selected_index().set(Some(key));
        Ok(())
    }

    /// Clear the selection
    fn clear_selection(&self) {
        self.selected_index().set(None)
    }

    /// Remove all items from the collection
    fn clear(&self) {
        self.items().write().clear();
        self.clear_selection();
    }

    /// Select an item by its value
    ///
    /// Recommended to use `Self::select` instead whenever possible.
    /// Not recommended for performance reasons and values may not be unique
    /// If values are not unique, this method selects the first matching item
    fn select_by_value(&self, value: &C) -> Result<(), String>
    where
        C: PartialEq,
    {
        for (i, item) in self.items().iter().enumerate() {
            if &item == value {
                return self.select(i);
            }
        }
        Err("Item not found in collection".to_string())
    }

    /// Select an item by its value
    ///
    /// Recommended to use `Self::select` instead whenever possible.
    /// Not recommended for performance reasons and values may not be unique
    /// If values are not unique, this method selects the first matching item
    fn select_by_display(&self, value: &str) -> Result<(), String>
    where
        C: PartialEq + std::fmt::Display,
    {
        for (i, item) in self.items().iter().enumerate() {
            if item.to_string() == *value {
                return self.select(i);
            }
        }
        Err("Item not found in collection".to_string())
    }
}

#[component]
pub fn StringEditor<C>(s: StoreItem<C>) -> Element
where
    C: std::fmt::Display + Clone + PartialEq + 'static,
{
    rsx! {
        div {
            h4 { "Edit String" }
            input { value: s.read().to_string(), oninput: move |_| {} }
        }
    }
}

#[component]
fn Selector<C>(state: Store<Coll<C>>) -> Element
where
    C: std::fmt::Display + Clone + PartialEq + 'static,
{
    let elem = state.selected();
    rsx! {
        h3 { "Strings" }
        select {
            value: state.selected().map(|s| s.read().to_string()).unwrap_or_default(),
            onchange: move |evt| {
                if let Ok(index) = evt.value().parse::<usize>() {
                    state.write().selected_index = Some(index);
                }
            },
        }
        if let Some(elem) = elem {
            StringEditor::<C> { s: elem }
        }
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Simple Vec<String> collection
    let string_collection = use_store(|| Coll {
        items: vec![
            "Hello".to_string(),
            "Dioxus".to_string(),
            "Store".to_string(),
        ],
        selected_index: None,
    });

    rsx! {
        div {
            Selector { state: string_collection }
        }
    }
}
