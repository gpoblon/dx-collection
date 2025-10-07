/// Iterator Demo - Showcasing the power of combining Rust iterators with reactive signals
///
/// This example demonstrates how CollectionStore's iterator provides BOTH read and write
/// capabilities in a single pass, unlike traditional Rust collections that require
/// separate iter() and iter_mut() methods.
use dioxus::prelude::*;
use dioxus_collection_store::use_collection;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Collection of tasks with different priorities
    let tasks = use_collection(|| {
        vec![
            "Review PR".to_string(),
            "URGENT: Fix bug".to_string(),
            "Write tests".to_string(),
            "URGENT: Deploy hotfix".to_string(),
            "Update docs".to_string(),
        ]
    });

    // Filter and mutate
    let process_tasks = move |_| {
        tasks
            .iter()
            .filter(|item| item.read().contains("URGENT"))
            .for_each(|mut item| {
                let mut value = item.write();
                *value = value.replace("URGENT: ", "✓ COMPLETED: ");
            });
    };

    // Conditional mutation based on read
    let add_suffix_to_long = move |_| {
        for mut item in tasks.iter() {
            let should_mutate = {
                let text = item.read();
                text.len() > 15
            };
            if should_mutate {
                item.write().push_str(" [LONG]");
            }
        }
    };

    // Chain iterator methods
    let uppercase_completed = move |_| {
        tasks
            .iter()
            .filter(|item| item.read().contains("COMPLETED"))
            .for_each(|item| {
                let current = item.read().clone();
                item.set(current.to_uppercase());
            });
    };

    // Count, filter, and find
    let count_stats = move |_| {
        let total = tasks.len();

        let urgent_count = tasks
            .iter()
            .filter(|item| item.read().contains("URGENT"))
            .count();

        let completed_count = tasks
            .iter()
            .filter(|item| item.read().contains("COMPLETED"))
            .count();

        let first_urgent = tasks
            .iter()
            .find(|item| item.read().contains("URGENT"))
            .map(|item| item.read().clone());

        println!("📊 Stats:");
        println!("  Total: {}", total);
        println!("  Urgent: {}", urgent_count);
        println!("  Completed: {}", completed_count);
        if let Some(task) = first_urgent {
            println!("  First urgent: {}", task);
        }
    };

    let reset = move |_| {
        tasks.clear();
        tasks.extend(vec![
            (0, "Review PR".to_string()),
            (1, "URGENT: Fix bug".to_string()),
            (2, "Write tests".to_string()),
            (3, "URGENT: Deploy hotfix".to_string()),
            (4, "Update docs".to_string()),
        ]);
    };

    rsx! {
        div { style: "padding: 20px; font-family: monospace; max-width: 800px;",
            h1 { "Iterator Example" }

            div { style: "margin: 20px 0; padding: 15px; border: 1px solid #ccc;",
                h2 { "Current Tasks" }
                if tasks.is_empty() {
                    p { "No tasks" }
                } else {
                    ul { style: "list-style: none; padding: 0;",
                        for item in tasks.iter() {
                            {
                                let text = item.read().clone();
                                let is_urgent = text.contains("URGENT");
                                let is_completed = text.contains("COMPLETED");
                                let color = if is_completed {
                                    "#28a745"
                                } else if is_urgent {
                                    "#dc3545"
                                } else {
                                    "#333"
                                };
                                rsx! {
                                    li {
                                        key: "{item.key()}",
                                        style: "padding: 8px; margin: 5px 0; background: #f9f9f9; border-left: 3px solid {color};",
                                        span { style: "color: {color}; font-weight: bold;",
                                            if is_urgent {
                                                "⚠️ "
                                            }
                                            if is_completed {
                                                "✓ "
                                            }
                                            "{text}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            div { style: "margin-top: 20px;",
                h3 { "Operations" }
                div { style: "display: flex; flex-wrap: wrap; gap: 10px; margin-top: 10px;",
                    button {
                        style: "padding: 10px 15px; background: #007acc; color: white; border: none; cursor: pointer;",
                        onclick: process_tasks,
                        "Complete Urgent"
                        br {}
                        span { style: "font-size: 0.8em;", "filter + mutate" }
                    }
                    button {
                        style: "padding: 10px 15px; background: #28a745; color: white; border: none; cursor: pointer;",
                        onclick: add_suffix_to_long,
                        "Tag Long Tasks"
                        br {}
                        span { style: "font-size: 0.8em;", "conditional write" }
                    }
                    button {
                        style: "padding: 10px 15px; background: #ffc107; color: black; border: none; cursor: pointer;",
                        onclick: uppercase_completed,
                        "Uppercase Done"
                        br {}
                        span { style: "font-size: 0.8em;", "chain operations" }
                    }
                    button {
                        style: "padding: 10px 15px; background: #6c757d; color: white; border: none; cursor: pointer;",
                        onclick: count_stats,
                        "Show Stats"
                        br {}
                        span { style: "font-size: 0.8em;", "count + find" }
                    }
                    button {
                        style: "padding: 10px 15px; background: #dc3545; color: white; border: none; cursor: pointer;",
                        onclick: reset,
                        "Reset"
                    }
                }
            }

            div { style: "margin-top: 30px; padding: 15px; background: #e7f3ff; border: 1px solid #007acc;",
                h3 { "Code Examples" }
                details { style: "margin: 10px 0;",
                    summary { style: "cursor: pointer; font-weight: bold;", "Example 1: Filter + Mutate" }
                    pre { style: "background: #f5f5f5; padding: 10px; overflow-x: auto;",
                        code {
                            "tasks.iter()
    .filter(|item| item.read().contains(\"URGENT\"))
    .for_each(|item| {{
        item.write().replace(\"URGENT: \", \"✓ COMPLETED: \");
    }});"

                        }
                    }
                }
                details { style: "margin: 10px 0;",
                    summary { style: "cursor: pointer; font-weight: bold;", "Example 2: Mixed Read/Write" }
                    pre { style: "background: #f5f5f5; padding: 10px; overflow-x: auto;",
                        code {
                            "for item in tasks.iter() {{
    let text = item.read();
    if text.len() > 15 {{
        item.write().push_str(\" [LONG]\");
    }}
}}"
                        }
                    }
                }
                details { style: "margin: 10px 0;",
                    summary { style: "cursor: pointer; font-weight: bold;", "Example 3: Count + Find" }
                    pre { style: "background: #f5f5f5; padding: 10px; overflow-x: auto;",
                        code {
                            "let count = tasks.iter()
    .filter(|item| item.read().contains(\"URGENT\"))
    .count();

let found = tasks.iter()
    .find(|item| item.read().len() > 20);"
                        }
                    }
                }
            }
        }
    }
}
