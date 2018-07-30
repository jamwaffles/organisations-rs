//! Db executor actor
use actix_web::actix::{Actor, Handler, Message, SyncContext};
use actix_web::*;
use context::Context;
use events::Event;
use postgres::{types::ToSql, Connection as PostgresConnection};
use r2d2_postgres::r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use serde_json::{from_value, to_value, Value as JsonValue};
use std::collections::HashMap;
use uuid::Uuid;

pub struct EventStoreExecutor(pub Pool<PostgresConnectionManager>);

pub struct GetOrganisationMembersQuery {
    pub organisation_id: Uuid,
}

impl From<GetOrganisationMembersQuery> for EventsQuery {
    fn from(other: GetOrganisationMembersQuery) -> EventsQuery {
        let mut search: HashMap<String, String> = HashMap::new();

        search.insert("organisation_id".into(), other.organisation_id.to_string());

        EventsQuery { search }
    }
}

/// Get events query
pub struct EventsQuery {
    /// Which fields/values to search by
    search: HashMap<String, String>,
}

impl EventsQuery {
    pub fn as_query_string(&self) -> String {
        let filters = self
            .search
            .keys()
            .enumerate()
            .map(|(i, key)| format!("events.data->>'{}' = ${}", key, i + 1))
            .collect::<Vec<String>>()
            .join(" AND ");

        format!("SELECT * FROM events WHERE {} ORDER BY time ASC", filters)
    }
}

impl Message for EventsQuery {
    type Result = Result<Vec<Event>, Error>;
}

impl Actor for EventStoreExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<EventsQuery> for EventStoreExecutor {
    type Result = Result<Vec<Event>, Error>;

    fn handle(&mut self, query: EventsQuery, _: &mut Self::Context) -> Self::Result {
        let conn: &PostgresConnection = &self.0.get().unwrap();

        let values = query.search.values().collect::<Vec<&String>>();

        let query_string = query.as_query_string();

        let mut params: Vec<&ToSql> = Vec::new();

        // TODO: Clean
        for (i, _) in query.search.values().enumerate() {
            params.push(values[i]);
        }

        println!("Query: {:?} args: {:?}", query_string, params);

        let stmt = conn.prepare(&query_string).expect("Prep");

        let results = stmt.query(params.as_slice()).expect("Query");

        Ok(results
            .iter()
            .map(|row| {
                let json: JsonValue = row.get("data");

                let evt: Event = from_value(json).expect("Row convert");

                evt
            }).collect())
    }
}

#[derive(Debug)]
pub struct SaveEvent {
    event: Event,
    context: Option<Context>,
}

impl SaveEvent {
    pub fn new(event: Event, context: Option<Context>) -> Self {
        Self { event, context }
    }
}

impl Message for SaveEvent {
    type Result = Result<(), Error>;
}

impl Handler<SaveEvent> for EventStoreExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, payload: SaveEvent, _: &mut Self::Context) -> Self::Result {
        let conn: &PostgresConnection = &self.0.get().unwrap();

        println!("DB insert {:?}", payload);

        conn.execute(
            "INSERT INTO events(data, context) \
             VALUES($1, $2) \
             ON CONFLICT (id) DO NOTHING \
             RETURNING *",
            &[
                &to_value(payload.event).expect("Event2JSON"),
                &to_value(payload.context).expect("Context2JSON"),
            ],
        ).expect("Insert failed");

        Ok(())
    }
}
