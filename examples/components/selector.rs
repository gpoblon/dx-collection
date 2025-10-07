/// Generic Selector component - Dropdown that displays items and manages selection
/// Works with any collection that supports selection
use dioxus::prelude::*;
use dioxus_collection_store::{Collection, CollectionStore};

#[component]
#[allow(non_snake_case)]
pub fn Selector<C>(collection: CollectionStore<C>) -> Element
where
    C: Collection + Clone + PartialEq + 'static,
    C::Key: std::fmt::Display + Clone + PartialEq,
    C::Value: std::fmt::Display + Clone + PartialEq + 'static,
{
    rsx! {
        div {
            label { "Select: " }
            select {
                value: collection.selected().map(|item| item.read().to_string()).unwrap_or_default(),
                onchange: move |evt| {
                    if !evt.value().is_empty() {
                        if let Err(e) = collection.select_by_display(&evt.value()) {
                            eprintln!("Selection error: {}", e);
                        }
                    } else {
                        collection.clear_selection();
                    }
                },
                option { value: "", "-- Choose --" }
                for item in collection.iter() {
                    option {
                        key: "{item.key()}",
                        value: "{item.read()}",
                        selected: item.is_selected(),
                        "{item.read()}"
                    }
                }
            }
        }
    }
}
