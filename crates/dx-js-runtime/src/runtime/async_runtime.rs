//! Async Runtime - Event Loop, Promises, Timers
//!
//! Implements:
//! - Event loop with microtask/macrotask queues
//! - Promise constructor and methods
//! - async/await support
//! - setTimeout/setInterval/setImmediate
//! - I/O integration

use crate::value::Value;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Promise state
#[derive(Debug, Clone, PartialEq)]
pub enum PromiseState {
    Pending,
    Fulfilled(Value),
    Rejected(Value),
}

/// A JavaScript Promise
#[derive(Debug, Clone)]
pub struct Promise {
    pub state: PromiseState,
    pub then_callbacks: Vec<usize>, // Function pointers
    pub catch_callbacks: Vec<usize>,
}

impl Promise {
    pub fn new() -> Self {
        Self {
            state: PromiseState::Pending,
            then_callbacks: Vec::new(),
            catch_callbacks: Vec::new(),
        }
    }

    pub fn resolve(&mut self, value: Value) {
        if matches!(self.state, PromiseState::Pending) {
            self.state = PromiseState::Fulfilled(value);
            // Trigger then callbacks
        }
    }

    pub fn reject(&mut self, reason: Value) {
        if matches!(self.state, PromiseState::Pending) {
            self.state = PromiseState::Rejected(reason);
            // Trigger catch callbacks
        }
    }
}

/// Task types
pub enum Task {
    Microtask(Box<dyn FnOnce() + Send>),
    Macrotask(Box<dyn FnOnce() + Send>),
    Timer(TimerTask),
}

#[derive(Debug)]
pub struct TimerTask {
    pub callback: usize, // Function pointer
    pub execute_at: Instant,
    pub interval: Option<Duration>,
    pub id: u32,
}

/// Event loop
pub struct EventLoop {
    /// Microtask queue (higher priority)
    microtasks: VecDeque<Task>,
    /// Macrotask queue (lower priority)
    macrotasks: VecDeque<Task>,
    /// Timer queue
    timers: Vec<TimerTask>,
    /// Next timer ID
    next_timer_id: u32,
    /// Running flag
    running: bool,
}

impl EventLoop {
    pub fn new() -> Self {
        Self {
            microtasks: VecDeque::new(),
            macrotasks: VecDeque::new(),
            timers: Vec::new(),
            next_timer_id: 1,
            running: false,
        }
    }

    /// Queue a microtask
    pub fn queue_microtask(&mut self, task: impl FnOnce() + Send + 'static) {
        self.microtasks.push_back(Task::Microtask(Box::new(task)));
    }

    /// Queue a macrotask
    pub fn queue_macrotask(&mut self, task: impl FnOnce() + Send + 'static) {
        self.macrotasks.push_back(Task::Macrotask(Box::new(task)));
    }

    /// Schedule a timer (setTimeout)
    pub fn set_timeout(&mut self, callback: usize, delay: Duration) -> u32 {
        let id = self.next_timer_id;
        self.next_timer_id += 1;

        self.timers.push(TimerTask {
            callback,
            execute_at: Instant::now() + delay,
            interval: None,
            id,
        });

        id
    }

    /// Schedule an interval (setInterval)
    pub fn set_interval(&mut self, callback: usize, interval: Duration) -> u32 {
        let id = self.next_timer_id;
        self.next_timer_id += 1;

        self.timers.push(TimerTask {
            callback,
            execute_at: Instant::now() + interval,
            interval: Some(interval),
            id,
        });

        id
    }

    /// Clear a timer
    pub fn clear_timer(&mut self, id: u32) {
        self.timers.retain(|t| t.id != id);
    }

    /// Run the event loop
    pub fn run(&mut self) {
        self.running = true;

        while self.running && self.has_pending_work() {
            // Process all microtasks first
            while let Some(task) = self.microtasks.pop_front() {
                match task {
                    Task::Microtask(f) => f(),
                    _ => {}
                }
            }

            // Check timers
            let now = Instant::now();
            let mut i = 0;
            while i < self.timers.len() {
                if self.timers[i].execute_at <= now {
                    let timer = self.timers.remove(i);
                    // Execute timer callback
                    // TODO: Call the actual function
                    
                    // Re-schedule if interval
                    if let Some(interval) = timer.interval {
                        self.timers.push(TimerTask {
                            callback: timer.callback,
                            execute_at: now + interval,
                            interval: Some(interval),
                            id: timer.id,
                        });
                    }
                } else {
                    i += 1;
                }
            }

            // Process one macrotask
            if let Some(task) = self.macrotasks.pop_front() {
                match task {
                    Task::Macrotask(f) => f(),
                    _ => {}
                }
            }

            // Yield to OS if no work
            if !self.has_pending_work() {
                std::thread::sleep(Duration::from_millis(1));
            }
        }
    }

    /// Stop the event loop
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if there's pending work
    fn has_pending_work(&self) -> bool {
        !self.microtasks.is_empty() || !self.macrotasks.is_empty() || !self.timers.is_empty()
    }
}

/// Promise methods
pub struct PromiseAPI;

impl PromiseAPI {
    /// Promise.resolve(value)
    pub fn resolve(value: Value) -> Promise {
        let mut promise = Promise::new();
        promise.resolve(value);
        promise
    }

    /// Promise.reject(reason)
    pub fn reject(reason: Value) -> Promise {
        let mut promise = Promise::new();
        promise.reject(reason);
        promise
    }

    /// Promise.all(promises)
    pub fn all(_promises: Vec<Promise>) -> Promise {
        // TODO: Implement Promise.all
        Promise::new()
    }

    /// Promise.race(promises)
    pub fn race(_promises: Vec<Promise>) -> Promise {
        // TODO: Implement Promise.race
        Promise::new()
    }

    /// Promise.allSettled(promises)
    pub fn all_settled(_promises: Vec<Promise>) -> Promise {
        // TODO: Implement Promise.allSettled
        Promise::new()
    }

    /// Promise.any(promises)
    pub fn any(_promises: Vec<Promise>) -> Promise {
        // TODO: Implement Promise.any
        Promise::new()
    }
}

/// Timer API
pub struct TimerAPI;

impl TimerAPI {
    /// setTimeout(callback, delay)
    pub fn set_timeout(event_loop: &mut EventLoop, callback: usize, delay: u64) -> u32 {
        event_loop.set_timeout(callback, Duration::from_millis(delay))
    }

    /// setInterval(callback, interval)
    pub fn set_interval(event_loop: &mut EventLoop, callback: usize, interval: u64) -> u32 {
        event_loop.set_interval(callback, Duration::from_millis(interval))
    }

    /// clearTimeout(id)
    pub fn clear_timeout(event_loop: &mut EventLoop, id: u32) {
        event_loop.clear_timer(id);
    }

    /// clearInterval(id)
    pub fn clear_interval(event_loop: &mut EventLoop, id: u32) {
        event_loop.clear_timer(id);
    }

    /// setImmediate(callback)
    pub fn set_immediate(event_loop: &mut EventLoop, callback: impl FnOnce() + Send + 'static) {
        event_loop.queue_macrotask(callback);
    }
}
