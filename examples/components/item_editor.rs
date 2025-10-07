/// Generic ItemEditor component - Input field for editing any parseable type
/// Automatically parses and updates the value on change
use dioxus::prelude::*;

#[component]
#[allow(non_snake_case)]
pub fn ItemEditor<T>(item: Store<T>) -> Element
where
    T: std::fmt::Display + std::str::FromStr + Clone + 'static,
    T::Err: std::fmt::Debug,
{
    rsx! {
        div { style: "display: flex; align-items: center; gap: 10px;",
            span { style: "font-weight: bold; color: #2c3e50;", "Value:" }
            input {
                style: "flex: 1; padding: 10px 15px; border: 2px solid #3498db; border-radius: 5px; font-size: 1.1em; color: #2c3e50;",
                value: item().to_string(),
                onchange: move |e| {
                    match e.value().parse::<T>() {
                        Ok(parsed) => {
                            item.set(parsed);
                        }
                        Err(err) => {
                            eprintln!("Parse error: {:?}", err);
                        }
                    }
                },
            }
            span { style: "color: #95a5a6; font-size: 0.9em;", "✓ Auto-saves on change" }
        }
    }
}
