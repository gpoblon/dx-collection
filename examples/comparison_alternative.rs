use dioxus::prelude::*;
use dioxus_collection_store::{CollectionItem, use_collection};
use std::fmt;

mod alternative_design;
use alternative_design::{Coll, StoreItem};
// Import all traits from alternative_design to make store methods available
use alternative_design::*;

// Simple enum for tasks - used across all 4 examples
#[derive(Clone, PartialEq, Debug)]
enum Task {
    Todo(String),
    InProgress(String),
    Done(String),
}

impl Task {
    /// Get the task name regardless of status
    fn name(&self) -> &str {
        match self {
            Task::Todo(s) | Task::InProgress(s) | Task::Done(s) => s,
        }
    }

    /// Check if the task is done
    fn is_done(&self) -> bool {
        matches!(self, Task::Done(_))
    }
}

impl Default for Task {
    fn default() -> Self {
        Task::Todo("New Task".to_string())
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Task::Todo(s) => write!(f, "📋 {}", s),
            Task::InProgress(s) => write!(f, "🔄 {}", s),
            Task::Done(s) => write!(f, "✅ {}", s),
        }
    }
}

// Shared initial data
fn initial_tasks() -> Vec<Task> {
    vec![
        Task::Todo("Write code".to_string()),
        Task::InProgress("Review PR".to_string()),
        Task::Done("Fix bugs".to_string()),
    ]
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { style: "font-family: sans-serif; padding: 20px;",
            h1 { "Dioxus Collection Comparison (with Alternative Design)" }
            p { style: "color: #666; margin-bottom: 20px;",
                "Click on items to select them. Notice how selection management differs across approaches."
            }
            section { style: "margin-bottom: 40px; padding: 20px; border: 2px solid #ccc; border-radius: 8px;",
                h2 { "1. Signal<Vec<T>>: No built-in selection" }
                SignalExample {}
            }
            section { style: "margin-bottom: 40px; padding: 20px; border: 2px solid #99c; border-radius: 8px;",
                h2 { "2. Store<Vec<T>>: Still manual selection" }
                StoreExample {}
            }
            section { style: "margin-bottom: 40px; padding: 20px; border: 2px solid #c9c; border-radius: 8px;",
                h2 { "3. Store<Coll<T>>: Alternative design with built-in selection" }
                AlternativeExample {}
            }
            section { style: "margin-bottom: 40px; padding: 20px; border: 2px solid #9c9; border-radius: 8px;",
                h2 { "4. CollectionStore: Built-in selection with trait-based abstraction" }
                CollectionExample {}
            }
        }
    }
}

#[component]
fn TodoList(
    description: String,
    selected_text: String,
    on_add: EventHandler<()>,
    on_mark_done: EventHandler<()>,
    on_remove: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        div {
            p { style: "color: #666; font-style: italic;", "{description}" }
            p { style: "color: #999; font-size: 0.9em;", "Selected: {selected_text}" }
            div { style: "margin-bottom: 10px;",
                button { onclick: move |_| on_add.call(()), "Add Task" }
                button {
                    style: "margin-left: 10px;",
                    onclick: move |_| on_mark_done.call(()),
                    "Mark Done"
                }
                button {
                    style: "margin-left: 10px; background: #f44336; color: white;",
                    onclick: move |_| on_remove.call(()),
                    "Remove"
                }
            }
            div { style: "margin-top: 10px;", {children} }
        }
    }
}

// ============================================================================
// 1. Signal Approach
// ============================================================================

