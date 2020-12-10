use crate::alerts::Alert;
use crate::asset_tags::AssetTag;
use crate::locations::Location;
use crate::db;
use crate::error_handler::CustomError;
use crate::schema::contact_events;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Insertable, Associations,
)]
#[belongs_to(Alert)]
#[belongs_to(AssetTag)]
#[belongs_to(Location)]
#[table_name = "contact_events"]
pub struct ContactEvent {
    pub id: i64,    
    pub asset_tag_id: i64,
    pub location_id: i64,
    pub alert_id: Option<i64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "contact_events"]
pub struct MaybeContactEvent {
    pub asset_tag_id: i64,
    pub location_id: i64,
    pub alert_id: Option<i64>
}

impl ContactEvent {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let contact_events = contact_events::table.load::<ContactEvent>(&conn)?;
        Ok(contact_events)
    }

    pub fn find_by_id(id: i64) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let contact_event = contact_events::table.filter(contact_events::id.eq(id)).first(&conn)?;
        Ok(contact_event)
    }    

    pub fn find_by_asset_tag(id: i64) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let contact_events = contact_events::table
            .filter(contact_events::asset_tag_id.eq(id))
            .load::<ContactEvent>(&conn)?;
        Ok(contact_events)
    }

    pub fn find_by_location(id: i64) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let contact_events = contact_events::table
            .filter(contact_events::location_id.eq(id))
            .load::<ContactEvent>(&conn)?;
        Ok(contact_events)
    }

    pub fn find_by_alert(id: i64) -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let contact_events = contact_events::table
            .filter(contact_events::alert_id.eq(id))
            .load::<ContactEvent>(&conn)?;
        Ok(contact_events)
    }

    pub fn create(contact_event: MaybeContactEvent) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let contact_event = diesel::insert_into(contact_events::table)
            .values(contact_event)
            .get_result(&conn)?;
        Ok(contact_event)
    }

    pub fn update(id: i64, contact_event: MaybeContactEvent) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let contact_event = diesel::update(contact_events::table)
            .filter(contact_events::id.eq(id))
            .set(contact_event)
            .get_result(&conn)?;
        Ok(contact_event)
    }

    pub fn delete(id: i64) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(contact_events::table.filter(contact_events::id.eq(id))).execute(&conn)?;
        Ok(res)
    }
}
