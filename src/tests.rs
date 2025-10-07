use crate::*;
use dioxus::prelude::*;
use std::collections::HashMap;

// Helper macro to run tests within a Dioxus runtime context
macro_rules! test_with_runtime {
    ($test_fn:expr) => {{
        let mut dom = VirtualDom::new(move || {
            use_hook(|| $test_fn());
            rsx! { div {} }
        });
        let _ = dom.rebuild_in_place();
    }};
}

#[test]
fn test_collection_store_vec_operations() {
    test_with_runtime!(|| {
        // Create a store with a Vec
        let store = CollectionStore::new(vec!["hello".to_string(), "world".to_string()]);

        // Test first/last
        assert_eq!(&*store.first().unwrap().read(), "hello");
        assert_eq!(&*store.last().unwrap().read(), "world");

        // Test get_item - using new API without clones
        assert_eq!(&*store.get(&0).read(), "hello");
        assert_eq!(&*store.get(&1).read(), "world");
        assert!(!store.contains_key(&2));

        // Test contains_key
        assert!(store.contains_key(&0));
        assert!(store.contains_key(&1));
        assert!(!store.contains_key(&2));

        // Test push (SequentialCollection)
        store.push("rust".to_string());
        assert_eq!(store.len(), 3);
        assert_eq!(&*store.get(&2).read(), "rust");

        // Test insert (replace existing)
        let old = store.insert(1, "dioxus".to_string());
        assert_eq!(old, Some("world".to_string()));
        assert_eq!(&*store.get(&1).read(), "dioxus");

        // Test swap
        store.swap(&0, &2).ok();
        assert_eq!(&*store.first().unwrap().read(), "rust");
        assert_eq!(&*store.last().unwrap().read(), "hello");

        // Test pop
        assert_eq!(store.pop(), Some("hello".to_string()));
        assert_eq!(store.len(), 2);

        // Test remove
        let removed = store.remove(&0);
        assert_eq!(removed, Some("rust".to_string()));
        assert_eq!(store.len(), 1);

        // Test is_empty
        assert!(!store.is_empty());

        // Test clear
        store.clear();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
        assert!(store.first().is_none());
        assert!(store.last().is_none());
    });
}

#[test]
fn test_collection_store_hashmap_operations() {
    test_with_runtime!(|| {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), 100);
        map.insert("key2".to_string(), 200);

        let store = CollectionStore::new(map);

        // Test get_item - new API without clones
        assert_eq!(*store.get(&"key1".to_string()).read(), 100);
        assert_eq!(*store.get(&"key2".to_string()).read(), 200);
        assert!(!store.contains_key(&"key3".to_string()));

        // Test insert
        let old = store.insert("key3".to_string(), 300);
        assert_eq!(old, None);
        assert_eq!(*store.get(&"key3".to_string()).read(), 300);

        // Test extend
        store.extend(vec![("key4".to_string(), 400), ("key5".to_string(), 500)]);
        assert_eq!(store.len(), 5);

        // Test insert with existing key
        let old = store.insert("key1".to_string(), 150);
        assert_eq!(old, Some(100));
        assert_eq!(*store.get(&"key1".to_string()).read(), 150);

        // Test remove
        let removed = store.remove(&"key2".to_string());
        assert_eq!(removed, Some(200));
        assert!(!store.contains_key(&"key2".to_string()));

        // Test clear
        store.clear();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    });
}

#[test]
fn test_collection_store_selection() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30]);

        // Initially no selection
        assert_eq!(store.selected_key(), None);
        assert!(store.selected().is_none());

        // Select by key
        assert!(store.select(&1).is_ok());
        assert_eq!(store.selected_key(), Some(1));
        let selected = store.selected().unwrap();
        assert_eq!(*selected.read(), 20);

        // Select by value
        assert!(store.select_by_value(&30).is_ok());
        assert_eq!(store.selected_key(), Some(2));
        let selected = store.selected().unwrap();
        assert_eq!(*selected.read(), 30);

        // Select invalid key
        assert!(store.select(&10).is_err());

        // Select invalid value
        assert!(store.select_by_value(&999).is_err());

        // Clear selection
        store.clear_selection();
        assert_eq!(store.selected_key(), None);
    });
}

#[test]
fn test_collection_store_select_by_display() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ]);

        // Select by display string
        assert!(store.select_by_display("banana").is_ok());
        assert_eq!(store.selected_key(), Some(1));
        let selected = store.selected().unwrap();
        assert_eq!(&*selected.read(), "banana");

        // Select non-existent value
        assert!(store.select_by_display("durian").is_err());
    });
}

