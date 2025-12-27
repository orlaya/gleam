#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::mem_forget,
    // TODO: enable once the false positive bug is solved
    // clippy::use_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    rust_2018_idioms,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    nonstandard_style,
    unexpected_cfgs,
    unused_import_braces,
    unused_qualifications,
    clippy::wildcard_enum_match_arm
)]
#![deny(
    clippy::await_holding_lock,
    clippy::disallowed_methods,
    clippy::if_let_mutex,
    clippy::indexing_slicing,
    clippy::mem_forget,
    clippy::ok_expect,
    clippy::unimplemented,
    clippy::unwrap_used,
    unsafe_code,
    unstable_features,
    unused_results
)]
#![allow(
    clippy::assign_op_pattern,
    clippy::to_string_trait_impl,
    clippy::match_single_binding,
    clippy::match_like_matches_macro,
    clippy::inconsistent_struct_constructor,
    clippy::len_without_is_empty,
    // TODO: fix
    clippy::arc_with_non_send_sync,
)]

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

//
//
//
mod __core;
pub use __core::ast;
// pub(crate) use __core::ast::ast_folder; // unused currently

pub use __core::errors::diagnostic;
pub use __core::errors::error;
pub use __core::errors::fix;
pub use __core::errors::warning;
pub use error::{Error, Result};
pub use warning::Warning;

pub use __core::type_;
pub(crate) use __core::type_::bit_array_options;

pub(crate) use __core::graph;
pub use __core::line_numbers;
pub use __core::strings;
pub use __core::uid;
pub use __core::version;

//
//
//
mod __parse;
pub use __parse::parse;

//
//
//
mod __analyse;
pub use __analyse::analyse;
pub(crate) use __analyse::call_graph;
pub(crate) use __analyse::exhaustiveness;
pub(crate) use __analyse::inline;
pub(crate) use __analyse::reference;

//
//
//
mod __generate;
pub use __generate::codegen;
pub use __generate::docs;
pub use __generate::erlang;
pub use __generate::javascript;

//
//
//
mod __format;
pub use __format::format;
pub use __format::pretty;

//
//
//
pub mod build;
pub use build::compiler;
pub use build::config;
pub use build::io;
pub use build::metadata;

pub use build::package_interface;
pub use build::paths;

pub use build::packages::encryption;
pub use build::packages::hex;
pub use build::packages::manifest;
pub use build::packages::requirement;

pub use build::dependencies::dep_tree;
pub use build::dependencies::dependency;
pub(crate) use build::dependencies::derivation_tree;

//
//
//
pub mod language_server;

//
//
//
const GLEAM_CORE_PACKAGE_NAME: &str = "";
const STDLIB_PACKAGE_NAME: &str = "gleam_stdlib";

//
//
//
// TWEAKED - easier to refactor
macro_rules! template_str {
    ($path:literal) => {
        std::include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/__templates/", $path))
    };
}
pub(crate) use template_str;

macro_rules! template_bytes {
    ($path:literal) => {
        std::include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/__templates/", $path))
    };
}
pub(crate) use template_bytes;

macro_rules! generated {
    ($path:literal) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/__generated/", $path));
    };
}

mod schema_capnp {
    #![allow(
        dead_code,
        unused_qualifications,
        clippy::all,
        clippy::unwrap_used,
        missing_debug_implementations,
        missing_copy_implementations
    )]
    generated!("schema_capnp.rs");
}