#[component]
fn SignalExample() -> Element {
    let mut todos = use_signal(initial_tasks);
    // Manual selection tracking - YOU must manage this yourself
    let mut selected_index = use_signal(|| Option::<usize>::None);

    let on_mark_done = selected_index()
        .map(|idx| {
            EventHandler::new(move |_| {
                // Signal: Must manually write() to get mutable access to entire Vec
                let mut todos_write = todos.write();
                // Signal: Must manually bounds-check and call method on Vec element
                if idx < todos_write.len() && !todos_write[idx].is_done() {
                    let task_name = todos_write[idx].name().to_string();
                    todos_write[idx] = Task::Done(task_name);
                }
            })
        })
        .unwrap_or_default();

    let on_remove = selected_index()
        .map(|idx| {
            EventHandler::new(move |_| {
                // Signal: Must manually write() and bounds-check before removing
                let mut todos_write = todos.write();
                if idx < todos_write.len() {
                    todos_write.remove(idx);
                    // Signal: Must manually clear selection
                    selected_index.set(None);
                }
            })
        })
        .unwrap_or_default();

    rsx! {
        TodoList {
            description: "❌ No selection management - must track selected_index manually",
            selected_text: selected_index().map(|i| format!("#{}", i)).unwrap_or("None".to_string()),
            // Signal: Must write() to push to Vec
            on_add: move |_| todos.write().push(Task::default()),
            on_mark_done,
            on_remove,
            // Signal: Must pass 3 separate props to child components
            for (i , _) in todos.read().iter().enumerate() {
                SignalTodoItem {
                    todos,
                    index: i,
                    selected_index,
                    key: "{i}",
                }
            }
        }
    }
}

// Signal: Must manually pass two more variables, including a signal to track selection
#[component]
fn SignalTodoItem(
    todos: Signal<Vec<Task>>,
    index: usize,
    mut selected_index: Signal<Option<usize>>,
) -> Element {
    // Signal: Must manually compare index with selected_index
    let is_selected = selected_index() == Some(index);
    let bg_color = if is_selected { "#ffeb3b" } else { "#f5f5f5" };

    rsx! {
        div {
            style: "padding: 10px; margin: 5px 0; background: {bg_color}; border-radius: 4px; cursor: pointer;",
            // Signal: Must manually set selected_index
            onclick: move |_| selected_index.set(Some(index)),
            span { "{todos.read()[index]}" }
        }
    }
}

// ============================================================================
// 2. Store Approach
// ============================================================================

#[component]
fn StoreExample() -> Element {
    let mut todos = use_store(initial_tasks);
    // Manual selection tracking - Still required with Store
    let mut selected_index = use_signal(|| Option::<usize>::None);

    let on_mark_done = selected_index()
        .and_then(|idx| {
            if idx < todos.len() {
                // Store: Can get() individual item but still need to filter manually
                let current_task = todos.get(idx).map(|t| t()).filter(|t| !t.is_done())?;
                Some(EventHandler::new(move |_| {
                    let task_name = current_task.name().to_string();
                    // Store: get() returns Store<T> wrapper, need mut to call set()
                    if let Some(mut item) = todos.get(idx) {
                        item.set(Task::Done(task_name));
                    }
                }))
            } else {
                None
            }
        })
        .unwrap_or_default();

    // Store doesn't support removal easily
    let on_remove = selected_index()
        .map(|idx| {
            EventHandler::new(move |_| {
                // Store: No remove() on Store<Vec>, must write() to get raw Vec
                let mut todos_write = todos.write();
                if idx < todos_write.len() {
                    todos_write.remove(idx);
                    // Store: Must manually clear selection
                    selected_index.set(None);
                }
            })
        })
        .unwrap_or_default();

    rsx! {
        TodoList {
            description: "❌ No selection management - must track selected_index manually",
            selected_text: selected_index().map(|i| format!("#{}", i)).unwrap_or("None".to_string()),
            // Store: Direct push() method available
            on_add: move |_| todos.push(Task::default()),
            on_mark_done,
            on_remove,
            // Store: Still need to pass 3 separate props to child components
            for (i , todo) in todos.iter().enumerate() {
                StoreTodoItem {
                    todo,
                    index: i,
                    selected_index,
                    key: "{i}",
                }
            }
        }
    }
}

// Store: Still must manually pass two more variables, including a signal to track selection
#[component]
fn StoreTodoItem(
    todo: Store<Task>,
    index: usize,
    mut selected_index: Signal<Option<usize>>,
) -> Element {
    // Store: Must manually check if item is selected
    let is_selected = selected_index() == Some(index);
    let bg_color = if is_selected { "#ffeb3b" } else { "#e5e5ff" };

    rsx! {
        div {
            style: "padding: 10px; margin: 5px 0; background: {bg_color}; border-radius: 4px; cursor: pointer;",
            // Store: Still must manually set selected_index
            onclick: move |_| selected_index.set(Some(index)),
            span { "{todo()}" }
            span { style: "margin-left: 10px; color: #666; font-size: 0.9em;", "(index: {index})" }
        }
    }
}

// ============================================================================
// 3. Alternative Store<Coll<T>> Approach
// ============================================================================

