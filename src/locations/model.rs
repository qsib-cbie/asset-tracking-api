use crate::db;
use crate::error_handler::CustomError;
use crate::schema::locations;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use ipnetwork::IpNetwork;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Insertable)]
#[table_name = "locations"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Location {
    pub id: i64,
    pub name: Option<String>,
    pub latitude: f32,
    pub longitude: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub ip: Option<IpNetwork>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "locations"]
#[changeset_options(treat_none_as_null = "true")]
pub struct MaybeLocation {
    pub name: Option<String>,
    pub latitude: f32,
    pub longitude: f32,
    pub ip: Option<IpNetwork>,
}

impl Location {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let locations = locations::table.load::<Location>(&conn)?;
        Ok(locations)
    }

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let location = locations::table.filter(locations::id.eq(id)).first(&conn)?;
        Ok(location)
    }

    pub fn find_by_name(name: String) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let location = locations::table
            .filter(locations::name.eq(name))
            .first(&conn)?;
        Ok(location)
    }

    pub fn find_by_ip(ip: IpNetwork) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let location = locations::table.filter(locations::ip.eq(ip)).first(&conn)?;
        Ok(location)
    }

    pub fn create(location: MaybeLocation) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let location = diesel::insert_into(locations::table)
            .values(location)
            .get_result(&conn)?;
        Ok(location)
    }

    pub fn update(id: i64, location: MaybeLocation) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let location = diesel::update(locations::table)
            .filter(locations::id.eq(id))
            .set(location)
            .get_result(&conn)?;
        Ok(location)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(locations::table.filter(locations::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
