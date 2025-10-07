/// Generic ItemList component - Displays a list of items with inline editing
/// Works with any collection (Vec, HashMap, BTreeMap, CircularBuffer, etc.)
use dioxus::prelude::*;
use dioxus_collection_store::{Collection, CollectionStore};

#[component]
#[allow(non_snake_case)]
pub fn ItemList<C>(
    collection: CollectionStore<C>,
    #[props(default = "No items yet!".to_string())] empty_message: String,
    #[props(default = false)] show_key_label: bool,
) -> Element
where
    C: Collection + Clone + PartialEq + 'static,
    C::Key: std::fmt::Display + Clone + PartialEq,
    C::Value: std::fmt::Display + std::str::FromStr + Clone + PartialEq + 'static,
    <C::Value as std::str::FromStr>::Err: std::fmt::Debug,
{
    rsx! {
        if collection.is_empty() {
            p { "{empty_message}" }
        } else {
            ul { style: "list-style: none; padding: 0;",
                for item in collection.iter() {
                    {
                        let key = item.key();
                        let value = item.read();
                        let is_selected = item.is_selected();
                        let item_for_edit = item.clone();
                        let item_for_remove = item.clone();
                        let border = if is_selected { "2px solid #000" } else { "1px solid #ccc" };
                        rsx! {
                            li {
                                key: "{key}",
                                style: "display: flex; gap: 10px; padding: 5px; border: {border};",
                                if show_key_label {
                                    span { "{key}:" }
                                } else {
                                    span { "[{key}]" }
                                }
                                input {
                                    value: "{value}",
                                    oninput: move |evt| {
                                        if let Ok(parsed) = evt.value().parse::<C::Value>() {
                                            item_for_edit.set(parsed);
                                        }
                                    },
                                }
                                button {
                                    onclick: move |_| {
                                        item_for_remove.remove();
                                    },
                                    "Remove"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
