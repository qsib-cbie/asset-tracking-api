use crate::db;
use crate::error_handler::CustomError;
use crate::schema::rooms;
use crate::locations::Location;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Insertable, Associations,
)]
#[belongs_to(Location)]
#[table_name = "rooms"]
pub struct Room {
    pub id: i64,
    pub name: String,
    pub location_id: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "rooms"]
pub struct MaybeRoom {
    pub name: String,
    pub location_id: i64,
}

impl Room {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let rooms = rooms::table.load::<Room>(&conn)?;
        Ok(rooms)
    }

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let room = rooms::table.filter(rooms::id.eq(id)).first(&conn)?;
        Ok(room)
    }

    pub fn find_by_name(name: String) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let room = rooms::table.filter(rooms::name.eq(name)).first(&conn)?;
        Ok(room)
    }

    pub fn find_by_location(id: i64) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let rooms = rooms::table
            .filter(rooms::location_id.eq(id))
            .load::<Room>(&conn)?;
        Ok(rooms)
    }

    pub fn create(room: MaybeRoom) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let room = diesel::insert_into(rooms::table)
            .values(room)
            .get_result(&conn)?;
        Ok(room)
    }

    pub fn update(id: i64, room: MaybeRoom) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let room = diesel::update(rooms::table)
            .filter(rooms::id.eq(id))
            .set(room)
            .get_result(&conn)?;
        Ok(room)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(rooms::table.filter(rooms::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