#[component]
fn AlternativeExample() -> Element {
    // Alternative: Must wrap in custom Coll struct
    let todos = use_store(|| Coll::from(initial_tasks()));

    let on_mark_done = todos
        .selected()
        .and_then(|item| {
            // Alternative: Can access selected item directly via StoreItem
            let current_task = item.read()();
            if !current_task.is_done() {
                Some(EventHandler::new(move |_| {
                    let task_name = current_task.name().to_string();
                    // Alternative: Direct set() on StoreItem
                    item.set(Task::Done(task_name));
                }))
            } else {
                None
            }
        })
        .unwrap_or_default();

    let on_remove = todos
        .selected()
        .map(|item| {
            EventHandler::new(move |_| {
                // Alternative: Direct remove() on StoreItem
                let _ = item.remove();
            })
        })
        .unwrap_or_default();

    rsx! {
        TodoList {
            description: "⚠️ Built-in selection, but exposes internal state - users can break it by modifying selected_index directly",
            selected_text: todos
                .selected()
                .map(|item| format!("{}", item.read()()))
                .unwrap_or("None".to_string()),
            // Alternative: Direct push() method available
            on_add: move |_| todos.push(Task::default()),
            on_mark_done,
            on_remove,
            // Alternative: Can use iter() to get StoreItems - only 1 prop needed!
            for item in todos.iter() {
                AlternativeTodoItem { todo: item, key: "{item.key()}" }
            }
        }
    }
}

// Alternative: Must pass store reference to enable selection checking
#[component]
fn AlternativeTodoItem(todo: StoreItem<Task>) -> Element {
    // Alternative: Built-in is_selected() but via wrapper item
    let is_selected = todo.is_selected();
    let bg_color = if is_selected { "#ffeb3b" } else { "#ffe5ff" };

    rsx! {
        div {
            style: "padding: 10px; margin: 5px 0; background: {bg_color}; border-radius: 4px; cursor: pointer;",
            onclick: move |_| {
                let _ = todo.select();
            },
            span { "{todo.read()}" }
        }
    }
}

// ============================================================================
// 4. Collection Approach
// ============================================================================

#[component]
fn CollectionExample() -> Element {
    // CollectionStore: No separate selected_index signal or wrapper struct needed!
    let todos = use_collection(initial_tasks);

    let on_mark_done = todos
        .selected()
        .and_then(|item| {
            // CollectionStore: Direct access to selected item, no index needed
            let current_task = item.read().clone();
            if !current_task.is_done() {
                Some(EventHandler::new(move |_| {
                    let task_name = current_task.name().to_string();
                    // CollectionStore: Safer, direct set() on item, no bounds checking needed
                    item.set(Task::Done(task_name));
                }))
            } else {
                None
            }
        })
        .unwrap_or_default();

    let on_remove = todos
        .selected()
        .map(|item| {
            EventHandler::new(move |_| {
                // CollectionStore: Direct remove() on item
                // Selection is automatically cleared when removing selected item
                item.remove();
            })
        })
        .unwrap_or_default();

    rsx! {
        TodoList {
            description: "✅ Built-in selection with trait abstraction - safe, encapsulated, works with Vec/HashMap/BTreeMap",
            selected_text: todos
                .selected()
                .map(|item| format!("{}", item.read()))
                .unwrap_or("None".to_string()),
            // CollectionStore: Direct push() method available
            on_add: move |_| todos.push(Task::default()),
            on_mark_done,
            on_remove,
            // CollectionStore: Only 1 prop needed - the item itself contains everything!
            for item in todos.iter() {
                CollectionTodoItem { todo: item, key: "{item.key()}" }
            }
        }
    }
}

#[component]
fn CollectionTodoItem(todo: CollectionItem<Vec<Task>>) -> Element {
    // CollectionStore: Built-in is_selected() - no manual comparison needed!
    let is_selected = todo.is_selected();
    let bg_color = if is_selected { "#ffeb3b" } else { "#e5ffe5" };

    rsx! {
        div {
            style: "padding: 10px; margin: 5px 0; background: {bg_color}; border-radius: 4px; cursor: pointer;",
            onclick: {
                let todo = todo.clone();
                move |_| {
                    let _ = todo.select();
                }
            },
            span { "{todo.read()}" }
        }
    }
}
