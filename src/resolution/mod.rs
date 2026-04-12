//! Resolution protocols for encounter resolution.

pub mod background;
pub mod multi_beat;
pub mod single;

pub use background::{SchemePhase, SchemeState};
pub use multi_beat::MultiBeat;
pub use single::SingleExchange;
