# Organisations (but in Rust)

[![Build Status](https://travis-ci.org/jamwaffles/organisations-rs.svg?branch=master)](https://travis-ci.org/jamwaffles/organisations-rs)

Setup the database:
```bash
db.sh
```
Note that database data directory is persisted in `./postgres_data`.

Run the server:

```bash
cd ./server
cargo run

# In another console (or use api.rest)
curl http://localhost:8080/health

# >>> Hello bar! id:1
```
