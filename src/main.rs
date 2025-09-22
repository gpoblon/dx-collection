use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[derive(Store)]
struct Collection<T> {
    items: Vec<T>,
    selected: Option<usize>,
}

#[store]
impl<T: 'static, Lens> Store<Collection<T>, Lens> {
    /// Safely selects an item by index with bounds checking
    fn select(&mut self, index: usize) -> Result<(), String> {
        if self.items().len() <= index {
            return Err(format!(
                "Item index {} out of bounds (max: {})",
                index,
                self.items().len() - 1
            ));
        }
        self.selected().set(Some(index));
        Ok(())
    }

    fn is_selected(&self, index: usize) -> bool {
        *self.selected().read() == Some(index)
    }

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
        let index = self.selected()()?;
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
    let string_state = use_store(|| Collection {
        items: vec![
            "Hello, world!".to_string(),
            "This is a test string.".to_string(),
            "Dioxus is awesome!".to_string(),
        ],
        selected: None,
    });

    let number_state = use_store(|| Collection {
        items: vec![42, 100, 256, 512],
        selected: None,
    });

    rsx! {
        div {
            h2 { "String Collection" }
            Selector { state: string_state }

            if let Some(s) = string_state.get_selected_item() {
                ItemEditor { item: s, key: "selected-str-{s}" }
            }
        }

        div {
            h2 { "Number Collection" }
            Selector { state: number_state }

            if let Some(num) = number_state.get_selected_item() {
                ItemEditor { item: num, key: "selected-num-{num}" }
            }
        }
    }
}

#[component]
fn Selector<T>(state: Store<Collection<T>>) -> Element
where
    T: std::fmt::Display + Clone + 'static,
{
    rsx! {
        h3 { "Items" }
        select {
            value: state.get_selected_item().map(|item| item.to_string()).unwrap_or_default(),
            onchange: move |evt| {
                if let Ok(index) = evt.value().parse::<usize>() && state.items().len() > index {
                    state.write().selected = Some(index);
                }
            },
            // Options populated from items
            option { value: "", "Choose an item" }
            for (index , item) in state.items().iter().enumerate() {
                option {
                    key: "{item()}",
                    value: "{index}",
                    selected: state.is_selected(index),
                    "{item()}"
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
