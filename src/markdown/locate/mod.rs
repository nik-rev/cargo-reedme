//! Locate the span of markdown items
//!
//! This module exists only because the markdown parser
//! doesn't give us enough information, like it doesn't tell us
//! anything about which reference links exist - we must extract that
//! information on our own

mod inline_link_destination;
mod reference_link_definition;

pub use inline_link_destination::inline_link_destination;
pub use reference_link_definition::reference_link_definition;
