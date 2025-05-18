//! Self-contained modules used for miscellaneous purposes.
//!
//! These are all candidates for being separated into different crates,
//! tell me if the functionality they provide is useful enough to do that.

pub mod ruler;
pub mod sourcemap;
pub mod utils;

mod typekey;
pub use typekey::TypeKey;