#[test]
fn test_collection_store_iter() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![1, 2, 3]);

        let items: Vec<_> = store
            .iter()
            .map(|item| (item.key(), *item.read()))
            .collect();
        assert_eq!(items, vec![(0, 1), (1, 2), (2, 3)]);
    });
}

#[test]
fn test_readonly_item_operations() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![100, 200, 300]);

        // Select an item first
        store.select(&1).unwrap();

        // Get item through iter
        let item = store.iter().nth(1).unwrap();

        assert_eq!(item.key(), 1);
        assert_eq!(*item.read(), 200);
        assert!(item.is_selected());

        // Check non-selected item
        let item0 = store.iter().next().unwrap();
        assert!(!item0.is_selected());
    });
}

#[test]
fn test_readonly_item_select_and_remove() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30, 40]);

        // Get an item and select it
        let item = store.iter().nth(2).unwrap();
        assert!(item.select().is_ok());
        let selected = store.selected().unwrap();
        assert_eq!(*selected.read(), 30);

        // Remove an item
        let item1 = store.iter().nth(1).unwrap();
        let removed = item1.remove();
        assert_eq!(removed, Some(20));
        assert_eq!(store.len(), 3);
    });
}

#[test]
fn test_item_write_and_set() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30, 40]);

        // Get an item and insert to it
        let item = store.iter().nth(1).unwrap();
        assert_eq!(*item.read(), 20);

        // Use set to update value
        item.set(99);
        assert_eq!(*store.get(&1).read(), 99);

        // Use set again to update value
        item.set(88);
        assert_eq!(*store.get(&1).read(), 88);

        // Test peek on item
        let item2 = store.iter().nth(2).unwrap();
        assert_eq!(*item2.peek(), 30);
        assert_eq!(*item2.read(), 30);
    });
}

#[test]
fn test_item_write_with_hashmap() {
    test_with_runtime!(|| {
        let mut map = std::collections::HashMap::new();
        map.insert("foo".to_string(), 100);
        map.insert("bar".to_string(), 200);

        let store = CollectionStore::new(map);

        // Find and update an item
        let item = store.iter().find(|i| i.key() == "foo").unwrap();
        assert_eq!(*item.read(), 100);

        // Set new value
        item.set(999);
        assert_eq!(*store.get(&"foo".to_string()).read(), 999);

        // Set new value again
        item.set(888);
        assert_eq!(*store.get(&"foo".to_string()).read(), 888);
    });
}

#[test]
fn test_read_insert_api() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![100, 200, 300]);

        // Test read - new API returns signal
        assert_eq!(*store.read(&0).read(), 100);
        assert_eq!(*store.read(&1).read(), 200);
        assert!(!store.contains_key(&5));

        // Test insert - update existing
        store.insert(1, 250);
        assert_eq!(*store.read(&1).read(), 250);

        // Test insert - insert at end
        store.insert(3, 400);
        assert_eq!(*store.read(&3).read(), 400);
        assert_eq!(store.len(), 4);

        // Verify insert returns previous value
        store.insert(0, 999);
        assert_eq!(*store.read(&0).read(), 999);
    });
}

#[test]
fn test_read_insert_with_hashmap() {
    test_with_runtime!(|| {
        let mut map = std::collections::HashMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);

        let store = CollectionStore::new(map);

        // Test read - new API
        assert_eq!(*store.read(&"a".to_string()).read(), 1);
        assert!(!store.contains_key(&"c".to_string()));

        // Test insert
        store.insert("a".to_string(), 10);
        assert_eq!(*store.read(&"a".to_string()).read(), 10);

        store.insert("c".to_string(), 3);
        assert_eq!(*store.read(&"c".to_string()).read(), 3);
    });
}

#[test]
fn test_set_api() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30]);

        // Test set - update existing
        store.set(1, 42);
        assert_eq!(*store.read(&1).read(), 42);

        // Test set with valid indices
        store.set(0, 99);
        assert_eq!(*store.read(&0).read(), 99);

        store.set(2, 77);
        assert_eq!(*store.read(&2).read(), 77);

        // Test set with out of bounds index (should fail)
        store.set(10, 100);
        assert_eq!(store.len(), 3); // Length unchanged

        // Verify original values were updated
        assert_eq!(*store.read(&0).read(), 99);
        assert_eq!(*store.read(&1).read(), 42);
        assert_eq!(*store.read(&2).read(), 77);
    });
}

