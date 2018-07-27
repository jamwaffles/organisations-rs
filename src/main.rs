//! Organisations domain implemented in Rust

#![deny(missing_docs)]

extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate postgres;
extern crate uuid;

mod events;
mod eventstore;
mod operations;
mod responses;

use actix_web::http::Method;
use actix_web::{server, App};
use eventstore::PgEventStore;
use postgres::{Connection, TlsMode};

use eventstore::{EventStore, GetInvitesQuery};

use operations::health;

fn main() {
    let conn = Connection::connect(
        "postgres://postgres:postgres@localhost:5431/organisations-rs",
        TlsMode::None,
    ).unwrap();

    let store = PgEventStore::new(conn).expect("Could not create store");

    let events = store.read_all(GetInvitesQuery {
        organisation_id: "b1272b5a-05a9-4658-a3e2-3f91ef765b96".into(),
    });

    println!("Events: {:?}", events);

    server::new(|| App::new().resource("/health", |r| r.method(Method::GET).f(health)))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run();
}
