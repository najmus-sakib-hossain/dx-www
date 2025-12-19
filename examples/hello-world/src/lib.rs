//! # Hello World Example
//!
//! Proof-of-concept demonstrating the HTIP protocol.
//!
//! This example creates a simple counter that:
//! 1. Registers a template (HTIP)
//! 2. Clones it to DOM (via batch cloner)
//! 3. Updates state with dirty bits
//! 4. Patches DOM (O(1) update)
//!
//! **NO VDOM. NO DIFFING. NO JSON. PURE BINARY PROTOCOL.**

use dx_www_morph::{ComponentState, CounterState};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub mod htip_demo;

// ============================================================================
// TEMPLATE BINARY DATA (Mock - in production, this comes from compiler)
// ============================================================================

/// Build template binary data
/// Format: count(u32) | [id(u32) | len(u32) | html_bytes]*
fn build_template_binary() -> Vec<u8> {
    let mut binary = Vec::new();

    // Count: 1 template
    binary.extend_from_slice(&1u32.to_le_bytes());

    // Template #1: Counter component
    let template_id = 1u32;
    let html = r#"<div class="counter">
        <h1 id="count-display">0</h1>
        <button id="increment-btn">Increment</button>
        <button id="decrement-btn">Decrement</button>
    </div>"#;
    let html_bytes = html.as_bytes();

    binary.extend_from_slice(&template_id.to_le_bytes());
    binary.extend_from_slice(&(html_bytes.len() as u32).to_le_bytes());
    binary.extend_from_slice(html_bytes);

    binary
}

// ============================================================================
// APPLICATION STATE
// ============================================================================

struct App {
    counter_state: CounterState,
}

impl App {
    fn new() -> Self {
        Self {
            counter_state: CounterState::new(0, 1),
        }
    }

    fn increment(&mut self) {
        self.counter_state.increment();
        log(&format!("Counter: {}", self.counter_state.count));
    }

    fn decrement(&mut self) {
        self.counter_state.count -= self.counter_state.step;
        // Mark dirty via atomic operation on dirty_mask
        use std::sync::atomic::{AtomicU64, Ordering};
        let dirty =
            unsafe { &*((&self.counter_state.dirty_mask) as *const u64 as *const AtomicU64) };
        dirty.fetch_or(1 << CounterState::BIT_COUNT, Ordering::Release);
        log(&format!("Counter: {}", self.counter_state.count));
    }

    fn render(&mut self) {
        if self.counter_state.is_dirty() {
            log("State is dirty, patching DOM...");

            // In production, this would call dx-morph's patcher
            // For now, manually queue update
            dx_www_dom::queue_update_text(1, 0, 0); // Mock text update

            // Update the actual DOM element directly (temporary until binding map is wired)
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(elem) = document.get_element_by_id("count-display") {
                        elem.set_text_content(Some(&self.counter_state.count.to_string()));
                    }
                }
            }

            // Clear dirty bits
            // Clear dirty mask
            use std::sync::atomic::{AtomicU64, Ordering};
            let dirty =
                unsafe { &*((&self.counter_state.dirty_mask) as *const u64 as *const AtomicU64) };
            dirty.swap(0, Ordering::AcqRel);
        }
    }
}

// ============================================================================
// GLOBAL APP INSTANCE
// ============================================================================

thread_local! {
    static APP: RefCell<Option<App>> = RefCell::new(None);
}

fn with_app<F, R>(f: F) -> R
where
    F: FnOnce(&mut App) -> R,
{
    APP.with(|app_cell| f(app_cell.borrow_mut().as_mut().expect("App not initialized")))
}

// ============================================================================
// EVENT HANDLERS (Exported to JS)
// ============================================================================

#[wasm_bindgen]
pub fn handle_increment() {
    with_app(|app| {
        app.increment();
        app.render();
    });
    dx_www_dom::flush_queue();
}

#[wasm_bindgen]
pub fn handle_decrement() {
    with_app(|app| {
        app.decrement();
        app.render();
    });
    dx_www_dom::flush_queue();
}

// ============================================================================
// INITIALIZATION
// ============================================================================

#[wasm_bindgen]
pub fn init_app() {
    // Set panic hook for better error messages
    #[cfg(target_arch = "wasm32")]
    dx_core::panic_hook();

    log("=== dx-www Runtime: Hello World Example ===");
    log("Initializing HTIP Engine...");

    // 1. Register templates
    let template_binary = build_template_binary();
    dx_www_dom::register_templates(&template_binary);
    log("✓ Templates registered");

    // 2. Initialize app state
    APP.with(|app_cell| {
        *app_cell.borrow_mut() = Some(App::new());
    });
    log("✓ App state initialized");

    // 3. Clone template to DOM
    dx_www_dom::queue_clone(1, 0); // Clone template #1, parent=0 (fragment)
    dx_www_dom::flush_to_element("#app");
    log("✓ Initial render complete");

    // 4. Wire up event listeners (in JS)
    log("✓ Ready! Click the buttons to test dirty-bit patching.");
}

#[wasm_bindgen]
pub fn demo_scheduler() {
    log("Starting scheduler demo...");

    // Schedule tasks at different priorities
    let immediate_callback = Closure::wrap(Box::new(|| {
        log("  → Immediate priority task executed");
    }) as Box<dyn FnMut()>);

    let normal_callback = Closure::wrap(Box::new(|| {
        log("  → Normal priority task executed");
    }) as Box<dyn FnMut()>);

    let idle_callback = Closure::wrap(Box::new(|| {
        log("  → Idle priority task executed");
    }) as Box<dyn FnMut()>);

    dx_www_sched::schedule_immediate(immediate_callback.as_ref().unchecked_ref());
    dx_www_sched::schedule_normal(normal_callback.as_ref().unchecked_ref());
    dx_www_sched::schedule_idle(idle_callback.as_ref().unchecked_ref());

    immediate_callback.forget();
    normal_callback.forget();
    idle_callback.forget();

    // Start the scheduler
    dx_www_sched::start_scheduler();

    log("Scheduler started. Check console for task execution logs.");
}

// ============================================================================
// UTILITIES
// ============================================================================

fn log(msg: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&msg.into());
}
