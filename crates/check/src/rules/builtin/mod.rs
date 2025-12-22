//! Built-in lint rules
//!
//! Core rules for JavaScript/TypeScript linting.

mod no_console;
mod no_debugger;
mod no_unused_vars;
mod eqeqeq;
mod prefer_const;
mod no_var;
mod no_eval;
mod no_with;

pub use no_console::NoConsole;
pub use no_debugger::NoDebugger;
pub use no_unused_vars::NoUnusedVars;
pub use eqeqeq::Eqeqeq;
pub use prefer_const::PreferConst;
pub use no_var::NoVar;
pub use no_eval::NoEval;
pub use no_with::NoWith;

use super::Rule;

/// Get all built-in rules
pub fn all_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(NoConsole::default()),
        Box::new(NoDebugger::default()),
        Box::new(NoUnusedVars::default()),
        Box::new(Eqeqeq::default()),
        Box::new(PreferConst::default()),
        Box::new(NoVar::default()),
        Box::new(NoEval::default()),
        Box::new(NoWith::default()),
    ]
}

/// Get recommended rules only
pub fn recommended_rules() -> Vec<Box<dyn Rule>> {
    all_rules()
        .into_iter()
        .filter(|r| r.meta().recommended)
        .collect()
}
