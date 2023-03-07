// Clippy
#![warn(clippy::all, clippy::cargo, clippy::pedantic)]
#![allow(clippy::cargo_common_metadata)]
// Documentation
#![allow(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::undocumented_unsafe_blocks
)]
// Restrictions
#![warn(
    clippy::as_conversions,
    clippy::dbg_macro,
    clippy::empty_structs_with_brackets,
    clippy::from_over_into,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::let_underscore_must_use,
    clippy::map_err_ignore,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_slice,
    clippy::string_to_string,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_used
)]

fn main() {}