#[test]
fn test_set_with_hashmap() {
    test_with_runtime!(|| {
        let mut map = std::collections::HashMap::new();
        map.insert("x".to_string(), 1);
        map.insert("y".to_string(), 2);

        let store = CollectionStore::new(map);

        // Test set on existing keys
        store.set("x".to_string(), 100);
        assert_eq!(*store.read(&"x".to_string()).read(), 100);

        store.set("y".to_string(), 200);
        assert_eq!(*store.read(&"y".to_string()).read(), 200);

        // Test set on non-existent key (should fail)
        store.set("z".to_string(), 300);
        assert!(!store.contains_key(&"z".to_string()));

        // Verify length unchanged
        assert_eq!(store.len(), 2);
    });
}

#[test]
fn test_peek_api() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![100, 200, 300]);

        // Test peek - new API returns signal
        assert_eq!(*store.peek(&0).peek(), 100);
        assert_eq!(*store.peek(&1).peek(), 200);
        assert_eq!(*store.peek(&2).peek(), 300);
        assert!(!store.contains_key(&5));

        // Peek should return same values as read
        assert_eq!(*store.peek(&1).peek(), *store.read(&1).read());

        // Modify and peek again
        store.insert(1, 999);
        assert_eq!(*store.peek(&1).peek(), 999);
    });
}

#[test]
fn test_peek_with_hashmap() {
    test_with_runtime!(|| {
        let mut map = std::collections::HashMap::new();
        map.insert("alpha".to_string(), 10);
        map.insert("beta".to_string(), 20);

        let store = CollectionStore::new(map);

        // Test peek with HashMap
        assert_eq!(*store.peek(&"alpha".to_string()).peek(), 10);
        assert_eq!(*store.peek(&"beta".to_string()).peek(), 20);
        assert!(!store.contains_key(&"gamma".to_string()));

        // Peek should match read
        assert_eq!(
            *store.peek(&"alpha".to_string()).peek(),
            *store.read(&"alpha".to_string()).read()
        );
    });
}

#[test]
fn test_items_lens() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30]);

        // Get items lens
        let mut items = store.items();

        // Read through lens
        assert_eq!(items.read().len(), 3);
        assert_eq!(items.read().get(&0), Some(&10));

        // Write through lens
        let mut items_write = items.write();
        items_write.insert(1, 99);
        drop(items_write);
        assert_eq!(*store.read(&1).read(), 99);
    });
}

#[test]
fn test_selected_signal_lens() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30]);

        // Get selection lens
        let mut selected = store.selected_key_signal();

        // Initially no selection
        assert_eq!(*selected.read(), None);

        // Select an item
        store.select(&1).ok();
        assert_eq!(*selected.read(), Some(1));

        // Clear selection through lens
        let mut sel_write = selected.write();
        *sel_write = None;
        drop(sel_write);
        assert_eq!(store.selected_key(), None);
    });
}

#[test]
fn test_item_kv_method() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
        ]);

        // Test kv() method on individual items
        let item = store.get(&0);
        let (key, value) = item.kv();
        assert_eq!(key, 0);
        assert_eq!(value, "first");

        // Test kv() with selected item
        store.select(&1).ok();
        if let Some(selected_item) = store.selected() {
            let (key, value) = selected_item.kv();
            assert_eq!(key, 1);
            assert_eq!(value, "second");
        } else {
            panic!("Expected a selected item");
        }

        // Test with hashmap
        let mut map = HashMap::new();
        map.insert("name".to_string(), 42);
        map.insert("age".to_string(), 100);
        let store2 = CollectionStore::new(map);

        let item = store2.get(&"name".to_string());
        let (key, value) = item.kv();
        assert_eq!(key, "name");
        assert_eq!(value, 42);
    });
}

