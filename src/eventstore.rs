use context::Context;
use events::Event;
use postgres::types::ToSql;
use postgres::Connection;
use serde_json::{from_value, to_value, Value as JsonValue};

pub trait ReadQuery {
    fn to_query_string(&self) -> (&str, Vec<&str>);
}

pub trait EventStore {
    fn read_all<T>(&self, query: T) -> Vec<Event>
    where
        T: ReadQuery;
    fn save(&self, data: &Event, context: &Option<Context>);
}

pub struct PgEventStore {
    connection: Connection,
}

impl PgEventStore {
    pub fn new(connection: Connection) -> Result<Self, String> {
        Ok(Self { connection })
    }
}

pub struct GetInvitesQuery {
    pub organisation_id: String,
}

impl ReadQuery for GetInvitesQuery {
    fn to_query_string(&self) -> (&str, Vec<&str>) {
        (
            "SELECT * FROM events WHERE events.data->>'organisation_id' = $1",
            vec![&self.organisation_id],
        )
    }
}

impl EventStore for PgEventStore {
    fn read_all<T>(&self, query: T) -> Vec<Event>
    where
        T: ReadQuery,
    {
        let (query, params) = query.to_query_string();

        let mut p: Vec<&ToSql> = Vec::new();

        // TODO: Clean
        for (i, _) in params.iter().enumerate() {
            p.push(&params[i]);
        }

        println!("Query: {:?} args: {:?}", query, p);

        let stmt = self.connection.prepare(query).expect("Prep");

        let results = stmt.query(&p).expect("Query");

        results
            .iter()
            .map(|row| {
                let json: JsonValue = row.get("data");

                from_value(json).expect("Row convert")
            }).collect()
    }

    fn save(&self, event: &Event, context: &Option<Context>) {
        self.connection
            .execute(
                "INSERT INTO events(data, context) \
                 VALUES($1, $2) \
                 ON CONFLICT (id) DO NOTHING \
                 RETURNING *",
                &[
                    &to_value(event).expect("Event2JSON"),
                    &to_value(context).expect("Context2JSON"),
                ],
            ).expect("Insert Event");
    }
}
