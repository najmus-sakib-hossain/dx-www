//! Property tests for Node.js events module.
//!
//! Feature: dx-js-compatibility, Property 10: Event Emitter Listener Invocation
//! Validates: Requirements 6.1, 6.2, 6.3
//!
//! Property: For any EventEmitter with N listeners registered for event E,
//! emitting event E SHALL invoke all N listeners exactly once, in registration order.

use dx_compat_node::events::EventEmitter;
use proptest::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property 10: Event Emitter Listener Invocation
    /// For N listeners, emit SHALL invoke all N listeners exactly once.
    #[test]
    fn event_emitter_invokes_all_listeners(n_listeners in 1usize..20) {
        let emitter = EventEmitter::new();
        let counters: Vec<Arc<AtomicUsize>> = (0..n_listeners)
            .map(|_| Arc::new(AtomicUsize::new(0)))
            .collect();

        // Register listeners
        for counter in &counters {
            let counter_clone = Arc::clone(counter);
            emitter.on("test", Box::new(move |_| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }

        // Emit event
        let result = emitter.emit("test", &[]);

        prop_assert!(result, "emit should return true when listeners exist");

        // Verify each listener was called exactly once
        for (i, counter) in counters.iter().enumerate() {
            prop_assert_eq!(
                counter.load(Ordering::SeqCst),
                1,
                "Listener {} should be called exactly once",
                i
            );
        }
    }

    /// Property 10: Event Emitter Listener Invocation - registration order
    /// Listeners SHALL be invoked in registration order.
    #[test]
    fn event_emitter_invokes_in_order(n_listeners in 2usize..10) {
        let emitter = EventEmitter::new();
        let order = Arc::new(std::sync::Mutex::new(Vec::new()));

        // Register listeners that record their index
        for i in 0..n_listeners {
            let order_clone = Arc::clone(&order);
            emitter.on("test", Box::new(move |_| {
                order_clone.lock().unwrap().push(i);
            }));
        }

        // Emit event
        emitter.emit("test", &[]);

        // Verify order
        let recorded_order = order.lock().unwrap();
        let expected_order: Vec<usize> = (0..n_listeners).collect();
        
        prop_assert_eq!(
            &*recorded_order,
            &expected_order,
            "Listeners should be invoked in registration order"
        );
    }

    /// Property 10: Event Emitter Listener Invocation - once listeners
    /// once() listeners SHALL be invoked exactly once, then removed.
    #[test]
    fn event_emitter_once_invoked_once(n_emits in 1usize..10) {
        let emitter = EventEmitter::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        emitter.once("test", Box::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }));

        // Emit multiple times
        for _ in 0..n_emits {
            emitter.emit("test", &[]);
        }

        prop_assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "once listener should be called exactly once regardless of emit count"
        );
    }

    /// Property 10: Event Emitter Listener Invocation - mixed on/once
    /// Mixed on() and once() listeners should work correctly together.
    #[test]
    fn event_emitter_mixed_on_once(
        n_on in 1usize..5,
        n_once in 1usize..5,
        n_emits in 2usize..5
    ) {
        let emitter = EventEmitter::new();
        let on_counters: Vec<Arc<AtomicUsize>> = (0..n_on)
            .map(|_| Arc::new(AtomicUsize::new(0)))
            .collect();
        let once_counters: Vec<Arc<AtomicUsize>> = (0..n_once)
            .map(|_| Arc::new(AtomicUsize::new(0)))
            .collect();

        // Register on() listeners
        for counter in &on_counters {
            let counter_clone = Arc::clone(counter);
            emitter.on("test", Box::new(move |_| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }

        // Register once() listeners
        for counter in &once_counters {
            let counter_clone = Arc::clone(counter);
            emitter.once("test", Box::new(move |_| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }));
        }

        // Emit multiple times
        for _ in 0..n_emits {
            emitter.emit("test", &[]);
        }

        // Verify on() listeners called n_emits times
        for (i, counter) in on_counters.iter().enumerate() {
            prop_assert_eq!(
                counter.load(Ordering::SeqCst),
                n_emits,
                "on() listener {} should be called {} times",
                i,
                n_emits
            );
        }

        // Verify once() listeners called exactly once
        for (i, counter) in once_counters.iter().enumerate() {
            prop_assert_eq!(
                counter.load(Ordering::SeqCst),
                1,
                "once() listener {} should be called exactly once",
                i
            );
        }
    }

    /// Property: emit returns false for non-existent events
    /// emit() on an event with no listeners SHALL return false.
    #[test]
    fn event_emitter_emit_returns_false_for_no_listeners(
        event_name in "[a-zA-Z][a-zA-Z0-9_]{0,20}"
    ) {
        let emitter = EventEmitter::new();
        let result = emitter.emit(&event_name, &[]);
        
        prop_assert!(!result, "emit should return false when no listeners exist");
    }

    /// Property: listener_count is accurate
    /// listener_count() SHALL return the exact number of registered listeners.
    #[test]
    fn event_emitter_listener_count_accurate(n_listeners in 0usize..20) {
        let emitter = EventEmitter::new();

        for _ in 0..n_listeners {
            emitter.on("test", Box::new(|_| {}));
        }

        prop_assert_eq!(
            emitter.listener_count("test"),
            n_listeners,
            "listener_count should return exact number of listeners"
        );
    }

    /// Property: remove_all_listeners clears all listeners
    /// After remove_all_listeners(event), listener_count(event) SHALL be 0.
    #[test]
    fn event_emitter_remove_all_listeners(n_listeners in 1usize..20) {
        let emitter = EventEmitter::new();

        for _ in 0..n_listeners {
            emitter.on("test", Box::new(|_| {}));
        }

        prop_assert_eq!(emitter.listener_count("test"), n_listeners);

        emitter.remove_all_listeners(Some("test"));

        prop_assert_eq!(
            emitter.listener_count("test"),
            0,
            "listener_count should be 0 after remove_all_listeners"
        );
    }

    /// Property: remove_all_listeners(None) clears all events
    /// After remove_all_listeners(None), all events should have 0 listeners.
    #[test]
    fn event_emitter_remove_all_listeners_all_events(
        events in prop::collection::vec("[a-zA-Z]{1,10}", 1..5)
    ) {
        let emitter = EventEmitter::new();

        // Register listeners for multiple events
        for event in &events {
            emitter.on(event, Box::new(|_| {}));
        }

        emitter.remove_all_listeners(None);

        // Verify all events have 0 listeners
        for event in &events {
            prop_assert_eq!(
                emitter.listener_count(event),
                0,
                "All events should have 0 listeners after remove_all_listeners(None)"
            );
        }
    }

    /// Property: event_names returns all registered events
    /// event_names() SHALL return all events that have listeners.
    #[test]
    fn event_emitter_event_names(
        events in prop::collection::hash_set("[a-zA-Z]{1,10}", 1..10)
    ) {
        let emitter = EventEmitter::new();

        for event in &events {
            emitter.on(event, Box::new(|_| {}));
        }

        let names = emitter.event_names();
        let names_set: std::collections::HashSet<_> = names.into_iter().collect();

        prop_assert_eq!(
            names_set.len(),
            events.len(),
            "event_names should return all registered events"
        );

        for event in &events {
            prop_assert!(
                names_set.contains(event.as_str()),
                "event_names should contain '{}'",
                event
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_emitter_new() {
        let emitter = EventEmitter::new();
        assert_eq!(emitter.get_max_listeners(), 10);
    }

    #[test]
    fn test_event_emitter_set_max_listeners() {
        let mut emitter = EventEmitter::new();
        emitter.set_max_listeners(20);
        assert_eq!(emitter.get_max_listeners(), 20);
    }

    #[test]
    fn test_event_emitter_clone() {
        let emitter = EventEmitter::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        emitter.on("test", Box::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        }));

        // Clone shares listeners
        let emitter2 = emitter.clone();
        emitter2.emit("test", &[]);

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_event_emitter_args_passed() {
        let emitter = EventEmitter::new();
        let received = Arc::new(std::sync::Mutex::new(Vec::new()));
        let received_clone = Arc::clone(&received);

        emitter.on("test", Box::new(move |args| {
            *received_clone.lock().unwrap() = args.to_vec();
        }));

        emitter.emit("test", &["arg1".to_string(), "arg2".to_string()]);

        let args = received.lock().unwrap();
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "arg1");
        assert_eq!(args[1], "arg2");
    }
}
