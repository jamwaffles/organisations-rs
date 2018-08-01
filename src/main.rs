//! Organisations domain implemented in Rust

#![deny(missing_docs)]

extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate jsonwebtoken;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate uuid;

mod aggregators;
mod context;
mod enforcement;
mod events;
mod eventstore;
mod middleware;
mod operations;
mod responses;

use actix_web::actix::{Addr, SyncArbiter, System};
use actix_web::http::Method;
use actix_web::{server, App};
// use eventstore::PgEventStore;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

use eventstore::EventStoreExecutor;
use operations::{get_organisation_members, health};

/// State given to requests
pub struct AppState {
    /// Database connection
    pub eventstore: Addr<EventStoreExecutor>,
}

fn main() {
    let sys = System::new("organisations-rs");

    let manager = PostgresConnectionManager::new(
        "postgres://postgres:postgres@localhost:5431/organisations-rs",
        TlsMode::None,
    ).expect("Could not connect to DB");

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || EventStoreExecutor(pool.clone()));

    server::new(move || {
        App::with_state(AppState {
            eventstore: addr.clone(),
        }).middleware(middleware::InjectJwt)
        .resource("/health", |r| r.get().f(health))
        .resource("/get-organisation-members/{organisation_id}", |r| {
            r.middleware(enforcement::OrganisationMember);
            r.get().with(get_organisation_members);
        })
    }).bind("0.0.0.0:8080")
    .unwrap()
    .start();

    sys.run();
}
