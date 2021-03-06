#![deny(unused_extern_crates)]
#![deny(missing_docs)]
#![deny(warnings)]

//! # Saphir
//!
//! Saphir is a progressive http server framework based on Hyper-rs that aims to reduce the time spent on playing with futures and
//! limiting the amount of copied code amongst request matching.
//!
//! Saphir provide what's needed to easily start with your own server with middleware, controllers and request routing.
//!
//! Futures version will comes with more macro and a nightly experiment is currently being tested to reproduces decorator in rust.

#[macro_use]
extern crate log;
extern crate futures;
extern crate ansi_term;
extern crate http as http_types;
extern crate hyperx;
extern crate rayon;
extern crate tokio;
#[cfg(feature = "https")]
extern crate rustls;
#[cfg(feature = "https")]
extern crate tokio_rustls;
extern crate parking_lot;
pub extern crate regex;
pub extern crate hyper;

#[macro_use]
mod utils;
mod http;
/// Modules for the error handling into saphir
pub mod error;
/// Modules for the middlewares
pub mod middleware;
/// Modules for the controllers
pub mod controller;
/// Modules for the router
pub mod router;
/// Modules for the http server
pub mod server;

pub use utils::*;
pub use http::*;
pub use utils::RequestContinuation;
pub use middleware::Middleware;
pub use middleware::MiddlewareStack;
pub use controller::Controller;
pub use controller::BasicController;
pub use controller::ControllerDispatch;
pub use controller::RequestGuard;
pub use controller::RequestGuardCollection;
pub use controller::BodyGuard;
pub use router::Router;
pub use server::{Server, ServerSpawn};
pub use error::ServerError;