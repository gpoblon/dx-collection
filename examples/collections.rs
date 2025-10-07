/// Complete demonstration of CollectionStore with generic components
/// Shows Vec<T>, HashMap<K, V>, BTreeMap<K, V>, and CircularBuffer<T> using reusable components
use dioxus::prelude::*;
use dioxus_collection_store::{SequentialCollection, use_collection};
use std::collections::{BTreeMap, HashMap};

// Import shared UI components
mod components;
use components::{ItemList, Selector};

// Import custom collection implementation
#[path = "lib/custom_collection.rs"]
mod custom_collection;
use custom_collection::CircularBuffer;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 1. Vec<String> collection
    let tasks = use_collection(|| {
        vec![
            "Buy groceries".to_string(),
            "Write documentation".to_string(),
            "Review pull request".to_string(),
        ]
    });

    // 2. HashMap<String, i32> collection
    let score_store = use_collection(|| {
        let mut scores = HashMap::new();
        scores.insert("Alice".to_string(), 95);
        scores.insert("Bob".to_string(), 87);
        scores.insert("Charlie".to_string(), 92);
        scores
    });

    // 3. BTreeMap<String, i32> collection (sorted by key)
    let rankings = use_collection(|| {
        let mut tree = BTreeMap::new();
        tree.insert("First".to_string(), 100);
        tree.insert("Second".to_string(), 95);
        tree.insert("Third".to_string(), 90);
        tree
    });

    // 4. CircularBuffer<String> collection (max 5 items)
    let logs = use_collection(|| {
        let mut log_buffer = CircularBuffer::new(5);
        SequentialCollection::push(&mut log_buffer, "System started".to_string());
        SequentialCollection::push(&mut log_buffer, "User logged in".to_string());
        SequentialCollection::push(&mut log_buffer, "Data loaded".to_string());
        log_buffer
    });

    rsx! {
        div { style: "padding: 20px; font-family: sans-serif; max-width: 1200px;",
            h1 { "Collection Store Example" }
            p { "Vec, HashMap, BTreeMap, and CircularBuffer with generic components" }

            // Vec<String> Example
            div { style: "margin: 20px 0; padding: 15px; border: 1px solid #ccc;",
                h2 { "Task List - Vec<String>" }
                p { "Dynamic vector with add/remove operations" }
                Selector { collection: tasks }
                ItemList {
                    collection: tasks,
                    empty_message: "No tasks yet!".to_string(),
                }
                div { style: "margin-top: 10px;",
                    button {
                        style: "padding: 8px 16px; margin-right: 10px;",
                        onclick: move |_| {
                            let task_num = tasks.len() + 1;
                            tasks.push(format!("Task #{}", task_num));
                        },
                        "Add Task"
                    }
                    span { "Total: {tasks.len()} tasks" }
                }
            }

            // HashMap<String, i32> Example
            div { style: "margin: 20px 0; padding: 15px; border: 1px solid #ccc;",
                h2 { "Score Board - HashMap<String, i32>" }
                p { "Key-value mapping with name → score" }
                Selector { collection: score_store }
                ItemList {
                    collection: score_store,
                    empty_message: "No scores yet!".to_string(),
                    show_key_label: true,
                }
                div { style: "margin-top: 10px;",
                    button {
                        style: "padding: 8px 16px; margin-right: 10px;",
                        onclick: move |_| {
                            let names = ["David", "Eve", "Frank", "Grace", "Henry"];
                            let random_name = names[score_store.iter().count() % names.len()];
                            let random_score = 70 + (score_store.iter().count() * 7) % 30;
                            score_store.insert(random_name.to_string(), random_score as i32);
                        },
                        "Add Score"
                    }
                    span { "Total: {score_store.iter().count()} entries" }
                }
            }

            // BTreeMap<String, i32> Example
            div { style: "margin: 20px 0; padding: 15px; border: 1px solid #ccc;",
                h2 { "Rankings - BTreeMap<String, i32>" }
                p { "Sorted key-value mapping (keys are automatically sorted)" }
                Selector { collection: rankings }
                ItemList {
                    collection: rankings,
                    empty_message: "No rankings yet!".to_string(),
                    show_key_label: true,
                }
                div { style: "margin-top: 10px;",
                    button {
                        style: "padding: 8px 16px; margin-right: 10px;",
                        onclick: move |_| {
                            let positions = [
                                "First",
                                "Second",
                                "Third",
                                "Fourth",
                                "Fifth",
                                "Sixth",
                                "Seventh",
                                "Eighth",
                            ];
                            let count = rankings.iter().count();
                            if let Some(position) = positions.get(count) {
                                let score = 100 - (count as i32) * 5;
                                rankings.insert(position.to_string(), score);
                            }
                        },
                        "Add Ranking"
                    }
                    span { "Total: {rankings.iter().count()} entries" }
                }
            }

            // CircularBuffer<String> Example
            div { style: "margin: 20px 0; padding: 15px; border: 1px solid #ccc;",
                h2 { "Event Log - CircularBuffer<String> (Max 5)" }
                p { "Custom collection that overwrites oldest items when full" }
                Selector { collection: logs }
                ItemList {
                    collection: logs,
                    empty_message: "No log entries yet!".to_string(),
                }
                div { style: "margin-top: 10px;",
                    button {
                        style: "padding: 8px 16px; margin-right: 10px;",
                        onclick: move |_| {
                            let events = [
                                "New message received",
                                "File uploaded",
                                "Settings updated",
                                "Cache cleared",
                                "Backup completed",
                                "User action detected",
                            ];
                            let event = events[logs.len() % events.len()];
                            logs.push(format!("{} ({})", event, logs.len() + 1));
                        },
                        "Add Log"
                    }
                    span { "{logs.len()}/5 slots used" }
                    if logs.len() >= 5 {
                        span { style: "margin-left: 10px; padding: 4px 8px; background: #f0f0f0;",
                            "⚠️ Buffer Full"
                        }
                    }
                }
            }

            // Info Footer
            div { style: "margin-top: 30px; padding: 15px; background: #f5f5f5;",
                h3 { "About This Example" }
                p { "Generic components used:" }
                ul {
                    li {
                        code { "ItemList<C>" }
                        ": Works with all collection types (Vec, HashMap, BTreeMap, CircularBuffer)"
                    }
                    li {
                        code { "Selector<C>" }
                        ": Interactive selection for any collection"
                    }
                }
                p { "All collections use the same CollectionStore<C> interface." }
            }
        }
    }
}
