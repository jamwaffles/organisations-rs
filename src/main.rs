//! Organisations domain implemented in Rust

#![deny(missing_docs)]

extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod operations;
mod responses;

use actix_web::http::Method;
use actix_web::{server, App};

use operations::health;

fn main() {
    server::new(|| App::new().resource("/health", |r| r.method(Method::GET).f(health)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