#[test]
fn test_collection_store_btreemap_operations() {
    use std::collections::BTreeMap;
    test_with_runtime!(|| {
        let mut map = BTreeMap::new();
        map.insert("key1".to_string(), 100);
        map.insert("key2".to_string(), 200);

        let store = CollectionStore::new(map);

        // Test get_item
        assert_eq!(*store.get(&"key1".to_string()).read(), 100);
        assert_eq!(*store.get(&"key2".to_string()).read(), 200);
        assert!(!store.contains_key(&"key3".to_string()));

        // Test insert
        let old = store.insert("key3".to_string(), 300);
        assert_eq!(old, None);
        assert_eq!(*store.get(&"key3".to_string()).read(), 300);

        // Test insert with existing key
        let old = store.insert("key1".to_string(), 150);
        assert_eq!(old, Some(100));
        assert_eq!(*store.get(&"key1".to_string()).read(), 150);

        // Test remove
        let removed = store.remove(&"key2".to_string());
        assert_eq!(removed, Some(200));
        assert!(!store.contains_key(&"key2".to_string()));

        // Test keys are sorted (using iterator)
        let keys: Vec<String> = store.iter().map(|item| item.key()).collect();
        assert_eq!(keys, vec!["key1".to_string(), "key3".to_string()]);
    });
}

#[test]
fn test_collection_store_clear() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![1, 2, 3, 4, 5]);

        assert_eq!(store.len(), 5);
        store.clear();
        assert!(store.is_empty());
        assert_eq!(store.len(), 0);
    });
}

#[test]
fn test_collection_store_extend() {
    use std::collections::HashMap;
    test_with_runtime!(|| {
        let store = CollectionStore::new(HashMap::new());

        store.extend(vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
            ("c".to_string(), 3),
        ]);

        assert_eq!(store.len(), 3);
        assert_eq!(*store.get(&"a".to_string()).read(), 1);
        assert_eq!(*store.get(&"b".to_string()).read(), 2);
        assert_eq!(*store.get(&"c".to_string()).read(), 3);
    });
}

#[test]
fn test_sequential_pop() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![1, 2, 3]);

        assert_eq!(store.pop(), Some(3));
        assert_eq!(store.len(), 2);
        assert_eq!(store.pop(), Some(2));
        assert_eq!(store.pop(), Some(1));
        assert_eq!(store.pop(), None);
        assert!(store.is_empty());
    });
}

#[test]
fn test_sequential_first_last() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30]);

        let first = store.first().unwrap();
        assert_eq!(*first.read(), 10);

        let last = store.last().unwrap();
        assert_eq!(*last.read(), 30);

        // Test with empty collection
        store.clear();
        assert!(store.first().is_none());
        assert!(store.last().is_none());
    });
}

#[test]
fn test_sequential_swap() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![1, 2, 3, 4, 5]);

        // Swap first and last
        assert!(store.swap(&0, &4).is_ok());
        assert_eq!(*store.get(&0).read(), 5);
        assert_eq!(*store.get(&4).read(), 1);

        // Swap middle elements
        assert!(store.swap(&1, &3).is_ok());
        assert_eq!(*store.get(&1).read(), 4);
        assert_eq!(*store.get(&3).read(), 2);

        // Try swap with invalid key
        assert!(store.swap(&0, &99).is_err());
    });
}

#[test]
fn test_remove_selected_item_clears_selection() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30]);

        // Select an item
        store.select(&1).unwrap();
        assert_eq!(store.selected_key(), Some(1));

        // Remove the selected item
        let removed = store.remove(&1);
        assert_eq!(removed, Some(20));

        // Selection should be cleared
        assert_eq!(store.selected_key(), None);
        assert!(store.selected().is_none());
    });
}

#[test]
fn test_remove_last_item_when_selected() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30]);

        // Select the last item
        store.select(&2).unwrap();
        assert_eq!(store.selected_key(), Some(2));

        // Remove the last (selected) item
        let removed = store.remove(&2);
        assert_eq!(removed, Some(30));
        assert_eq!(store.len(), 2);

        // Selection should be cleared
        assert_eq!(store.selected_key(), None);
        assert!(store.selected().is_none());

        // Verify we can still access remaining items
        assert_eq!(*store.get(&0).read(), 10);
        assert_eq!(*store.get(&1).read(), 20);
    });
}

#[test]
fn test_item_remove_clears_selection() {
    test_with_runtime!(|| {
        let store = CollectionStore::new(vec![10, 20, 30, 40]);

        // Get the third item and select it
        let item = store.get(&2);
        item.select().unwrap();
        assert_eq!(store.selected_key(), Some(2));

        // Remove it via the item
        let removed = item.remove();
        assert_eq!(removed, Some(30));

        // Selection should be cleared
        assert_eq!(store.selected_key(), None);

        // Verify we can still access other items
        assert_eq!(store.len(), 3);
        assert_eq!(*store.get(&0).read(), 10);
        assert_eq!(*store.get(&1).read(), 20);
    });
}
