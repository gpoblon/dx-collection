use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[derive(Store)]
struct State {
    strs: Vec<String>,
    selected: Option<usize>, // or better: selected_sche: Option<Store<String>>,
}

#[store]
impl<Lens: Writable + Readable> Store<State, Lens> {
    /// Safely selects a str by index with bounds checking
    fn select(&mut self, index: usize) -> Result<(), String> {
        if index >= self.strs().len() {
            Err(format!(
                "String index {} out of bounds (max: {})",
                index,
                self.strs().len() - 1
            ))
        } else {
            self.selected().set(Some(index));
            Ok(())
        }
    }

    fn is_selected(&self, index: usize) -> bool {
        *self.selected().read() == Some(index)
    }

    /// Returns a store for the selected string that can be edited
    fn get_selected_str(
        &self,
    ) -> Option<Store<String, impl Writable<Target = String> + Readable<Target = String> + Clone>>
    {
        let index = self.selected()()?;
        if index >= self.strs().len() {
            return None;
        }
        Some(self.strs().index(index))
    }

    // goal is to remove this function and use get_selected_str instead
    fn get_selected_index(&self) -> Option<usize> {
        self.selected().read().clone()
    }
}

#[component]
fn App() -> Element {
    let mut state = use_store(|| State {
        strs: vec![
            "Hello, world!".to_string(),
            "This is a test string.".to_string(),
            "Dioxus is awesome!".to_string(),
        ],
        selected: None,
    });

    // let s: Store<String> = state.get_selected_str().unwrap();

    rsx! {
        Selector { state }

        // does not work
        // if let Some(s) = state.get_selected_str() {
        //     StringEditor { s, key: "selected-str-{s}" }
        // }

        // works, type coercion from Index -> Store<Schema>
        if let Some(index) = state.get_selected_index() {
            StringEditor { s: state.strs().index(index), key: "selected-str-{index}" }
        }
    }
}

#[component]
fn Selector(state: Store<State>) -> Element {
    rsx! {
        h3 { "Strings" }
        select {
            value: state.get_selected_index().map(|i| i.to_string()).unwrap_or_default(),
            onchange: move |evt| {
                if let Ok(index) = evt.value().parse::<usize>() && index < state.strs().len() {
                    state.write().selected = Some(index);
                }
            },
            // Options populated from strs
            option { value: "", "Choose a str" }
            for (index , str) in state.strs().iter().enumerate() {
                option {
                    key: "{str()}",
                    value: "{index}",
                    selected: state.is_selected(index),
                    "{str()}"
                }
            }
        }
    }
}

#[component]
pub fn StringEditor(s: Store<String>) -> Element {
    rsx! {
        textarea {
            value: s(),
            onchange: move |e| {
                s.set(e.value());
            },
        }
    }
}
