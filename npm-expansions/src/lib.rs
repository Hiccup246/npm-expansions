//! # Npm Expansions
//!
//! A library creating a npm expansions web server based on HTTP/1.1

#![feature(rustdoc_missing_doc_code_examples)]
#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

/// A series of default HTTP response methods
pub mod default_controller;

/// A static database of npm expansions and methods to search them
pub mod expansions_model;

/// A series of helper functions and modules for processing mime types
pub mod mime_type;

/// A mock of the expansions model for testing purposes
pub mod mock_expansions_model;

/// A mock of the expansions model for testing purposes
pub mod mock_tcp_stream;

/// A series of JSON HTTP response methods for searching the expansions model
pub mod npm_controller;

/// A generic error used to represent server failings
pub mod npm_expansion_error;

/// A representation of the different parts of a HTTP request
pub mod request;

/// A representation of a HTTP response
pub mod response;

/// A series of utilities to handle incoming HTTP requests and to response to them
pub mod router;

/// A module for taking a TCP stream and correctly handling its HTTP request
pub mod stream_handler;
