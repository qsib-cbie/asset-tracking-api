# QSIB Assets

This Rust/Diesel application provides the API for managing generic and official assets for the group. This includes standard asset information such as name and serial number as well as locations visited, user comments, user manual documentation.

## Development

* Install Rust with rustup.rs
* Install Postgres
* Install Diesel: `cargo install diesel_cli --no-default-features --features postgres`
    * On Windows, you need to add the location of the Postgres `lib` and `bin` files as a PATH environmental variable on the system. Reference link to the solution: https://stackoverflow.com/questions/20412084/postgresql-error-the-program-cant-start-because-libpq-dll-is-missing-from-your
    * On Mac, `brew install libpq`
    * On Ubuntu, `sudo apt install libpq-dev`
* Setup Postgres db with your `.env` credentials
    * The `DATABASE_URL` var has to be configured in the .env file before the next step
* Setup the db in Postgres with diesel `diesel setup` after navigating to the root directory


## Testing

Models reflect schema in the postgres database. The Diesel framework exposes the database interactions in the `model.rs` for each table schema in the database schema. `tests` modules belong in each of the `model.rs`. To test basic database interactions. The database connections is acquired by reference through a pool of connections. During test execution, the pooled connection manager holds a single connection that starts a test transaction that is never committed. This test connection is made against a local postgres instance; however, during standard runs, the DATABASE_URL environment variable is used to open a pooled connection manager with 10 connections without injecting transactions.

Actix provides the routing and error handling for actions that run through the HttpServer services. The `routes.rs` for each table schema defines what routes will be accessible via the HTTP endpoint and how they will communicate with the `model.rs`. Integration testing of the Actix routes and Diesel models exists in `main.rs` where a series of requests can be verified against an App that is running internally (not exposed via HTTP server).

### Automated testing

`cargo test` should pass, but if it fails use the env logger to get more information. For example, if you are working on new tests or debugging something you might choose various log levels for different crates:
* `RUST_LOG=trace cargo test`: All of the logs you can get
* `RUST_LOG=asset_api=trace,rest_api=debug`: All of the logs we create and some of the logs from the another crate
* Click individual tests in vs code to run or debug individual tests

### Manual testing

Bring up an endpoint:
* `cargo run`: Starts an http server
* Your `.env` file needs to define a couple things. The RUST_LOG config is optional depending on what you are looking to test/debug
    * `AUTH_SECRET` must be defined. Generate a random string at least **48** bytes long for local development.
    * Do **not** commit sensitive environment variables or tokens.
```
RUST_LOG=qsib_asset=trace,info
DATABASE_URL=postgres://[username]:[password]@localhost:5432/asset_api
HOST=0.0.0.0
PORT=6001
AUTH_SECRET=*************** 
```

You can hit the endpoint however you want; it is an HTTP server. I use httpie like so
* `http :6001/health`: No auth required 200 OK
* `http :6001/asset_tags`: Auth required 401 Unauthorized
* `http :6001/asset_tags 'Auhtorization: Bearer A841BE66-84AC-4BA7-B0E1-D34B1FC2F08A'`: Uses the test auth token for success if cfg(test) guard is disabled in `main.rs`'s validator
* `http :6001/asset_tags/foo 'Auhtorization: Bearer A841BE66-84AC-4BA7-B0E1-D34B1FC2F08A' > asset_tag.json`: httpie writes file with stdout with the json, trim to create valid json template to upload later
* `cat asset_tag.json | http put :6001/asset_tags 'Auhtorization: Bearer A841BE66-84AC-4BA7-B0E1-D34B1FC2F08A'`: httpie reads file from stdin, adds a content type application json header and uses the file contents as the body.

Connect to the local postgres database and make changes behind the scenes

* `psql postgres://localhost/asset_api` or `psql -d asset_api`: To connect to a local postgres database named asset_api
* `select * from asset_tags order by created_at desc;`: Example query if connected to database to lookup data
* `update asset_tags set description = 'adsf' where id = 1;`: Example query to change the description of the record with id 1
* `\q` or `ctrl+d`: quit postgres
* `\d`, `\dS`, `\dS asset_tags`, `\s`, `\l`: there are a million useful postgres commands


## Anticipated Workflows

### Bulk Asset Add

At the end of semesters, prior to audits, or after a bulk order, we expect it to be common for many items to be added to inventory tracking.

### Location Lookup

Assets will move over time and periodically add location updates to the database. These updates will be queried by location as well as by asset to help narrow down where an asset or group of assets associated with a collaborator is located.

### Event Driven Interaction

Users may add comments on an asset state, which should trigger interaction with a real person. Initiating an email chain would be a sensible start for discussion asset problems.

Similarly, if warning conditions trigger due to events like low battery notifications or extended silence. The users most recently involved with the asset should receive a notification along with the asset supervisor.

## Authentication

Bearer Token authentication with a base64 encoded symmetric hash string of secret, username, and bcrypted password hash as the token.

- [x] Create user with username and password
- [x] Store bcrypted hash string of password as internal token
- [x] Expose symmetrically encrypted hash string as token for user that expires on secret or password change
- [x] Lookup user from a symmetrically decrypted token and verify bcrypted hash password matches
- [x] Access protected routes after verifying token
- [x] Allow user to change own password
- [ ] TBD: Roles and Policies

Specific TODOs:

- [x] Limit user to update only self during req/token verification
- [x] Configure seed data for the database and remove alternative validator paths
- [ ] Wrap crypto errors in enum and produce proper error status and message
- [ ] Replace String usages with &str, array, and slices where possible in token manipulation


### Schema

* AssetTag
    * Name
    * Description
    * Serial Number
    * Supervisor (Role)
* AssetScanner
    * Name
    * Room
* User
    * Name
    * Roles
* Role
    * Name
* Comment
    * Content
    * User
    * AssetTag
* Room
    * Name
    * Location
* Location
    * Name
    * Latitude
    * Longitude
    * IP Address
* ContactEvent
    * Asset
    * Location
    * Alert
* Alert
    * User
    * Message


### Relationships

* AssetTag
    * has many Comments
    * has many ContactEvents
    * belongs to a Role
* AssetScanner
    * has many ContactEvents
* ContactEvents
    * belongs to a Room
* Room
    * belongs to a Location
* User
    * has many Comments
    * has many Roles
* Alert
    * belongs to a Role
    * belongs to a Comment
