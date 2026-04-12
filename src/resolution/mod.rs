//! Resolution protocols for encounter resolution.

pub mod multi_beat;
pub mod single;

pub use multi_beat::MultiBeat;
pub use single::SingleExchange;
