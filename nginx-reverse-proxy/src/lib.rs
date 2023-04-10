//! # NPM Expansions Static
//!
//! A library allowing for static files and directories to have environment variables injected and minified

#![feature(rustdoc_missing_doc_code_examples)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

/// A series of utility functions that allow files and directories to be minifed
pub mod minify;

/// A series of utility functions that allow a hashmap of environment variables to be injected into
/// files
pub mod env_injector;
