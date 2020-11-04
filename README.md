# QSIB Assets

This Rust/Diesel application provides the API for managing generic and official assets for the group. This includes standard asset information such as name and serial number as well as locations visited, user comments, user manual documentation.

## Development

* Install Rust with rustup.rs
* Install Postgres
* Install Diesel: `cargo install diesel_cli --no-default-features --features postgres`
* Setup Postgres db with your .env credentials
* Setup the db in Postgres with diesel `diesel run setup`

## Testing

Models reflect schema in the postgres database. The Diesel framework exposes the database interactions in the `model.rs` for each table schema in the database schema. `tests` modules belong in each of the `model.rs`. To test basic database interactions. The database connections is acquired by reference through a pool of connections. During test execution, the pooled connection manager holds a single connection that starts a test transaction that is never committed. This test connection is made against a local postgres instance; however, during standard runs, the DATABASE_URL environment variable is used to open a pooled connection manager with 10 connections without injecting transactions.

Actix provides the routing and error handling for actions that run through the HttpServer services. The `routes.rs` for each table schema defines what routes will be accessible via the HTTP endpoint and how they will communicate with the `model.rs`. Integration testing of the Actix routes and Diesel models exists in `main.rs` where a series of requests can be verified against an App that is running internally (not exposed via HTTP server).

## Anticipated Workflows

### Bulk Asset Add

At the end of semesters, prior to audits, or after a bulk order, we expect it to be common for many items to be added to inventory tracking.

### Location Lookup

Assets will move over time and periodically add location updates to the database. These updates will be queried by location as well as by asset to help narrow down where an asset or group of assets associated with a collaborator is located.

### Event Driven Interaction

Users may add comments on an asset state, which should trigger interaction with a real person. Initiating an email chain would be a sensible start for discussion asset problems.

Similarly, if warning conditions trigger due to events like low battery notifications or extended silence. The users most recently involved with the asset should receive a notification along with the asset supervisor.

## Authentication

JWT Bearer Token Authentication. Maybe through Auth0 or hand-rolled

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
